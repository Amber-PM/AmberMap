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

mod nbt_tests;

use ambermap::core::coordinates::block_subchunk_index;
use ambermap::subchunk::parser::SubChunkParser;
use byteorder::{ByteOrder, LittleEndian};
use nbt_tests::create_test_block_nbt;

fn build_synthetic_subchunk_v8(
    bits_per_block: usize,
    indices: &[u16; 4096],
    palette_names: &[&str],
) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.push(8);
    buf.push(1);

    let header = ((bits_per_block as u8) << 1) & 0xfe;
    buf.push(header);

    if bits_per_block > 0 {
        let blocks_per_word = 32 / bits_per_block;
        let words_count = (4096 + blocks_per_word - 1) / blocks_per_word;

        let mut block_idx = 0;
        for _ in 0..words_count {
            let mut word = 0u32;
            let to_write = (4096 - block_idx).min(blocks_per_word);
            for bit_i in 0..to_write {
                let p_idx = (indices[block_idx] as u32) & ((1 << bits_per_block) - 1);
                word |= p_idx << (bit_i * bits_per_block);
                block_idx += 1;
            }
            let mut word_buf = [0u8; 4];
            LittleEndian::write_u32(&mut word_buf, word);
            buf.extend_from_slice(&word_buf);
        }
    }

    let mut pal_size_buf = [0u8; 4];
    LittleEndian::write_i32(&mut pal_size_buf, palette_names.len() as i32);
    buf.extend_from_slice(&pal_size_buf);

    for &name in palette_names {
        let nbt = create_test_block_nbt(name, 18090528);
        buf.extend_from_slice(&nbt);
    }

    buf
}

#[test]
fn test_subchunk_v8_zero_bits_single_block() {
    let indices = [0u16; 4096];
    let names = ["minecraft:stone"];
    let raw = build_synthetic_subchunk_v8(0, &indices, &names);

    let subchunk = SubChunkParser::parse(&raw).expect("failed to parse v8 0-bits subchunk");
    assert_eq!(subchunk.version, 8);
    assert_eq!(subchunk.layers.len(), 1);

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let block = subchunk.get_block(0, x, y, z).expect("block not found");
                assert_eq!(block.name, "minecraft:stone");
            }
        }
    }
}

#[test]
fn test_subchunk_v8_1_bit_checkerboard() {
    let mut indices = [0u16; 4096];
    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let idx = block_subchunk_index(x, y, z);
                indices[idx] = ((x + y + z) % 2) as u16;
            }
        }
    }

    let names = ["minecraft:grass_block", "minecraft:dirt"];
    let raw = build_synthetic_subchunk_v8(1, &indices, &names);

    let subchunk = SubChunkParser::parse(&raw).expect("failed to parse v8 1-bit subchunk");
    assert_eq!(subchunk.version, 8);
    assert_eq!(subchunk.layers.len(), 1);

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let block = subchunk.get_block(0, x, y, z).expect("block not found");
                let expected = if (x + y + z) % 2 == 0 {
                    "minecraft:grass_block"
                } else {
                    "minecraft:dirt"
                };
                assert_eq!(block.name, expected);
            }
        }
    }
}

#[test]
fn test_subchunk_v8_4_bits_multi_blocks() {
    let mut indices = [0u16; 4096];
    for i in 0..4096 {
        indices[i] = (i % 16) as u16;
    }

    let mut names = Vec::new();
    for i in 0..16 {
        names.push(match i {
            0 => "minecraft:air",
            1 => "minecraft:stone",
            2 => "minecraft:granite",
            3 => "minecraft:diorite",
            4 => "minecraft:andesite",
            5 => "minecraft:dirt",
            6 => "minecraft:grass_block",
            7 => "minecraft:bedrock",
            8 => "minecraft:water",
            9 => "minecraft:lava",
            10 => "minecraft:sand",
            11 => "minecraft:gravel",
            12 => "minecraft:gold_ore",
            13 => "minecraft:iron_ore",
            14 => "minecraft:coal_ore",
            _ => "minecraft:oak_log",
        });
    }

    let raw = build_synthetic_subchunk_v8(4, &indices, &names);
    let subchunk = SubChunkParser::parse(&raw).expect("failed to parse v8 4-bits subchunk");

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let idx = block_subchunk_index(x, y, z);
                let block = subchunk.get_block(0, x, y, z).expect("block not found");
                let expected_name = names[idx % 16];
                assert_eq!(block.name, expected_name);
            }
        }
    }
}

#[test]
fn test_subchunk_v9_two_layers_with_sub_y() {
    let mut buf = Vec::new();
    buf.push(9);
    buf.push(2);
    buf.push((-3i8) as u8);

    buf.push(0);
    let mut pal_size_buf = [0u8; 4];
    LittleEndian::write_i32(&mut pal_size_buf, 1);
    buf.extend_from_slice(&pal_size_buf);
    buf.extend_from_slice(&create_test_block_nbt("minecraft:stone", 18090528));

    buf.push(0);
    LittleEndian::write_i32(&mut pal_size_buf, 1);
    buf.extend_from_slice(&pal_size_buf);
    buf.extend_from_slice(&create_test_block_nbt("minecraft:water", 18090528));

    let subchunk = SubChunkParser::parse(&buf).expect("failed to parse v9 two-layer subchunk");
    assert_eq!(subchunk.version, 9);
    assert_eq!(subchunk.subchunk_y, Some(-3));
    assert_eq!(subchunk.layers.len(), 2);

    let layer0_block = subchunk.get_block(0, 5, 5, 5).expect("layer 0 block missing");
    assert_eq!(layer0_block.name, "minecraft:stone");

    let layer1_block = subchunk.get_block(1, 5, 5, 5).expect("layer 1 block missing");
    assert_eq!(layer1_block.name, "minecraft:water");
}
