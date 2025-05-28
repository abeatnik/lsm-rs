use bytes::Bytes;
use chrono::Utc;
use crc32fast::Hasher;
use serde::{ Deserialize, Serialize };
use std::io::{ Error, ErrorKind, Read, Result, Write };

const BLOCK_SIZE: usize = 32 * 1024;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalEntry {
    pub timestamp: i64,
    pub key: Bytes,
    pub value: Bytes,
}

impl WalEntry {
    pub fn new(key: Bytes, value: Bytes) -> Self {
        Self {
            timestamp: Utc::now().timestamp_nanos(),
            key,
            value,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum RecordType {
    ZeroType = 0,
    FullType = 1,
    FirstType = 2,
    MiddleType = 3,
    LastType = 4,
}

impl RecordType {
    fn from_u8(byte: u8) -> Option<RecordType> {
        match byte {
            0 => Some(RecordType::ZeroType),
            1 => Some(RecordType::FullType),
            2 => Some(RecordType::FirstType),
            3 => Some(RecordType::MiddleType),
            4 => Some(RecordType::LastType),
            _ => None,
        }
    }
}

pub struct WalEncoder;

impl WalEncoder {
    pub fn encode(entry: &WalEntry, log_number: u32, writer: &mut impl Write) -> Result<()> {
        let payload = bincode::serialize(entry)?;
        let mut start = 0;

        while start < payload.len() {
            let remaining = payload.len() - start;
            let chunk_size = std::cmp::min(remaining, BLOCK_SIZE - 11);
            let chunk = &payload[start..start + chunk_size];

            let record_type = match (start == 0, start + chunk_size == payload.len()) {
                (true, true) => RecordType::FullType,
                (true, false) => RecordType::FirstType,
                (false, false) => RecordType::MiddleType,
                (false, true) => RecordType::LastType,
            };

            let mut crc_data = Vec::with_capacity(4 + chunk.len());
            crc_data.extend_from_slice(&log_number.to_le_bytes());
            crc_data.extend_from_slice(chunk);

            let mut hasher = Hasher::new();
            hasher.update(&crc_data);
            let crc = hasher.finalize();

            writer.write_all(&crc.to_le_bytes())?;
            writer.write_all(&(chunk.len() as u16).to_le_bytes())?;
            writer.write_all(&(record_type as u8).to_le_bytes())?;
            writer.write_all(&log_number.to_le_bytes())?;
            writer.write_all(chunk)?;

            start += chunk_size;
        }

        Ok(())
    }
}

pub struct WalDecoder;

impl WalDecoder {
    pub fn decode(reader: &mut impl Read) -> Result<(u32, WalEntry)> {
        let mut full_payload = Vec::new();
        let mut found_log_number = None;

        loop {
            let mut crc_buf = [0u8; 4];
            let mut size_buf = [0u8; 2];
            let mut type_buf = [0u8; 1];
            let mut lognum_buf = [0u8; 4];

            if reader.read_exact(&mut crc_buf).is_err() {
                if full_payload.is_empty() {
                    return Err(Error::new(ErrorKind::UnexpectedEof, "No WAL entry found"));
                } else {
                    break;
                }
            }

            reader.read_exact(&mut size_buf)?;
            reader.read_exact(&mut type_buf)?;
            reader.read_exact(&mut lognum_buf)?;

            let size = u16::from_le_bytes(size_buf) as usize;
            let log_number = u32::from_le_bytes(lognum_buf);
            if found_log_number.is_none() {
                found_log_number = Some(log_number);
            }

            let mut chunk = vec![0u8; size];
            reader.read_exact(&mut chunk)?;

            let mut crc_data = Vec::with_capacity(4 + size);
            crc_data.extend_from_slice(&lognum_buf);
            crc_data.extend_from_slice(&chunk);

            let mut hasher = Hasher::new();
            hasher.update(&crc_data);
            let actual_crc = hasher.finalize();
            let expected_crc = u32::from_le_bytes(crc_buf);

            if actual_crc != expected_crc {
                return Err(Error::new(ErrorKind::InvalidData, "CRC mismatch"));
            }

            let record_type = RecordType::from_u8(type_buf[0]).ok_or_else(||
                Error::new(ErrorKind::InvalidData, "Invalid record type")
            )?;

            full_payload.extend_from_slice(&chunk);

            match record_type {
                RecordType::FullType | RecordType::LastType => {
                    break;
                }
                RecordType::FirstType | RecordType::MiddleType => {
                    continue;
                }
                RecordType::ZeroType => {
                    return Err(Error::new(ErrorKind::InvalidData, "ZeroType is reserved"));
                }
            }
        }

        let entry: WalEntry = bincode::deserialize(&full_payload)?;
        Ok((found_log_number.unwrap(), entry))
    }
}
