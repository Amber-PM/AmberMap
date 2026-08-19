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

use ambermap::core::coordinates::ChunkPos;
use ambermap::leveldb::keys::{DbKey, Dimension, KeyTag};

#[test]
fn test_overworld_subchunk_key() {
    let key = DbKey {
        dimension: Dimension::Overworld,
        chunk_pos: ChunkPos::new(12, -5),
        tag: KeyTag::SubChunkPrefix,
        subchunk_y: Some(-2),
    };

    let bytes = key.to_bytes();
    assert_eq!(bytes.len(), 10);

    let parsed = DbKey::parse(&bytes).expect("failed to parse overworld subchunk key");
    assert_eq!(parsed.dimension, Dimension::Overworld);
    assert_eq!(parsed.chunk_pos.x, 12);
    assert_eq!(parsed.chunk_pos.z, -5);
    assert_eq!(parsed.tag, KeyTag::SubChunkPrefix);
    assert_eq!(parsed.subchunk_y, Some(-2));
}

#[test]
fn test_overworld_data2d_key() {
    let key = DbKey {
        dimension: Dimension::Overworld,
        chunk_pos: ChunkPos::new(100, 200),
        tag: KeyTag::Data2D,
        subchunk_y: None,
    };

    let bytes = key.to_bytes();
    assert_eq!(bytes.len(), 9);

    let parsed = DbKey::parse(&bytes).expect("failed to parse data2d key");
    assert_eq!(parsed.dimension, Dimension::Overworld);
    assert_eq!(parsed.chunk_pos.x, 100);
    assert_eq!(parsed.chunk_pos.z, 200);
    assert_eq!(parsed.tag, KeyTag::Data2D);
    assert_eq!(parsed.subchunk_y, None);
}

#[test]
fn test_nether_subchunk_key() {
    let key = DbKey {
        dimension: Dimension::Nether,
        chunk_pos: ChunkPos::new(-30, 45),
        tag: KeyTag::SubChunkPrefix,
        subchunk_y: Some(4),
    };

    let bytes = key.to_bytes();
    assert_eq!(bytes.len(), 14);

    let parsed = DbKey::parse(&bytes).expect("failed to parse nether subchunk key");
    assert_eq!(parsed.dimension, Dimension::Nether);
    assert_eq!(parsed.chunk_pos.x, -30);
    assert_eq!(parsed.chunk_pos.z, 45);
    assert_eq!(parsed.tag, KeyTag::SubChunkPrefix);
    assert_eq!(parsed.subchunk_y, Some(4));
}

#[test]
fn test_the_end_chunk_version_key() {
    let key = DbKey {
        dimension: Dimension::TheEnd,
        chunk_pos: ChunkPos::new(0, 0),
        tag: KeyTag::Version,
        subchunk_y: None,
    };

    let bytes = key.to_bytes();
    assert_eq!(bytes.len(), 13);

    let parsed = DbKey::parse(&bytes).expect("failed to parse the end version key");
    assert_eq!(parsed.dimension, Dimension::TheEnd);
    assert_eq!(parsed.tag, KeyTag::Version);
    assert_eq!(parsed.subchunk_y, None);
}
