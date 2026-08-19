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
use ambermap::leveldb::keys::Dimension;
use ambermap::leveldb::reader::WorldDb;
use ambermap::raster::{rasterize_chunk, ColorMap};
use ambermap::tiler::{render_world_map, RenderOptions, TileFormat};
use ambermap::web::{start_server, ServerConfig};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "ambermap")]
#[command(about = "High-Performance Bedrock World Map Renderer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Render {
        #[arg(
            value_name = "WORLD",
            help = "Path to Bedrock world directory or db folder"
        )]
        world: PathBuf,

        #[arg(short, long, default_value = "./public/map", help = "Output directory")]
        output: PathBuf,

        #[arg(
            short,
            long,
            default_value = "overworld",
            help = "Dimension (overworld/nether/the_end)"
        )]
        dimension: String,

        #[arg(short = 'z', long, default_value_t = 5, help = "Zoom levels")]
        zoom_levels: u8,

        #[arg(
            short = 'f',
            long,
            default_value = "webp",
            help = "Tile format (webp/png)"
        )]
        format: String,

        #[arg(short, long, default_value_t = 0, help = "Worker threads (0 = auto)")]
        threads: usize,

        #[arg(long, help = "Disable 2.5D hillshading")]
        no_hillshading: bool,

        #[arg(short = 's', long, help = "Start web server after rendering")]
        serve: bool,

        #[arg(short = 'p', long, default_value_t = 8080, help = "Web server port")]
        port: u16,

        #[arg(short = 'b', long, default_value = "127.0.0.1", help = "Bind address")]
        bind: String,

        #[arg(long, help = "Automatically open map in default browser")]
        open: bool,
    },
    Serve {
        #[arg(
            short = 'd',
            long,
            default_value = "./public/map",
            help = "Directory containing map tiles"
        )]
        dir: PathBuf,

        #[arg(short = 'p', long, default_value_t = 8080, help = "Web server port")]
        port: u16,

        #[arg(short = 'b', long, default_value = "127.0.0.1", help = "Bind address")]
        bind: String,

        #[arg(
            short = 'f',
            long,
            default_value = "webp",
            help = "Tile format (webp/png)"
        )]
        format: String,

        #[arg(short = 'z', long, default_value_t = 5, help = "Max zoom levels")]
        zoom_levels: u8,

        #[arg(long, help = "Automatically open map in default browser")]
        open: bool,
    },
    Inspect {
        #[arg(
            value_name = "WORLD",
            help = "Path to Bedrock world directory or db folder"
        )]
        world: PathBuf,

        #[arg(short = 'x', long, default_value_t = 0, help = "Chunk X coordinate")]
        chunk_x: i32,

        #[arg(short = 'z', long, default_value_t = 0, help = "Chunk Z coordinate")]
        chunk_z: i32,

        #[arg(
            short,
            long,
            default_value = "overworld",
            help = "Dimension (overworld/nether/the_end)"
        )]
        dimension: String,
    },
    RenderChunk {
        #[arg(
            value_name = "WORLD",
            help = "Path to Bedrock world directory or db folder"
        )]
        world: PathBuf,

        #[arg(short = 'x', long, default_value_t = 0, help = "Chunk X coordinate")]
        chunk_x: i32,

        #[arg(short = 'z', long, default_value_t = 0, help = "Chunk Z coordinate")]
        chunk_z: i32,

        #[arg(
            short,
            long,
            default_value = "overworld",
            help = "Dimension (overworld/nether/the_end)"
        )]
        dimension: String,

        #[arg(short, long, default_value = "chunk.png", help = "Output PNG path")]
        output: PathBuf,

        #[arg(long, help = "Disable 2.5D hillshading")]
        no_hillshading: bool,

        #[arg(
            long,
            default_value_t = 16,
            help = "Scale multiplier (16 = 256x256 PNG)"
        )]
        scale: u32,
    },
}

fn parse_dimension(dim: &str) -> Dimension {
    match dim.to_lowercase().as_str() {
        "nether" => Dimension::Nether,
        "the_end" | "end" => Dimension::TheEnd,
        _ => Dimension::Overworld,
    }
}

#[allow(clippy::manual_is_multiple_of)]
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Render {
            world,
            output,
            dimension,
            zoom_levels,
            format,
            threads,
            no_hillshading,
            serve,
            port,
            bind,
            open,
        } => {
            let dim = parse_dimension(&dimension);
            let tile_fmt = TileFormat::from_str(&format);

            println!("[info] Scanning database for {:?} chunks...", dim);

            let options = RenderOptions {
                world_path: world.clone(),
                output_dir: output.clone(),
                dimension: dim,
                zoom_levels: zoom_levels.max(1),
                format: tile_fmt,
                threads,
                enable_hillshading: !no_hillshading,
            };

            let thread_count = if threads == 0 {
                rayon::current_num_threads()
            } else {
                threads
            };

            println!(
                "[render] Processing base tiles (z={}) using {} threads...",
                zoom_levels.saturating_sub(1),
                thread_count
            );

            if zoom_levels > 1 {
                println!(
                    "[render] Generating pyramid levels (z{} -> z0)...",
                    zoom_levels.saturating_sub(2)
                );
            }

            match render_world_map(options) {
                Ok(stats) => {
                    let secs = stats.elapsed.as_secs_f64();
                    let speed = if secs > 0.0 {
                        (stats.total_chunks as f64) / secs
                    } else {
                        0.0
                    };

                    println!(
                        "✓ Render completed: {} tiles generated (from {} chunks) in {:.2?} ({:.0} chunks/sec)",
                        format_number(stats.total_tiles),
                        format_number(stats.total_chunks),
                        stats.elapsed,
                        speed
                    );
                    println!("[info] Tiles saved to {:?}", output);

                    if serve {
                        let srv_cfg = ServerConfig {
                            bind_addr: bind,
                            port,
                            tiles_dir: output,
                            auto_open: open,
                            tile_format: format,
                            max_zoom: zoom_levels.saturating_sub(1),
                        };
                        if let Err(e) = start_server(srv_cfg) {
                            eprintln!("[error] Server error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[error] Render failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Serve {
            dir,
            port,
            bind,
            format,
            zoom_levels,
            open,
        } => {
            let srv_cfg = ServerConfig {
                bind_addr: bind,
                port,
                tiles_dir: dir,
                auto_open: open,
                tile_format: format,
                max_zoom: zoom_levels.saturating_sub(1),
            };
            if let Err(e) = start_server(srv_cfg) {
                eprintln!("[error] Server failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Inspect {
            world,
            chunk_x,
            chunk_z,
            dimension,
        } => {
            let start = Instant::now();
            let dim = parse_dimension(&dimension);
            let pos = ChunkPos::new(chunk_x, chunk_z);

            let db = match WorldDb::open(&world) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("[error] Failed to open LevelDB at {:?}: {}", world, e);
                    std::process::exit(1);
                }
            };

            let chunk_data = match db.get_chunk(pos, dim) {
                Ok(cd) => cd,
                Err(e) => {
                    eprintln!("[error] Failed to read chunk ({}, {}): {}", pos.x, pos.z, e);
                    std::process::exit(1);
                }
            };

            println!("[info] Database: {:?}", db.path);
            println!(
                "[info] Target: Chunk ({}, {}) • {:?} • Blocks [{}, {}]",
                pos.x,
                pos.z,
                dim,
                pos.min_block_x(),
                pos.min_block_z()
            );

            if chunk_data.subchunks.is_empty() {
                eprintln!("[warn] No subchunks found at ({}, {})", pos.x, pos.z);
                let available = db.scan_chunks(dim);
                println!(
                    "[info] Database contains {} chunks",
                    format_number(available.len())
                );
                if !available.is_empty() {
                    let sample: Vec<String> = available
                        .iter()
                        .take(8)
                        .map(|c| format!("({}, {})", c.x, c.z))
                        .collect();
                    println!("   Sample coordinates: {}", sample.join(", "));
                }
                return;
            }

            let min_y = chunk_data.subchunks.first().map(|(y, _)| *y).unwrap_or(0);
            let max_y = chunk_data.subchunks.last().map(|(y, _)| *y).unwrap_or(0);
            let total_subchunks = chunk_data.subchunks.len();
            let total_blocks = total_subchunks * 4096;

            if let Some(v) = chunk_data.version {
                println!("   Format Version   {}", v);
            } else {
                println!("   Format Version   Legacy");
            }
            println!(
                "   SubChunks        {} (Y: {}..{})",
                total_subchunks, min_y, max_y
            );
            println!("   Decoded Voxels   {}", format_number(total_blocks));

            if let Some((top_y, top_subchunk)) = chunk_data.subchunks.last() {
                if let Some(layer) = top_subchunk.layers.first() {
                    let names: Vec<&str> = (0..layer.palette.len())
                        .filter_map(|i| layer.palette.get(i))
                        .map(|b| b.name.strip_prefix("minecraft:").unwrap_or(&b.name))
                        .collect();
                    println!(
                        "   Palette (Y={})    {} ({} states)",
                        top_y,
                        names.join(", "),
                        layer.palette.len()
                    );
                }
            }

            let elapsed = start.elapsed();
            println!("✓ Inspected in {:.2?}", elapsed);
        }
        Commands::RenderChunk {
            world,
            chunk_x,
            chunk_z,
            dimension,
            output,
            no_hillshading,
            scale,
        } => {
            let start = Instant::now();
            let dim = parse_dimension(&dimension);
            let pos = ChunkPos::new(chunk_x, chunk_z);

            let db = match WorldDb::open(&world) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("[error] Failed to open LevelDB at {:?}: {}", world, e);
                    std::process::exit(1);
                }
            };

            let chunk_data = match db.get_chunk(pos, dim) {
                Ok(cd) => cd,
                Err(e) => {
                    eprintln!("[error] Failed to read chunk ({}, {}): {}", pos.x, pos.z, e);
                    std::process::exit(1);
                }
            };

            println!("[info] Database: {:?}", db.path);
            println!(
                "[info] Target: Chunk ({}, {}) • {:?} • Blocks [{}, {}]",
                pos.x,
                pos.z,
                dim,
                pos.min_block_x(),
                pos.min_block_z()
            );

            if chunk_data.subchunks.is_empty() {
                eprintln!("[warn] No subchunks found at ({}, {})", pos.x, pos.z);
                return;
            }

            let colormap = ColorMap::new();
            let raster = rasterize_chunk(&chunk_data, &colormap, !no_hillshading);

            let scale = scale.max(1);
            let img_w = 16 * scale;
            let img_h = 16 * scale;
            let mut img = image::RgbaImage::new(img_w, img_h);

            for z in 0..16 {
                for x in 0..16 {
                    let idx = (z * 16 + x) * 4;
                    let pixel = image::Rgba([
                        raster.pixels[idx],
                        raster.pixels[idx + 1],
                        raster.pixels[idx + 2],
                        raster.pixels[idx + 3],
                    ]);

                    for py in 0..scale {
                        for px in 0..scale {
                            img.put_pixel((x as u32) * scale + px, (z as u32) * scale + py, pixel);
                        }
                    }
                }
            }

            if let Err(e) = img.save(&output) {
                eprintln!("[error] Failed to save PNG to {:?}: {}", output, e);
                std::process::exit(1);
            }

            let min_h = raster.heightmap.iter().copied().min().unwrap_or(0);
            let max_h = raster.heightmap.iter().copied().max().unwrap_or(0);

            println!(
                "[info] Render: {}x{} PNG • Hillshading: {}",
                img_w,
                img_h,
                if no_hillshading { "off" } else { "on" }
            );
            println!("   Surface Height   Min: {}, Max: {}", min_h, max_h);
            println!("   Output           {:?}", output);

            let elapsed = start.elapsed();
            println!("✓ Rendered in {:.2?}", elapsed);
        }
    }
}
