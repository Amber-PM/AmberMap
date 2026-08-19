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

use ambermap::leveldb::decompressor::decompress_payload;

#[test]
fn test_zstd_decompression() {
    let original = b"Bedrock World SubChunk Block Data Payload 1234567890";
    let compressed = zstd::encode_all(&original[..], 3).expect("zstd compression failed");

    let result = decompress_payload(&compressed).expect("decompression failed");
    assert_eq!(result, original);
}

#[test]
fn test_snappy_decompression() {
    let original = b"Legacy Bedrock Snappy Chunk Data String For Testing";
    let mut encoder = snap::raw::Encoder::new();
    let compressed = encoder
        .compress_vec(original)
        .expect("snappy compression failed");

    let result = decompress_payload(&compressed).expect("decompression failed");
    assert_eq!(result, original);
}

#[test]
fn test_raw_payload_fallback() {
    let original = b"Raw Uncompressed Metadata Payload";
    let result = decompress_payload(original).expect("raw fallback failed");
    assert_eq!(result, original);
}
