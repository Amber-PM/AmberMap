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

use crate::tiler::compositor::{TileBuffer, TILE_SIZE};

pub fn downsample_quadrants(children: &[Option<&TileBuffer>; 4]) -> TileBuffer {
    let mut parent = TileBuffer::new();

    let quadrants = [(0, 0), (1, 0), (0, 1), (1, 1)];

    for (quad_idx, &(qx, qy)) in quadrants.iter().enumerate() {
        let child = match children[quad_idx] {
            Some(c) => c,
            None => continue,
        };

        for py in 0..128 {
            for px in 0..128 {
                let cy0 = py * 2;
                let cy1 = cy0 + 1;
                let cx0 = px * 2;
                let cx1 = cx0 + 1;

                let p00_idx = (cy0 * TILE_SIZE + cx0) * 4;
                let p10_idx = (cy0 * TILE_SIZE + cx1) * 4;
                let p01_idx = (cy1 * TILE_SIZE + cx0) * 4;
                let p11_idx = (cy1 * TILE_SIZE + cx1) * 4;

                let a00 = child.pixels[p00_idx + 3] as f32;
                let a10 = child.pixels[p10_idx + 3] as f32;
                let a01 = child.pixels[p01_idx + 3] as f32;
                let a11 = child.pixels[p11_idx + 3] as f32;

                let total_a = a00 + a10 + a01 + a11;

                let dst_x = qx * 128 + px;
                let dst_y = qy * 128 + py;
                let dst_offset = (dst_y * TILE_SIZE + dst_x) * 4;

                if total_a <= 0.0 {
                    continue;
                }

                let r = ((child.pixels[p00_idx] as f32) * a00
                    + (child.pixels[p10_idx] as f32) * a10
                    + (child.pixels[p01_idx] as f32) * a01
                    + (child.pixels[p11_idx] as f32) * a11)
                    / total_a;

                let g = ((child.pixels[p00_idx + 1] as f32) * a00
                    + (child.pixels[p10_idx + 1] as f32) * a10
                    + (child.pixels[p01_idx + 1] as f32) * a01
                    + (child.pixels[p11_idx + 1] as f32) * a11)
                    / total_a;

                let b = ((child.pixels[p00_idx + 2] as f32) * a00
                    + (child.pixels[p10_idx + 2] as f32) * a10
                    + (child.pixels[p01_idx + 2] as f32) * a01
                    + (child.pixels[p11_idx + 2] as f32) * a11)
                    / total_a;

                let avg_a = total_a / 4.0;

                parent.pixels[dst_offset] = r.round().clamp(0.0, 255.0) as u8;
                parent.pixels[dst_offset + 1] = g.round().clamp(0.0, 255.0) as u8;
                parent.pixels[dst_offset + 2] = b.round().clamp(0.0, 255.0) as u8;
                parent.pixels[dst_offset + 3] = avg_a.round().clamp(0.0, 255.0) as u8;
            }
        }
    }

    parent
}
