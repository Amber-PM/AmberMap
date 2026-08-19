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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockColor {
    pub rgba: [u8; 4],
    pub is_translucent: bool,
    pub is_opaque: bool,
}

impl BlockColor {
    pub const fn opaque(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgba: [r, g, b, 255],
            is_translucent: false,
            is_opaque: true,
        }
    }

    pub const fn translucent(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            rgba: [r, g, b, a],
            is_translucent: a > 0 && a < 255,
            is_opaque: a == 255,
        }
    }

    pub const fn invisible() -> Self {
        Self {
            rgba: [0, 0, 0, 0],
            is_translucent: false,
            is_opaque: false,
        }
    }
}

pub struct ColorMap {
    entries: HashMap<&'static str, BlockColor>,
}

impl Default for ColorMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorMap {
    pub fn new() -> Self {
        let mut entries = HashMap::with_capacity(256);

        entries.insert("minecraft:air", BlockColor::invisible());
        entries.insert("minecraft:cave_air", BlockColor::invisible());
        entries.insert("minecraft:void_air", BlockColor::invisible());
        entries.insert("minecraft:structure_void", BlockColor::invisible());
        entries.insert("minecraft:light_block", BlockColor::invisible());
        entries.insert("minecraft:barrier", BlockColor::invisible());

        entries.insert("minecraft:grass_block", BlockColor::opaque(92, 157, 59));
        entries.insert("minecraft:grass", BlockColor::opaque(92, 157, 59));
        entries.insert("minecraft:dirt", BlockColor::opaque(134, 96, 67));
        entries.insert("minecraft:dirt_with_roots", BlockColor::opaque(138, 98, 70));
        entries.insert("minecraft:coarse_dirt", BlockColor::opaque(119, 85, 59));
        entries.insert("minecraft:podzol", BlockColor::opaque(90, 63, 28));
        entries.insert("minecraft:mycelium", BlockColor::opaque(111, 99, 105));
        entries.insert("minecraft:farmland", BlockColor::opaque(104, 66, 38));
        entries.insert("minecraft:dirt_path", BlockColor::opaque(159, 126, 69));
        entries.insert("minecraft:grass_path", BlockColor::opaque(159, 126, 69));
        entries.insert("minecraft:mud", BlockColor::opaque(60, 57, 60));
        entries.insert("minecraft:clay", BlockColor::opaque(160, 166, 179));

        entries.insert("minecraft:stone", BlockColor::opaque(125, 125, 125));
        entries.insert("minecraft:granite", BlockColor::opaque(154, 107, 89));
        entries.insert(
            "minecraft:polished_granite",
            BlockColor::opaque(154, 107, 89),
        );
        entries.insert("minecraft:diorite", BlockColor::opaque(188, 188, 191));
        entries.insert(
            "minecraft:polished_diorite",
            BlockColor::opaque(188, 188, 191),
        );
        entries.insert("minecraft:andesite", BlockColor::opaque(135, 135, 137));
        entries.insert(
            "minecraft:polished_andesite",
            BlockColor::opaque(135, 135, 137),
        );
        entries.insert("minecraft:deepslate", BlockColor::opaque(75, 75, 78));
        entries.insert(
            "minecraft:cobbled_deepslate",
            BlockColor::opaque(75, 75, 78),
        );
        entries.insert(
            "minecraft:polished_deepslate",
            BlockColor::opaque(70, 70, 72),
        );
        entries.insert("minecraft:tuff", BlockColor::opaque(108, 109, 102));
        entries.insert("minecraft:calcite", BlockColor::opaque(224, 225, 220));
        entries.insert(
            "minecraft:dripstone_block",
            BlockColor::opaque(134, 106, 92),
        );
        entries.insert(
            "minecraft:pointed_dripstone",
            BlockColor::opaque(134, 106, 92),
        );
        entries.insert("minecraft:bedrock", BlockColor::opaque(55, 55, 55));
        entries.insert("minecraft:cobblestone", BlockColor::opaque(120, 120, 120));
        entries.insert(
            "minecraft:mossy_cobblestone",
            BlockColor::opaque(100, 122, 95),
        );
        entries.insert("minecraft:gravel", BlockColor::opaque(136, 131, 131));
        entries.insert("minecraft:sand", BlockColor::opaque(219, 206, 153));
        entries.insert("minecraft:red_sand", BlockColor::opaque(190, 103, 34));
        entries.insert("minecraft:sandstone", BlockColor::opaque(216, 203, 155));
        entries.insert("minecraft:red_sandstone", BlockColor::opaque(181, 97, 31));
        entries.insert("minecraft:obsidian", BlockColor::opaque(20, 18, 30));
        entries.insert("minecraft:crying_obsidian", BlockColor::opaque(34, 16, 52));

        entries.insert("minecraft:coal_ore", BlockColor::opaque(105, 105, 105));
        entries.insert(
            "minecraft:deepslate_coal_ore",
            BlockColor::opaque(65, 65, 68),
        );
        entries.insert("minecraft:iron_ore", BlockColor::opaque(136, 124, 117));
        entries.insert(
            "minecraft:deepslate_iron_ore",
            BlockColor::opaque(82, 77, 74),
        );
        entries.insert("minecraft:copper_ore", BlockColor::opaque(128, 118, 112));
        entries.insert(
            "minecraft:deepslate_copper_ore",
            BlockColor::opaque(78, 74, 71),
        );
        entries.insert("minecraft:gold_ore", BlockColor::opaque(143, 136, 103));
        entries.insert(
            "minecraft:deepslate_gold_ore",
            BlockColor::opaque(88, 85, 67),
        );
        entries.insert("minecraft:redstone_ore", BlockColor::opaque(140, 95, 95));
        entries.insert(
            "minecraft:lit_redstone_ore",
            BlockColor::opaque(180, 70, 70),
        );
        entries.insert(
            "minecraft:deepslate_redstone_ore",
            BlockColor::opaque(90, 60, 60),
        );
        entries.insert("minecraft:lapis_ore", BlockColor::opaque(102, 114, 138));
        entries.insert(
            "minecraft:deepslate_lapis_ore",
            BlockColor::opaque(62, 70, 88),
        );
        entries.insert("minecraft:diamond_ore", BlockColor::opaque(114, 142, 140));
        entries.insert(
            "minecraft:deepslate_diamond_ore",
            BlockColor::opaque(68, 90, 90),
        );
        entries.insert("minecraft:emerald_ore", BlockColor::opaque(106, 136, 112));
        entries.insert(
            "minecraft:deepslate_emerald_ore",
            BlockColor::opaque(64, 88, 70),
        );

        entries.insert("minecraft:water", BlockColor::translucent(44, 98, 204, 180));
        entries.insert(
            "minecraft:flowing_water",
            BlockColor::translucent(44, 98, 204, 180),
        );
        entries.insert("minecraft:lava", BlockColor::opaque(216, 92, 18));
        entries.insert("minecraft:flowing_lava", BlockColor::opaque(216, 92, 18));

        entries.insert("minecraft:oak_log", BlockColor::opaque(103, 82, 49));
        entries.insert("minecraft:spruce_log", BlockColor::opaque(59, 39, 19));
        entries.insert("minecraft:birch_log", BlockColor::opaque(215, 215, 210));
        entries.insert("minecraft:jungle_log", BlockColor::opaque(86, 68, 27));
        entries.insert("minecraft:acacia_log", BlockColor::opaque(104, 98, 89));
        entries.insert("minecraft:dark_oak_log", BlockColor::opaque(60, 47, 26));
        entries.insert("minecraft:mangrove_log", BlockColor::opaque(84, 42, 33));
        entries.insert("minecraft:cherry_log", BlockColor::opaque(54, 27, 34));
        entries.insert("minecraft:crimson_stem", BlockColor::opaque(101, 30, 48));
        entries.insert("minecraft:warped_stem", BlockColor::opaque(43, 105, 99));

        entries.insert("minecraft:oak_planks", BlockColor::opaque(162, 130, 78));
        entries.insert("minecraft:spruce_planks", BlockColor::opaque(104, 78, 47));
        entries.insert("minecraft:birch_planks", BlockColor::opaque(196, 178, 123));
        entries.insert("minecraft:jungle_planks", BlockColor::opaque(160, 115, 80));
        entries.insert("minecraft:acacia_planks", BlockColor::opaque(168, 90, 50));
        entries.insert("minecraft:dark_oak_planks", BlockColor::opaque(66, 43, 20));
        entries.insert("minecraft:mangrove_planks", BlockColor::opaque(118, 54, 46));
        entries.insert("minecraft:cherry_planks", BlockColor::opaque(226, 178, 173));
        entries.insert("minecraft:bamboo_planks", BlockColor::opaque(194, 168, 77));
        entries.insert("minecraft:crimson_planks", BlockColor::opaque(101, 48, 70));
        entries.insert("minecraft:warped_planks", BlockColor::opaque(43, 105, 99));

        entries.insert(
            "minecraft:leaves",
            BlockColor::translucent(45, 115, 30, 240),
        );
        entries.insert(
            "minecraft:oak_leaves",
            BlockColor::translucent(45, 115, 30, 240),
        );
        entries.insert(
            "minecraft:spruce_leaves",
            BlockColor::translucent(55, 90, 55, 240),
        );
        entries.insert(
            "minecraft:birch_leaves",
            BlockColor::translucent(90, 130, 45, 240),
        );
        entries.insert(
            "minecraft:jungle_leaves",
            BlockColor::translucent(35, 120, 20, 240),
        );
        entries.insert(
            "minecraft:acacia_leaves",
            BlockColor::translucent(75, 110, 25, 240),
        );
        entries.insert(
            "minecraft:dark_oak_leaves",
            BlockColor::translucent(30, 85, 15, 240),
        );
        entries.insert(
            "minecraft:azalea_leaves",
            BlockColor::translucent(75, 105, 35, 240),
        );
        entries.insert(
            "minecraft:flowering_azalea_leaves",
            BlockColor::translucent(140, 90, 110, 240),
        );
        entries.insert(
            "minecraft:mangrove_leaves",
            BlockColor::translucent(60, 110, 30, 240),
        );
        entries.insert(
            "minecraft:cherry_leaves",
            BlockColor::translucent(230, 160, 185, 240),
        );

        entries.insert(
            "minecraft:short_grass",
            BlockColor::translucent(80, 140, 45, 200),
        );
        entries.insert(
            "minecraft:tall_grass",
            BlockColor::translucent(80, 140, 45, 200),
        );
        entries.insert("minecraft:fern", BlockColor::translucent(75, 135, 40, 200));
        entries.insert(
            "minecraft:large_fern",
            BlockColor::translucent(75, 135, 40, 200),
        );
        entries.insert("minecraft:dandelion", BlockColor::opaque(245, 215, 40));
        entries.insert("minecraft:poppy", BlockColor::opaque(220, 35, 35));
        entries.insert("minecraft:blue_orchid", BlockColor::opaque(45, 160, 225));
        entries.insert("minecraft:allium", BlockColor::opaque(180, 110, 220));
        entries.insert("minecraft:azure_bluet", BlockColor::opaque(220, 225, 230));
        entries.insert("minecraft:red_tulip", BlockColor::opaque(215, 40, 30));
        entries.insert("minecraft:orange_tulip", BlockColor::opaque(230, 115, 20));
        entries.insert("minecraft:white_tulip", BlockColor::opaque(230, 230, 235));
        entries.insert("minecraft:pink_tulip", BlockColor::opaque(235, 140, 175));
        entries.insert("minecraft:oxeye_daisy", BlockColor::opaque(225, 230, 230));
        entries.insert("minecraft:cornflower", BlockColor::opaque(65, 105, 215));
        entries.insert(
            "minecraft:lily_of_the_valley",
            BlockColor::opaque(230, 235, 235),
        );
        entries.insert("minecraft:wither_rose", BlockColor::opaque(35, 30, 30));
        entries.insert("minecraft:sunflower", BlockColor::opaque(240, 195, 30));
        entries.insert("minecraft:lilac", BlockColor::opaque(190, 135, 195));
        entries.insert("minecraft:rose_bush", BlockColor::opaque(195, 35, 35));
        entries.insert("minecraft:peony", BlockColor::opaque(220, 130, 155));
        entries.insert("minecraft:sugar_cane", BlockColor::opaque(135, 185, 80));
        entries.insert("minecraft:cactus", BlockColor::opaque(85, 125, 40));
        entries.insert("minecraft:bamboo", BlockColor::opaque(95, 145, 40));
        entries.insert("minecraft:vine", BlockColor::translucent(50, 100, 30, 180));
        entries.insert("minecraft:lily_pad", BlockColor::opaque(40, 95, 30));
        entries.insert("minecraft:waterlily", BlockColor::opaque(40, 95, 30));

        entries.insert(
            "minecraft:glass",
            BlockColor::translucent(215, 230, 240, 80),
        );
        entries.insert(
            "minecraft:glass_pane",
            BlockColor::translucent(215, 230, 240, 80),
        );
        entries.insert(
            "minecraft:tinted_glass",
            BlockColor::translucent(40, 35, 45, 200),
        );
        entries.insert("minecraft:ice", BlockColor::translucent(145, 180, 245, 190));
        entries.insert("minecraft:packed_ice", BlockColor::opaque(140, 175, 240));
        entries.insert("minecraft:blue_ice", BlockColor::opaque(115, 160, 245));
        entries.insert("minecraft:snow", BlockColor::opaque(240, 245, 245));
        entries.insert("minecraft:snow_layer", BlockColor::opaque(240, 245, 245));

        entries.insert("minecraft:netherrack", BlockColor::opaque(111, 40, 40));
        entries.insert("minecraft:crimson_nylium", BlockColor::opaque(142, 26, 40));
        entries.insert("minecraft:warped_nylium", BlockColor::opaque(43, 114, 104));
        entries.insert("minecraft:soul_sand", BlockColor::opaque(81, 62, 50));
        entries.insert("minecraft:soul_soil", BlockColor::opaque(75, 57, 46));
        entries.insert("minecraft:basalt", BlockColor::opaque(80, 81, 86));
        entries.insert("minecraft:smooth_basalt", BlockColor::opaque(75, 76, 80));
        entries.insert("minecraft:polished_basalt", BlockColor::opaque(85, 86, 90));
        entries.insert("minecraft:blackstone", BlockColor::opaque(42, 38, 46));
        entries.insert("minecraft:glowstone", BlockColor::opaque(189, 147, 94));
        entries.insert("minecraft:magma", BlockColor::opaque(142, 66, 27));
        entries.insert("minecraft:nether_bricks", BlockColor::opaque(44, 21, 26));

        entries.insert("minecraft:end_stone", BlockColor::opaque(220, 223, 158));
        entries.insert(
            "minecraft:end_stone_bricks",
            BlockColor::opaque(218, 221, 155),
        );
        entries.insert("minecraft:purpur_block", BlockColor::opaque(169, 125, 169));

        entries.insert("minecraft:stone_bricks", BlockColor::opaque(122, 122, 122));
        entries.insert(
            "minecraft:mossy_stone_bricks",
            BlockColor::opaque(110, 124, 105),
        );
        entries.insert(
            "minecraft:cracked_stone_bricks",
            BlockColor::opaque(115, 115, 115),
        );
        entries.insert(
            "minecraft:chiseled_stone_bricks",
            BlockColor::opaque(118, 118, 118),
        );
        entries.insert("minecraft:bricks", BlockColor::opaque(150, 70, 56));
        entries.insert("minecraft:brick_block", BlockColor::opaque(150, 70, 56));
        entries.insert("minecraft:mud_bricks", BlockColor::opaque(137, 104, 76));
        entries.insert("minecraft:prismarine", BlockColor::opaque(100, 158, 147));
        entries.insert(
            "minecraft:prismarine_bricks",
            BlockColor::opaque(98, 170, 158),
        );
        entries.insert("minecraft:dark_prismarine", BlockColor::opaque(52, 90, 76));
        entries.insert("minecraft:sea_lantern", BlockColor::opaque(172, 198, 192));
        entries.insert("minecraft:torch", BlockColor::translucent(255, 215, 0, 180));

        Self { entries }
    }

    #[inline(always)]
    pub fn get(&self, block_name: &str) -> BlockColor {
        if let Some(c) = self.entries.get(block_name) {
            return *c;
        }

        if block_name.contains("air") {
            return BlockColor::invisible();
        }
        if block_name.contains("water") {
            return BlockColor::translucent(44, 98, 204, 180);
        }
        if block_name.contains("lava") {
            return BlockColor::opaque(216, 92, 18);
        }
        if block_name.contains("leaves") {
            return BlockColor::translucent(50, 110, 30, 240);
        }
        if block_name.contains("log") || block_name.contains("wood") {
            return BlockColor::opaque(110, 85, 50);
        }
        if block_name.contains("plank") {
            return BlockColor::opaque(160, 130, 80);
        }
        if block_name.contains("stone") || block_name.contains("rock") {
            return BlockColor::opaque(125, 125, 125);
        }
        if block_name.contains("sand") {
            return BlockColor::opaque(215, 200, 150);
        }
        if block_name.contains("dirt") || block_name.contains("soil") {
            return BlockColor::opaque(130, 95, 65);
        }
        if block_name.contains("glass") {
            return BlockColor::translucent(215, 230, 240, 80);
        }

        BlockColor::opaque(128, 128, 128)
    }

    #[inline(always)]
    pub fn is_air(&self, block_name: &str) -> bool {
        matches!(
            block_name,
            "minecraft:air"
                | "minecraft:cave_air"
                | "minecraft:void_air"
                | "minecraft:structure_void"
                | "minecraft:light_block"
                | "minecraft:barrier"
        )
    }
}
