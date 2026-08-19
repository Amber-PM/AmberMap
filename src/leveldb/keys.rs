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

use crate::core::coordinates::ChunkPos;
use crate::core::error::{AmberError, Result};
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Overworld = 0,
    Nether = 1,
    TheEnd = 2,
}

impl Dimension {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Overworld),
            1 => Some(Self::Nether),
            2 => Some(Self::TheEnd),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyTag {
    Data2D,
    Data3D,
    SubChunkPrefix,
    LegacyTerrain,
    BlockEntity,
    Entity,
    PendingTicks,
    BlockTicks,
    BiomeState,
    Version,
    Other(u8),
}

impl KeyTag {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0x2b => Self::Data2D,
            0x2c => Self::Data3D,
            0x2f => Self::SubChunkPrefix,
            0x30 => Self::LegacyTerrain,
            0x31 => Self::BlockEntity,
            0x32 => Self::Entity,
            0x33 => Self::PendingTicks,
            0x34 => Self::BlockTicks,
            0x35 => Self::BiomeState,
            0x76 => Self::Version,
            other => Self::Other(other),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Data2D => 0x2b,
            Self::Data3D => 0x2c,
            Self::SubChunkPrefix => 0x2f,
            Self::LegacyTerrain => 0x30,
            Self::BlockEntity => 0x31,
            Self::Entity => 0x32,
            Self::PendingTicks => 0x33,
            Self::BlockTicks => 0x34,
            Self::BiomeState => 0x35,
            Self::Version => 0x76,
            Self::Other(v) => *v,
        }
    }
}

impl From<u8> for KeyTag {
    fn from(val: u8) -> Self {
        Self::from_u8(val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbKey {
    pub dimension: Dimension,
    pub chunk_pos: ChunkPos,
    pub tag: KeyTag,
    pub subchunk_y: Option<i8>,
}

impl DbKey {
    pub fn parse(raw: &[u8]) -> Result<Self> {
        let len = raw.len();
        if len < 9 {
            return Err(AmberError::InvalidKeyLength {
                expected: 9,
                actual: len,
            });
        }

        let chunk_x = LittleEndian::read_i32(&raw[0..4]);
        let chunk_z = LittleEndian::read_i32(&raw[4..8]);
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        if len == 9 || len == 10 {
            let tag = KeyTag::from(raw[8]);
            let subchunk_y = if len == 10 { Some(raw[9] as i8) } else { None };
            return Ok(Self {
                dimension: Dimension::Overworld,
                chunk_pos,
                tag,
                subchunk_y,
            });
        }

        if len == 13 || len == 14 {
            let dim_id = LittleEndian::read_i32(&raw[8..12]);
            let dimension = Dimension::from_id(dim_id).unwrap_or(Dimension::Overworld);
            let tag = KeyTag::from(raw[12]);
            let subchunk_y = if len == 14 { Some(raw[13] as i8) } else { None };
            return Ok(Self {
                dimension,
                chunk_pos,
                tag,
                subchunk_y,
            });
        }

        Err(AmberError::InvalidKeyLength {
            expected: 9,
            actual: len,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(14);
        let mut xz = [0u8; 8];
        LittleEndian::write_i32(&mut xz[0..4], self.chunk_pos.x);
        LittleEndian::write_i32(&mut xz[4..8], self.chunk_pos.z);
        bytes.extend_from_slice(&xz);

        if self.dimension != Dimension::Overworld {
            let mut dim_buf = [0u8; 4];
            LittleEndian::write_i32(&mut dim_buf, self.dimension as i32);
            bytes.extend_from_slice(&dim_buf);
        }

        bytes.push(self.tag.as_u8());
        if let Some(sy) = self.subchunk_y {
            bytes.push(sy as u8);
        }
        bytes
    }
}
