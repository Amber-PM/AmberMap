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

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NbtTag>),
    Compound(HashMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    pub fn get_string(&self, key: &str) -> Option<&str> {
        match self {
            Self::Compound(map) => match map.get(key)? {
                Self::String(s) => Some(s.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        match self {
            Self::Compound(map) => match map.get(key)? {
                Self::Int(v) => Some(*v),
                Self::Byte(v) => Some(*v as i32),
                Self::Short(v) => Some(*v as i32),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_compound(&self, key: &str) -> Option<&HashMap<String, NbtTag>> {
        match self {
            Self::Compound(map) => match map.get(key)? {
                Self::Compound(c) => Some(c),
                _ => None,
            },
            _ => None,
        }
    }
}
