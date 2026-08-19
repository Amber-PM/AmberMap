/*
 *     _             _                __  __             
 *    / \   _ __ ___ | |__   ___ _ __ |  \/  | __ _ _ __  
 *   / _ \ | '_ ` _ \| '_ \ / _ \ '__|| |\/| |/ _` | '_ \ 
 *  / ___ \| | | | | | |_) |  __/ |   | |  | | (_| | |_) |
 * /_/   \_\_| |_| |_|_.__/ \___|_|   |_|  |_|\__,_| .__/ 
 *                                                 |_|    
 * 
 * AmberMap - High-Performance Bedrock World Map Renderer
 * https://github.com/Amber-PM/AmberMap
 *
 * Copyright (c) 2026 Amber-PM
 * Licensed under Apache-2.0 or MIT
 */

use crate::core::error::{AmberError, Result};
use byteorder::{ByteOrder, LittleEndian};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const TABLE_MAGIC_FOOTER: [u8; 8] = [0x57, 0xfb, 0x80, 0x8b, 0x24, 0x75, 0x47, 0xdb];

pub struct BlockHandle {
    pub offset: u64,
    pub size: u64,
}

impl BlockHandle {
    pub fn read(data: &[u8], offset: &mut usize) -> Result<Self> {
        let block_offset = read_varint_u64(data, offset)?;
        let size = read_varint_u64(data, offset)?;
        Ok(Self {
            offset: block_offset,
            size,
        })
    }
}

pub fn read_varint_u64(data: &[u8], offset: &mut usize) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0;
    while *offset < data.len() {
        let b = data[*offset];
        *offset += 1;
        result |= ((b & 0x7f) as u64) << shift;
        if (b & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift >= 64 {
            return Err(AmberError::CorruptSubChunk("varint overflow".into()));
        }
    }
    Err(AmberError::BufferUnderflow {
        needed: 1,
        remaining: 0,
    })
}

pub fn decompress_block(raw: &[u8], compression_type: u8) -> Result<Vec<u8>> {
    match compression_type {
        0 => Ok(raw.to_vec()),
        1 => {
            let mut decoder = snap::raw::Decoder::new();
            decoder
                .decompress_vec(raw)
                .map_err(|e| AmberError::DecompressionFailed(format!("snappy block: {e}")))
        }
        2 => {
            let mut decoder = flate2::read::ZlibDecoder::new(raw);
            let mut out = Vec::new();
            if decoder.read_to_end(&mut out).is_ok() && !out.is_empty() {
                Ok(out)
            } else {
                let mut def_decoder = flate2::read::DeflateDecoder::new(raw);
                let mut def_out = Vec::new();
                def_decoder
                    .read_to_end(&mut def_out)
                    .map_err(|e| AmberError::DecompressionFailed(format!("zlib/deflate block: {e}")))?;
                Ok(def_out)
            }
        }
        4 => {
            let mut def_decoder = flate2::read::DeflateDecoder::new(raw);
            let mut def_out = Vec::new();
            if def_decoder.read_to_end(&mut def_out).is_ok() && !def_out.is_empty() {
                return Ok(def_out);
            }

            let mut decoder = zstd::Decoder::new(raw)
                .map_err(|e| AmberError::DecompressionFailed(format!("zstd block init: {e}")))?;
            let mut out = Vec::new();
            decoder
                .read_to_end(&mut out)
                .map_err(|e| AmberError::DecompressionFailed(format!("zstd block read: {e}")))?;
            Ok(out)
        }
        other => Err(AmberError::DecompressionFailed(format!(
            "unsupported block compression type {other}"
        ))),
    }
}

pub fn read_sstable(file_path: impl AsRef<Path>, map: &mut HashMap<Vec<u8>, Vec<u8>>) -> Result<()> {
    let p = file_path.as_ref();
    let mut file = File::open(p)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 48 {
        return Ok(());
    }

    let footer_start = data.len() - 48;
    let magic = &data[data.len() - 8..];
    if magic != TABLE_MAGIC_FOOTER {
        return Ok(());
    }

    let mut cursor = footer_start;
    let _metaindex_handle = BlockHandle::read(&data, &mut cursor)?;
    let index_handle = BlockHandle::read(&data, &mut cursor)?;

    let index_block_raw = match read_raw_block(&data, &index_handle) {
        Ok(b) => b,
        Err(_) => return Ok(()),
    };

    let index_entries = match parse_block_kvs(&index_block_raw) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for (_key, val) in index_entries {
        let mut handle_cursor = 0;
        if let Ok(data_handle) = BlockHandle::read(&val, &mut handle_cursor) {
            if let Ok(data_block_raw) = read_raw_block(&data, &data_handle) {
                if let Ok(kvs) = parse_block_kvs(&data_block_raw) {
                    for (k, v) in kvs {
                        if k.len() >= 8 {
                            let user_key = k[..k.len() - 8].to_vec();
                            map.insert(user_key, v);
                        } else {
                            map.insert(k, v);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_raw_block(data: &[u8], handle: &BlockHandle) -> Result<Vec<u8>> {
    let start = handle.offset as usize;
    let end = start + (handle.size as usize);
    let total_end = end + 5;

    if total_end > data.len() {
        return Err(AmberError::BufferUnderflow {
            needed: total_end,
            remaining: data.len().saturating_sub(start),
        });
    }

    let raw_payload = &data[start..end];
    let compression_type = data[end];
    decompress_block(raw_payload, compression_type)
}

fn parse_block_kvs(block_data: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    if block_data.len() < 4 {
        return Ok(Vec::new());
    }

    let num_restarts = LittleEndian::read_u32(&block_data[block_data.len() - 4..]) as usize;
    let restarts_len = num_restarts * 4 + 4;
    if block_data.len() < restarts_len {
        return Ok(Vec::new());
    }

    let limit = block_data.len() - restarts_len;
    let mut offset = 0;
    let mut prev_key = Vec::new();
    let mut entries = Vec::new();

    while offset < limit {
        let shared_len = read_varint_u64(block_data, &mut offset)? as usize;
        let unshared_len = read_varint_u64(block_data, &mut offset)? as usize;
        let val_len = read_varint_u64(block_data, &mut offset)? as usize;

        if offset + unshared_len + val_len > limit {
            break;
        }

        let mut key = Vec::with_capacity(shared_len + unshared_len);
        if shared_len <= prev_key.len() {
            key.extend_from_slice(&prev_key[..shared_len]);
        }
        key.extend_from_slice(&block_data[offset..offset + unshared_len]);
        offset += unshared_len;

        let val = block_data[offset..offset + val_len].to_vec();
        offset += val_len;

        prev_key = key.clone();
        entries.push((key, val));
    }

    Ok(entries)
}
