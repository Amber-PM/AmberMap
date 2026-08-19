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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl BlockPos {
    #[inline(always)]
    pub const fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(self.x >> 4, self.z >> 4)
    }

    #[inline(always)]
    pub fn subchunk_y(&self) -> i8 {
        (self.y >> 4) as i8
    }

    #[inline(always)]
    pub fn local_x(&self) -> usize {
        (self.x & 15) as usize
    }

    #[inline(always)]
    pub fn local_y(&self) -> usize {
        (self.y & 15) as usize
    }

    #[inline(always)]
    pub fn local_z(&self) -> usize {
        (self.z & 15) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    #[inline(always)]
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    #[inline(always)]
    pub fn min_block_x(&self) -> i32 {
        self.x << 4
    }

    #[inline(always)]
    pub fn min_block_z(&self) -> i32 {
        self.z << 4
    }
}

#[inline(always)]
pub const fn block_subchunk_index(x: usize, y: usize, z: usize) -> usize {
    (x << 8) | (z << 4) | y
}
