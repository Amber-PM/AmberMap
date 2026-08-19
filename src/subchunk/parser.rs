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

use super::palette::{BlockState, Palette};
use super::storage::{SubChunk, SubChunkStorage};
use crate::core::error::{AmberError, Result};
use crate::nbt::reader::NbtReader;
use byteorder::{ByteOrder, LittleEndian};

pub struct SubChunkParser;

impl SubChunkParser {
    pub fn parse(data: &[u8]) -> Result<SubChunk> {
        if data.is_empty() {
            return Err(AmberError::BufferUnderflow {
                needed: 1,
                remaining: 0,
            });
        }

        let version = data[0];
        let mut offset = 1;

        match version {
            8 | 9 => Self::parse_v8_v9(data, version, &mut offset),
            1 => Self::parse_v1_legacy(data, &mut offset),
            other => Err(AmberError::UnsupportedSubChunkVersion(other)),
        }
    }

    fn parse_v8_v9(data: &[u8], version: u8, offset: &mut usize) -> Result<SubChunk> {
        if *offset >= data.len() {
            return Err(AmberError::BufferUnderflow {
                needed: 1,
                remaining: 0,
            });
        }

        let storage_count = data[*offset] as usize;
        *offset += 1;

        let subchunk_y = if version == 9 {
            if *offset >= data.len() {
                return Err(AmberError::BufferUnderflow {
                    needed: 1,
                    remaining: 0,
                });
            }
            let y = data[*offset] as i8;
            *offset += 1;
            Some(y)
        } else {
            None
        };

        let mut layers = Vec::with_capacity(storage_count);
        for _ in 0..storage_count {
            let storage = Self::parse_storage_layer(data, offset)?;
            layers.push(storage);
        }

        Ok(SubChunk {
            version,
            subchunk_y,
            layers,
        })
    }

    fn parse_storage_layer(data: &[u8], offset: &mut usize) -> Result<SubChunkStorage> {
        if *offset >= data.len() {
            return Err(AmberError::BufferUnderflow {
                needed: 1,
                remaining: 0,
            });
        }

        let header = data[*offset];
        *offset += 1;

        let is_runtime = (header & 1) != 0;
        let bits_per_block = (header >> 1) as usize;

        let mut blocks = Box::new([0u16; 4096]);

        if bits_per_block != 0 {
            let blocks_per_word = 32 / bits_per_block;
            let words_count = (4096 + blocks_per_word - 1) / blocks_per_word;
            let bytes_needed = words_count * 4;

            if data.len().saturating_sub(*offset) < bytes_needed {
                return Err(AmberError::BufferUnderflow {
                    needed: bytes_needed,
                    remaining: data.len().saturating_sub(*offset),
                });
            }

            let mask = (1u32 << bits_per_block) - 1;
            let mut block_idx = 0;

            for _ in 0..words_count {
                let word = LittleEndian::read_u32(&data[*offset..*offset + 4]);
                *offset += 4;

                let mut w = word;
                let to_read = (4096 - block_idx).min(blocks_per_word);
                for _ in 0..to_read {
                    blocks[block_idx] = (w & mask) as u16;
                    w >>= bits_per_block;
                    block_idx += 1;
                }
            }
        }

        let palette = if is_runtime {
            return Err(AmberError::CorruptSubChunk(
                "runtime palette not supported in persistent leveldb parser".into(),
            ));
        } else {
            Self::parse_nbt_palette(data, offset)?
        };

        Ok(SubChunkStorage::new(blocks, palette))
    }

    fn parse_nbt_palette(data: &[u8], offset: &mut usize) -> Result<Palette> {
        if data.len().saturating_sub(*offset) < 4 {
            return Err(AmberError::BufferUnderflow {
                needed: 4,
                remaining: data.len().saturating_sub(*offset),
            });
        }

        let palette_size = LittleEndian::read_i32(&data[*offset..*offset + 4]) as usize;
        *offset += 4;

        let mut entries = Vec::with_capacity(palette_size);
        for _ in 0..palette_size {
            let mut reader = NbtReader::new(&data[*offset..]);
            let (_, root_tag) = reader.read_root_compound()?;
            *offset += reader.offset();

            let state = BlockState::from_nbt(&root_tag).ok_or_else(|| {
                AmberError::NbtError("palette compound tag missing block name".into())
            })?;
            entries.push(state);
        }

        Ok(Palette::new(entries))
    }

    fn parse_v1_legacy(data: &[u8], offset: &mut usize) -> Result<SubChunk> {
        let needed = 4096 + 2048;
        if data.len().saturating_sub(*offset) < needed {
            return Err(AmberError::BufferUnderflow {
                needed,
                remaining: data.len().saturating_sub(*offset),
            });
        }

        let block_ids = &data[*offset..*offset + 4096];
        *offset += 4096;
        let _data_nibbles = &data[*offset..*offset + 2048];
        *offset += 2048;

        let mut blocks = Box::new([0u16; 4096]);
        let mut palette_map = std::collections::HashMap::new();
        let mut palette_entries = Vec::new();

        for i in 0..4096 {
            let id = block_ids[i];
            let palette_idx = *palette_map.entry(id).or_insert_with(|| {
                let idx = palette_entries.len() as u16;
                palette_entries.push(BlockState::new(format!("legacy:id_{id}")));
                idx
            });
            blocks[i] = palette_idx;
        }

        Ok(SubChunk {
            version: 1,
            subchunk_y: None,
            layers: vec![SubChunkStorage::new(blocks, Palette::new(palette_entries))],
        })
    }
}
