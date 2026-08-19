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
use crate::core::coordinates::block_subchunk_index;

#[derive(Debug, Clone, PartialEq)]
pub struct SubChunkStorage {
    pub blocks: Box<[u16; 4096]>,
    pub palette: Palette,
}

impl SubChunkStorage {
    pub fn new(blocks: Box<[u16; 4096]>, palette: Palette) -> Self {
        Self { blocks, palette }
    }

    #[inline(always)]
    pub fn get_palette_index(&self, x: usize, y: usize, z: usize) -> usize {
        let idx = block_subchunk_index(x, y, z);
        self.blocks[idx] as usize
    }

    #[inline(always)]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&BlockState> {
        let palette_idx = self.get_palette_index(x, y, z);
        self.palette.get(palette_idx)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubChunk {
    pub version: u8,
    pub subchunk_y: Option<i8>,
    pub layers: Vec<SubChunkStorage>,
}

impl SubChunk {
    #[inline(always)]
    pub fn get_block(&self, layer: usize, x: usize, y: usize, z: usize) -> Option<&BlockState> {
        self.layers.get(layer)?.get_block(x, y, z)
    }
}
