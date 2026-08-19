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

pub mod core;
pub mod leveldb;
pub mod nbt;
pub mod raster;
pub mod subchunk;
pub mod tiler;
pub mod web;

pub use core::{block_subchunk_index, AmberError, BlockPos, ChunkPos, Result};
pub use leveldb::{decompress_payload, ChunkData, DbKey, Dimension, KeyTag, WorldDb};
pub use raster::{
    apply_hillshade, compute_light_factor, rasterize_chunk, BlockColor, ChunkRasterResult,
    ColorMap, ColumnScanResult,
};
pub use subchunk::{BlockState, Palette, SubChunk, SubChunkParser, SubChunkStorage};
pub use tiler::{
    downsample_quadrants, render_world_map, save_tile, RenderOptions, RenderStats, TileBuffer,
    TileCoord, TileFormat,
};
pub use web::{get_embedded_asset, resolve_mime, start_server, ServerConfig, WebAssets};
