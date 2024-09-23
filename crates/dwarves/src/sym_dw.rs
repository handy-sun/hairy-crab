#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(dead_code)]

// use regex::bytes::Regex;
use std::borrow::{self, Cow};
// use std::cmp;
// use std::collections::HashMap;
// use std::fmt::{self, Debug};
// use std::fs;
// use std::error;

use symbolic::debuginfo::dwarf::*;
use symbolic::debuginfo::elf::*;

pub struct SymDwarf<'a> {
    elf: ElfObject<'a>,
}

type UniteError = Box<dyn std::error::Error + Send + Sync + 'static>;

impl<'a> SymDwarf<'a> {
    pub fn parse(byte_slice: &'a [u8]) -> Result<Self, ElfError> {
        Ok(Self {
            elf: ElfObject::parse(byte_slice)?,
        })
    }

    pub fn loop_sym(&self) {
        let mut iter = self.elf.symbols();
        while let Some(elf_sym) = iter.next() {
            println!(
                "{:#x} | {:?} | {}",
                elf_sym.address,
                elf_sym.name(),
                elf_sym.size
            );
        }
    }

    pub fn dump_deubg_info(&self, _name: &str) -> Result<(), UniteError> {
        match self.elf.section("debug_info") {
            Some(dw_section) => {
                println!(
                    "{:#x} | {} | {}",
                    dw_section.address, dw_section.align, dw_section.offset
                );
                let row_data_len = dw_section.data.len();
                println!("data_len: {}", row_data_len);
                // let size = usize::min(row_data_len, 32);
                // println!("data:{:?}", dw_section.data.get(..size));
                Ok(())
            }
            None => Err("Connot contains debug_info".into()),
        }
    }
}
