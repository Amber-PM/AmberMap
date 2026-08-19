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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TileCoord {
    pub zoom: u8,
    pub x: i32,
    pub y: i32,
}

impl TileCoord {
    #[inline(always)]
    pub const fn new(zoom: u8, x: i32, y: i32) -> Self {
        Self { zoom, x, y }
    }

    #[inline(always)]
    pub fn from_chunk_pos(chunk: ChunkPos, zoom: u8) -> Self {
        Self {
            zoom,
            x: chunk.x.div_euclid(16),
            y: chunk.z.div_euclid(16),
        }
    }

    #[inline(always)]
    pub fn chunk_offset_in_tile(chunk: ChunkPos) -> (usize, usize) {
        let ox = (chunk.x.rem_euclid(16) as usize) * 16;
        let oy = (chunk.z.rem_euclid(16) as usize) * 16;
        (ox, oy)
    }

    #[inline(always)]
    pub fn parent(&self) -> Option<Self> {
        if self.zoom == 0 {
            return None;
        }
        Some(Self {
            zoom: self.zoom - 1,
            x: self.x.div_euclid(2),
            y: self.y.div_euclid(2),
        })
    }

    #[inline(always)]
    pub fn quadrant_in_parent(&self) -> (usize, usize) {
        (self.x.rem_euclid(2) as usize, self.y.rem_euclid(2) as usize)
    }

    #[inline(always)]
    pub fn children(&self) -> [Self; 4] {
        let next_z = self.zoom + 1;
        let bx = self.x * 2;
        let by = self.y * 2;
        [
            Self::new(next_z, bx, by),
            Self::new(next_z, bx + 1, by),
            Self::new(next_z, bx, by + 1),
            Self::new(next_z, bx + 1, by + 1),
        ]
    }
}
