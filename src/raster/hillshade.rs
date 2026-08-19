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

#[inline(always)]
pub fn compute_light_factor(h_curr: i16, h_west: i16, h_north: i16) -> f32 {
    let delta_x = (h_curr - h_west) as f32;
    let delta_z = (h_curr - h_north) as f32;
    let slope = (delta_x + delta_z) / 2.0;
    1.0 + (slope * 0.12).clamp(-0.40, 0.30)
}

pub fn apply_hillshade(pixels: &mut [u8; 16 * 16 * 4], heightmap: &[i16; 16 * 16]) {
    for z in 0..16 {
        for x in 0..16 {
            let col_idx = z * 16 + x;
            let pixel_offset = col_idx * 4;

            if pixels[pixel_offset + 3] == 0 {
                continue;
            }

            let h_curr = heightmap[col_idx];
            let h_west = if x > 0 {
                heightmap[z * 16 + (x - 1)]
            } else {
                h_curr
            };
            let h_north = if z > 0 {
                heightmap[(z - 1) * 16 + x]
            } else {
                h_curr
            };

            let light_factor = compute_light_factor(h_curr, h_west, h_north);

            for c in 0..3 {
                let channel_val = (pixels[pixel_offset + c] as f32) * light_factor;
                pixels[pixel_offset + c] = channel_val.round().clamp(0.0, 255.0) as u8;
            }
        }
    }
}
