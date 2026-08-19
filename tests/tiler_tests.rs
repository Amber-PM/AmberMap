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

use ambermap::core::coordinates::ChunkPos;
use ambermap::tiler::compositor::{TileBuffer, TILE_SIZE};
use ambermap::tiler::coordinates::TileCoord;
use ambermap::tiler::pyramid::downsample_quadrants;

#[test]
fn test_chunk_to_tile_coordinates() {
    let c0 = ChunkPos::new(0, 0);
    let t0 = TileCoord::from_chunk_pos(c0, 5);
    assert_eq!(t0.x, 0);
    assert_eq!(t0.y, 0);
    assert_eq!(TileCoord::chunk_offset_in_tile(c0), (0, 0));

    let c1 = ChunkPos::new(17, 33);
    let t1 = TileCoord::from_chunk_pos(c1, 5);
    assert_eq!(t1.x, 1);
    assert_eq!(t1.y, 2);
    assert_eq!(TileCoord::chunk_offset_in_tile(c1), (16, 16));

    let c_neg1 = ChunkPos::new(-1, -1);
    let t_neg1 = TileCoord::from_chunk_pos(c_neg1, 5);
    assert_eq!(t_neg1.x, -1);
    assert_eq!(t_neg1.y, -1);
    assert_eq!(TileCoord::chunk_offset_in_tile(c_neg1), (240, 240));

    let c_neg16 = ChunkPos::new(-16, -16);
    let t_neg16 = TileCoord::from_chunk_pos(c_neg16, 5);
    assert_eq!(t_neg16.x, -1);
    assert_eq!(t_neg16.y, -1);
    assert_eq!(TileCoord::chunk_offset_in_tile(c_neg16), (0, 0));

    let c_neg17 = ChunkPos::new(-17, -33);
    let t_neg17 = TileCoord::from_chunk_pos(c_neg17, 5);
    assert_eq!(t_neg17.x, -2);
    assert_eq!(t_neg17.y, -3);
    assert_eq!(TileCoord::chunk_offset_in_tile(c_neg17), (240, 240));
}

#[test]
fn test_tile_compositor_stitching() {
    let mut tile = TileBuffer::new();
    assert!(tile.is_empty());

    let mut chunk_pixels = [0u8; 16 * 16 * 4];
    for i in 0..16 * 16 {
        chunk_pixels[i * 4] = 255;
        chunk_pixels[i * 4 + 1] = 128;
        chunk_pixels[i * 4 + 2] = 64;
        chunk_pixels[i * 4 + 3] = 255;
    }

    tile.blit_chunk(16, 32, &chunk_pixels);
    assert!(!tile.is_empty());

    let top_left_idx = (0 * TILE_SIZE + 0) * 4;
    assert_eq!(&tile.pixels[top_left_idx..top_left_idx + 4], &[0, 0, 0, 0]);

    let blit_start_idx = (32 * TILE_SIZE + 16) * 4;
    assert_eq!(
        &tile.pixels[blit_start_idx..blit_start_idx + 4],
        &[255, 128, 64, 255]
    );

    let blit_end_idx = ((32 + 15) * TILE_SIZE + (16 + 15)) * 4;
    assert_eq!(
        &tile.pixels[blit_end_idx..blit_end_idx + 4],
        &[255, 128, 64, 255]
    );
}

#[test]
fn test_pyramid_downsampling_alpha_weighted() {
    let mut child_tl = TileBuffer::new();
    let mut child_tr = TileBuffer::new();
    let mut child_bl = TileBuffer::new();

    for i in 0..TILE_SIZE * TILE_SIZE {
        let idx = i * 4;
        child_tl.pixels[idx..idx + 4].copy_from_slice(&[200, 100, 50, 255]);
        child_tr.pixels[idx..idx + 4].copy_from_slice(&[0, 200, 100, 255]);
        child_bl.pixels[idx..idx + 4].copy_from_slice(&[100, 50, 200, 255]);
    }

    let children = [Some(&child_tl), Some(&child_tr), Some(&child_bl), None];

    let parent = downsample_quadrants(&children);

    let p_tl = (0 * TILE_SIZE + 0) * 4;
    assert_eq!(&parent.pixels[p_tl..p_tl + 4], &[200, 100, 50, 255]);

    let p_tr = (0 * TILE_SIZE + 128) * 4;
    assert_eq!(&parent.pixels[p_tr..p_tr + 4], &[0, 200, 100, 255]);

    let p_bl = (128 * TILE_SIZE + 0) * 4;
    assert_eq!(&parent.pixels[p_bl..p_bl + 4], &[100, 50, 200, 255]);

    let p_br = (128 * TILE_SIZE + 128) * 4;
    assert_eq!(&parent.pixels[p_br..p_br + 4], &[0, 0, 0, 0]);
}
