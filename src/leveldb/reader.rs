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

use super::sstable::read_sstable;
use crate::core::coordinates::ChunkPos;
use crate::core::error::{AmberError, Result};
use crate::leveldb::decompressor::decompress_payload;
use crate::leveldb::keys::{DbKey, Dimension, KeyTag};
use crate::subchunk::parser::SubChunkParser;
use crate::subchunk::storage::SubChunk;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ChunkData {
    pub chunk_pos: ChunkPos,
    pub dimension: Dimension,
    pub version: Option<u8>,
    pub subchunks: Vec<(i8, SubChunk)>,
}

pub struct WorldDb {
    entries: HashMap<Vec<u8>, Vec<u8>>,
    pub path: PathBuf,
}

impl WorldDb {
    pub fn open(world_path: impl AsRef<Path>) -> Result<Self> {
        let p = world_path.as_ref();
        let db_dir = if p.join("db").is_dir() {
            p.join("db")
        } else {
            p.to_path_buf()
        };

        if !db_dir.is_dir() {
            return Err(AmberError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("world db directory not found at {:?}", db_dir),
            )));
        }

        let mut entries = HashMap::new();

        if let Ok(read_dir) = fs::read_dir(&db_dir) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "ldb" || ext == "sst" {
                        let _ = read_sstable(&path, &mut entries);
                    }
                }
            }
        }

        Ok(Self {
            entries,
            path: db_dir,
        })
    }

    pub fn scan_chunks(&self, dimension: Dimension) -> Vec<ChunkPos> {
        let mut chunks = HashSet::new();

        for k in self.entries.keys() {
            if let Ok(key) = DbKey::parse(k) {
                if key.dimension == dimension {
                    chunks.insert(key.chunk_pos);
                }
            }
        }

        let mut list: Vec<ChunkPos> = chunks.into_iter().collect();
        list.sort_by_key(|c| (c.x, c.z));
        list
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos, dimension: Dimension) -> Result<ChunkData> {
        let mut subchunks = Vec::new();
        let mut version = None;

        let v_key_new = DbKey {
            dimension,
            chunk_pos,
            tag: KeyTag::Data3D,
            subchunk_y: None,
        };
        if let Some(v_raw) = self.entries.get(&v_key_new.to_bytes()) {
            if let Ok(decompressed) = decompress_payload(v_raw) {
                version = decompressed.first().copied();
            }
        }

        if version.is_none() {
            let v_key_old = DbKey {
                dimension,
                chunk_pos,
                tag: KeyTag::Version,
                subchunk_y: None,
            };
            if let Some(v_raw) = self.entries.get(&v_key_old.to_bytes()) {
                if let Ok(decompressed) = decompress_payload(v_raw) {
                    version = decompressed.first().copied();
                }
            }
        }

        let (min_y, max_y) = match dimension {
            Dimension::Overworld => (-4i8, 19i8),
            Dimension::Nether | Dimension::TheEnd => (0i8, 15i8),
        };

        for sub_y in min_y..=max_y {
            let key = DbKey {
                dimension,
                chunk_pos,
                tag: KeyTag::SubChunkPrefix,
                subchunk_y: Some(sub_y),
            };

            if let Some(raw_data) = self.entries.get(&key.to_bytes()) {
                if let Ok(decompressed) = decompress_payload(raw_data) {
                    if let Ok(subchunk) = SubChunkParser::parse(&decompressed) {
                        subchunks.push((sub_y, subchunk));
                    }
                }
            }
        }

        subchunks.sort_by_key(|(y, _)| *y);

        Ok(ChunkData {
            chunk_pos,
            dimension,
            version,
            subchunks,
        })
    }
}
