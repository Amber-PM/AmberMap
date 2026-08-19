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

use crate::leveldb::reader::ChunkData;
use crate::raster::colormap::ColorMap;
use crate::raster::hillshade::apply_hillshade;
use crate::raster::traverser::traverse_chunk;

pub struct ChunkRasterResult {
    pub pixels: [u8; 16 * 16 * 4],
    pub heightmap: [i16; 16 * 16],
}

pub fn rasterize_chunk(
    chunk: &ChunkData,
    colormap: &ColorMap,
    enable_hillshade: bool,
) -> ChunkRasterResult {
    let scan = traverse_chunk(chunk, colormap);
    let mut pixels = scan.pixels;
    let heightmap = scan.heightmap;

    if enable_hillshade {
        apply_hillshade(&mut pixels, &heightmap);
    }

    ChunkRasterResult { pixels, heightmap }
}
