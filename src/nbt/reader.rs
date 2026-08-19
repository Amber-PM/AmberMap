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

use super::tag::NbtTag;
use crate::core::error::{AmberError, Result};
use byteorder::{ByteOrder, LittleEndian};
use std::collections::HashMap;

pub struct NbtReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> NbtReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn read_root_compound(&mut self) -> Result<(String, NbtTag)> {
        let tag_type = self.read_u8()?;
        if tag_type != 0x0a {
            return Err(AmberError::NbtError(format!(
                "expected compound tag 0x0a at root, found 0x{tag_type:02x}"
            )));
        }

        let name = self.read_string()?;
        let compound = self.read_compound_payload()?;
        Ok((name, compound))
    }

    fn read_u8(&mut self) -> Result<u8> {
        if self.offset >= self.data.len() {
            return Err(AmberError::BufferUnderflow {
                needed: 1,
                remaining: 0,
            });
        }
        let b = self.data[self.offset];
        self.offset += 1;
        Ok(b)
    }

    fn read_i8(&mut self) -> Result<i8> {
        self.read_u8().map(|b| b as i8)
    }

    fn read_i16(&mut self) -> Result<i16> {
        let bytes = self.read_bytes(2)?;
        Ok(LittleEndian::read_i16(bytes))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let bytes = self.read_bytes(4)?;
        Ok(LittleEndian::read_i32(bytes))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let bytes = self.read_bytes(8)?;
        Ok(LittleEndian::read_i64(bytes))
    }

    fn read_f32(&mut self) -> Result<f32> {
        let bytes = self.read_bytes(4)?;
        Ok(LittleEndian::read_f32(bytes))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let bytes = self.read_bytes(8)?;
        Ok(LittleEndian::read_f64(bytes))
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let remaining = self.data.len().saturating_sub(self.offset);
        if remaining < len {
            return Err(AmberError::BufferUnderflow {
                needed: len,
                remaining,
            });
        }
        let slice = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Ok(slice)
    }

    fn read_string(&mut self) -> Result<String> {
        let len = self.read_i16()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|e| AmberError::NbtError(format!("invalid utf8 string: {e}")))
    }

    fn read_compound_payload(&mut self) -> Result<NbtTag> {
        let mut map = HashMap::new();
        loop {
            let tag_type = self.read_u8()?;
            if tag_type == 0x00 {
                break;
            }
            let key = self.read_string()?;
            let val = self.read_tag_payload(tag_type)?;
            map.insert(key, val);
        }
        Ok(NbtTag::Compound(map))
    }

    fn read_tag_payload(&mut self, tag_type: u8) -> Result<NbtTag> {
        match tag_type {
            0x00 => Ok(NbtTag::End),
            0x01 => Ok(NbtTag::Byte(self.read_i8()?)),
            0x02 => Ok(NbtTag::Short(self.read_i16()?)),
            0x03 => Ok(NbtTag::Int(self.read_i32()?)),
            0x04 => Ok(NbtTag::Long(self.read_i64()?)),
            0x05 => Ok(NbtTag::Float(self.read_f32()?)),
            0x06 => Ok(NbtTag::Double(self.read_f64()?)),
            0x07 => {
                let len = self.read_i32()? as usize;
                let bytes = self.read_bytes(len)?.to_vec();
                Ok(NbtTag::ByteArray(bytes))
            }
            0x08 => Ok(NbtTag::String(self.read_string()?)),
            0x09 => {
                let elem_type = self.read_u8()?;
                let len = self.read_i32()? as usize;
                let mut list = Vec::with_capacity(len.min(4096));
                for _ in 0..len {
                    list.push(self.read_tag_payload(elem_type)?);
                }
                Ok(NbtTag::List(list))
            }
            0x0a => self.read_compound_payload(),
            0x0b => {
                let len = self.read_i32()? as usize;
                let mut ints = Vec::with_capacity(len.min(4096));
                for _ in 0..len {
                    ints.push(self.read_i32()?);
                }
                Ok(NbtTag::IntArray(ints))
            }
            0x0c => {
                let len = self.read_i32()? as usize;
                let mut longs = Vec::with_capacity(len.min(4096));
                for _ in 0..len {
                    longs.push(self.read_i64()?);
                }
                Ok(NbtTag::LongArray(longs))
            }
            _ => Err(AmberError::NbtError(format!(
                "unrecognized nbt tag type 0x{tag_type:02x}"
            ))),
        }
    }
}
