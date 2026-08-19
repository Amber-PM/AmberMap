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

use ambermap::nbt::reader::NbtReader;
use ambermap::subchunk::palette::BlockState;
use byteorder::{ByteOrder, LittleEndian};

pub fn create_test_block_nbt(name: &str, version: i32) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.push(0x0a);
    let mut len_buf = [0u8; 2];
    LittleEndian::write_u16(&mut len_buf, 0);
    buf.extend_from_slice(&len_buf);

    buf.push(0x08);
    LittleEndian::write_u16(&mut len_buf, 4);
    buf.extend_from_slice(&len_buf);
    buf.extend_from_slice(b"name");
    LittleEndian::write_u16(&mut len_buf, name.len() as u16);
    buf.extend_from_slice(&len_buf);
    buf.extend_from_slice(name.as_bytes());

    buf.push(0x03);
    LittleEndian::write_u16(&mut len_buf, 7);
    buf.extend_from_slice(&len_buf);
    buf.extend_from_slice(b"version");
    let mut int_buf = [0u8; 4];
    LittleEndian::write_i32(&mut int_buf, version);
    buf.extend_from_slice(&int_buf);

    buf.push(0x0a);
    LittleEndian::write_u16(&mut len_buf, 6);
    buf.extend_from_slice(&len_buf);
    buf.extend_from_slice(b"states");
    buf.push(0x00);

    buf.push(0x00);

    buf
}

#[test]
fn test_nbt_reader_blockstate() {
    let raw = create_test_block_nbt("minecraft:stone", 17959425);
    let mut reader = NbtReader::new(&raw);
    let (root_name, tag) = reader.read_root_compound().expect("failed to read nbt compound");

    assert_eq!(root_name, "");
    assert_eq!(tag.get_string("name"), Some("minecraft:stone"));
    assert_eq!(tag.get_i32("version"), Some(17959425));

    let block_state = BlockState::from_nbt(&tag).expect("failed to convert from nbt");
    assert_eq!(block_state.name, "minecraft:stone");
    assert_eq!(block_state.version, 17959425);
}
