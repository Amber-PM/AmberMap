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
use std::io::Read;

const ZSTD_MAGIC: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

pub fn decompress_payload(raw: &[u8]) -> Result<Vec<u8>> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }

    if raw.len() >= 4 && raw[0..4] == ZSTD_MAGIC {
        let mut decoder = zstd::Decoder::new(raw)
            .map_err(|e| AmberError::DecompressionFailed(format!("zstd init: {e}")))?;
        let mut out = Vec::new();
        decoder
            .read_to_end(&mut out)
            .map_err(|e| AmberError::DecompressionFailed(format!("zstd read: {e}")))?;
        return Ok(out);
    }

    let mut snap_decoder = snap::raw::Decoder::new();
    if let Ok(decompressed) = snap_decoder.decompress_vec(raw) {
        if !decompressed.is_empty() {
            return Ok(decompressed);
        }
    }

    Ok(raw.to_vec())
}
