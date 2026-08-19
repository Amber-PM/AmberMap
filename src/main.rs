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
        #[arg(value_name = "WORLD", help = "Path to Bedrock world directory or db folder")]
        world: PathBuf,

        #[arg(short, long, default_value = "./public/map", help = "Output directory")]
        output: PathBuf,

        #[arg(short, long, default_value = "overworld", help = "Dimension (overworld/nether/the_end)")]
        dimension: String,

        #[arg(short, long, default_value_t = 5, help = "Zoom levels")]
        zoom_levels: u8,

        #[arg(short, long, default_value_t = 0, help = "Worker threads (0 = auto)")]
        threads: usize,
    },
    Inspect {
        #[arg(value_name = "WORLD", help = "Path to Bedrock world directory or db folder")]
        world: PathBuf,

        #[arg(long, default_value_t = 0, help = "Chunk X coordinate")]
        chunk_x: i32,

        #[arg(long, default_value_t = 0, help = "Chunk Z coordinate")]
        chunk_z: i32,

        #[arg(short, long, default_value = "overworld", help = "Dimension (overworld/nether/the_end)")]
        dimension: String,
    },
}

fn parse_dimension(dim: &str) -> Dimension {
    match dim.to_lowercase().as_str() {
        "nether" => Dimension::Nether,
        "the_end" | "end" => Dimension::TheEnd,
        _ => Dimension::Overworld,
    }
}

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
        Commands::Render { world, .. } => {
            println!("[info] Render pipeline (Phase 2)");
            println!("[info] World path: {:?}", world);
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
                println!("[info] Database contains {} chunks", format_number(available.len()));
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
            println!("   SubChunks        {} (Y: {}..{})", total_subchunks, min_y, max_y);
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
    }
}
