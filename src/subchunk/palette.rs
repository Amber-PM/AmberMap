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

use crate::nbt::tag::NbtTag;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockState {
    pub name: String,
    pub states: HashMap<String, NbtTag>,
    pub version: i32,
}

impl BlockState {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            states: HashMap::new(),
            version: 0,
        }
    }

    pub fn from_nbt(tag: &NbtTag) -> Option<Self> {
        let name = tag.get_string("name")?.to_string();
        let version = tag.get_i32("version").unwrap_or(0);
        let states = tag.get_compound("states").cloned().unwrap_or_default();
        Some(Self {
            name,
            states,
            version,
        })
    }

    #[inline(always)]
    pub fn is_air(&self) -> bool {
        self.name == "minecraft:air" || self.name == "minecraft:structure_void"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    entries: Vec<BlockState>,
}

impl Palette {
    pub fn new(entries: Vec<BlockState>) -> Self {
        Self { entries }
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&BlockState> {
        self.entries.get(index)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
