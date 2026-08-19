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

pub struct ColumnScanResult {
    pub pixels: [u8; 16 * 16 * 4],
    pub heightmap: [i16; 16 * 16],
}

pub fn traverse_chunk(chunk: &ChunkData, colormap: &ColorMap) -> ColumnScanResult {
    let mut pixels = [0u8; 16 * 16 * 4];
    let mut heightmap = [0i16; 16 * 16];

    for z in 0..16 {
        for x in 0..16 {
            let col_idx = z * 16 + x;
            let pixel_offset = col_idx * 4;

            let mut acc_r = 0.0f32;
            let mut acc_g = 0.0f32;
            let mut acc_b = 0.0f32;
            let mut acc_a = 0.0f32;
            let mut recorded_height: Option<i16> = None;

            // ray-march top-down through subchunks
            'ray: for (sub_y, subchunk) in chunk.subchunks.iter().rev() {
                let base_y = (*sub_y as i32) * 16;

                for local_y in (0..16).rev() {
                    let world_y = base_y + (local_y as i32);

                    // inspect layers top-to-bottom (layer 1 waterlogged before layer 0 base)
                    for layer in subchunk.layers.iter().rev() {
                        let block_idx = (x << 8) | (z << 4) | local_y;
                        let palette_idx = layer.blocks[block_idx] as usize;

                        let block_name = match layer.palette.get(palette_idx) {
                            Some(b) => b.name.as_str(),
                            None => continue,
                        };

                        if colormap.is_air(block_name) {
                            continue;
                        }

                        let color = colormap.get(block_name);
                        if color.rgba[3] == 0 {
                            continue;
                        }

                        if recorded_height.is_none() {
                            recorded_height = Some(world_y as i16);
                        }

                        let src_a = (color.rgba[3] as f32) / 255.0;
                        let weight = src_a * (1.0 - acc_a);

                        acc_r += (color.rgba[0] as f32) * weight;
                        acc_g += (color.rgba[1] as f32) * weight;
                        acc_b += (color.rgba[2] as f32) * weight;
                        acc_a += weight;

                        if acc_a >= 0.98 {
                            acc_a = 1.0;
                            break 'ray;
                        }
                    }
                }
            }

            if acc_a > 0.0 {
                pixels[pixel_offset] = acc_r.round().clamp(0.0, 255.0) as u8;
                pixels[pixel_offset + 1] = acc_g.round().clamp(0.0, 255.0) as u8;
                pixels[pixel_offset + 2] = acc_b.round().clamp(0.0, 255.0) as u8;
                pixels[pixel_offset + 3] = (acc_a * 255.0).round().clamp(0.0, 255.0) as u8;
                heightmap[col_idx] = recorded_height.unwrap_or(0);
            }
        }
    }

    ColumnScanResult { pixels, heightmap }
}
