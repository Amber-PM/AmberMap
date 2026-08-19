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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AmberError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid key length: expected at least {expected}, got {actual}")]
    InvalidKeyLength { expected: usize, actual: usize },

    #[error("unknown key tag: 0x{0:02x}")]
    UnknownKeyTag(u8),

    #[error("decompression failure: {0}")]
    DecompressionFailed(String),

    #[error("unsupported subchunk version: {0}")]
    UnsupportedSubChunkVersion(u8),

    #[error("corrupt subchunk payload: {0}")]
    CorruptSubChunk(String),

    #[error("nbt parse error: {0}")]
    NbtError(String),

    #[error("palette index out of bounds: index {index}, size {size}")]
    PaletteOutOfBounds { index: usize, size: usize },

    #[error("buffer underflow: needed {needed} bytes, remaining {remaining}")]
    BufferUnderflow { needed: usize, remaining: usize },
}

pub type Result<T> = std::result::Result<T, AmberError>;
