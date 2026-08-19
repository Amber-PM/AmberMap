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

pub const TILE_SIZE: usize = 256;
pub const TILE_BYTES: usize = TILE_SIZE * TILE_SIZE * 4;

#[derive(Clone, Debug)]
pub struct TileBuffer {
    pub pixels: Box<[u8; TILE_BYTES]>,
}

impl Default for TileBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TileBuffer {
    pub fn new() -> Self {
        let boxed: Box<[u8; TILE_BYTES]> = vec![0u8; TILE_BYTES]
            .into_boxed_slice()
            .try_into()
            .unwrap_or_else(|_| unreachable!());
        Self { pixels: boxed }
    }

    pub fn from_raw(data: Box<[u8; TILE_BYTES]>) -> Self {
        Self { pixels: data }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.pixels.iter().all(|&b| b == 0)
    }

    #[inline(always)]
    pub fn blit_chunk(&mut self, ox: usize, oy: usize, chunk_pixels: &[u8; 16 * 16 * 4]) {
        for cz in 0..16 {
            let tile_row_start = ((oy + cz) * TILE_SIZE + ox) * 4;
            let chunk_row_start = cz * 16 * 4;
            self.pixels[tile_row_start..tile_row_start + 64]
                .copy_from_slice(&chunk_pixels[chunk_row_start..chunk_row_start + 64]);
        }
    }
}
