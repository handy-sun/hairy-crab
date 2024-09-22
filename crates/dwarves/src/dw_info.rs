#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(dead_code)]

// use fallible_iterator::FallibleIterator;
use gimli::{DwarfSections, Reader, Attribute};
use object::{Object, ObjectSection};
// use regex::bytes::Regex;
use std::borrow::{self, Cow};
// use std::cmp;
// use std::collections::HashMap;
// use std::fmt::{self, Debug};
// use std::fs;
use std::error;

// This is a simple wrapper around `object::read::RelocationMap` that implements
// `gimli::read::Relocate` for use with `gimli::RelocateReader`.
// You only need this if you are parsing relocatable object files.
#[derive(Debug, Default)]
struct RelocMap(object::read::RelocationMap);

// The section data that will be stored in `DwarfSections` and `DwarfPackageSections`.
#[derive(Default)]
struct CusSection<'d> {
    data: Cow<'d, [u8]>,
    relocations: RelocMap,
}

impl<'a> gimli::read::Relocate for &'a RelocMap {
    fn relocate_address(&self, offset: usize, value: u64) -> gimli::Result<u64> {
        Ok(self.0.relocate(offset as u64, value))
    }

    fn relocate_offset(&self, offset: usize, value: usize) -> gimli::Result<usize> {
        <usize as gimli::ReaderOffset>::from_u64(self.0.relocate(offset as u64, value as u64))
    }
}

// The reader type that will be stored in `Dwarf` and `DwarfPackage`.
// If you don't need relocations, you can use `gimli::EndianSlice` directly.
type CusReader<'d> = gimli::RelocateReader<gimli::EndianSlice<'d, gimli::RunTimeEndian>, &'d RelocMap>;

// Borrow a `Section` to create a `Reader`.
fn borrow_section<'d>(
    section: &'d CusSection<'d>,
    endian: gimli::RunTimeEndian,
) -> CusReader<'d> {
    let slice = gimli::EndianSlice::new(borrow::Cow::as_ref(&section.data), endian);
    gimli::RelocateReader::new(slice, &section.relocations)
}

fn load_section<'d>(
    object: &object::File<'d>,
    name: &str,
) -> Result<CusSection<'d>, Box<dyn error::Error>> {
    Ok(match object.section_by_name(name) {
        Some(section) => CusSection {
            data: section.uncompressed_data()?,
            relocations: section.relocation_map().map(RelocMap)?,
        },
        None => Default::default(),
    })
}

pub struct DwarfMgr<'a> {
    dwarf_sections: DwarfSections<CusSection<'a>>,
}

impl<'a> DwarfMgr<'a> {
    pub fn parse(byte_slice: &'a [u8]) -> Result<Self, Box<dyn error::Error>> {

        let obj_file = object::File::parse(byte_slice)?;

        // let dwarf_sections = gimli::DwarfSections::load(|id| load_section(&obj_file, id.name()))?;
        // let endian = if obj_file.is_little_endian() {
        //     gimli::RunTimeEndian::Little
        // } else {
        //     gimli::RunTimeEndian::Big
        // };
        // let empty_relocations = RelocMap::default();
        // let empty_section = gimli::RelocateReader::new(gimli::EndianSlice::new(&[], endian), &empty_relocations);
    
        // Create `Reader`s for all of the sections and do preliminary parsing.
        // Alternatively, we could have used `Dwarf::load` with an owned type such as `EndianRcSlice`.
        // let dwarf = dwarf_sections.borrow(|section| borrow_section(section, endian));

        // let dwp_sections = dwp_object
        //     .map(|dwp_object| {
        //         gimli::DwarfPackageSections::load(|id| load_section(dwp_object, id.dwo_name().unwrap()))
        //     })
        //     .transpose()?;

        // let old_load_section = |id: SectionId| -> Result<_, Error> {
        //     match obj_file.section_by_name(id.name()) {
        //         // DWO sections never have relocations, so don't bother.
        //         Some(ref section) => Ok(section.uncompressed_data()?),
        //         // Use a non-zero capacity so that `ReaderOffsetId`s are unique.
        //         None => Ok(Cow::Owned(Vec::with_capacity(1)))
        //     }
        // };

        Ok(Self {
            // dwarf: gimli::Dwarf::load(|_| load_section(SectionId::DebugInfo)).unwrap().debug_info,
            // dwarf: gimli::Dwarf::load(&load_section)?,
            // dwarf:  dwarf_sections.borrow(|section| borrow_section(section, endian))
            dwarf_sections: gimli::DwarfSections::load(|id| load_section(&obj_file, id.name()))?
        })
    }
 
    pub fn dump_deubg_info(&self, name: &str) -> Result<(), Box<dyn error::Error>> {
        // Create `Reader`s for all of the sections and do preliminary parsing.
        // Alternatively, we could have used `Dwarf::load` with an owned type such as `EndianRcSlice`.
        let dwarf = self.dwarf_sections.borrow(|section| borrow_section(section, gimli::RunTimeEndian::Little));
        let mut iter = dwarf.units();

        while let Some(header) = iter.next()? {
            println!(
                "Unit at <.debug_info+0x{:x}>",
                header.offset().as_debug_info_offset().unwrap().0
            );
            let unit = dwarf.unit(header)?;
            let unit_ref = unit.unit_ref(&dwarf);
            dump_unit(unit_ref, name)?;
        }

        // let process_unit = |header: UnitHeader<R>, buf: &mut Vec<u8>| -> Result<()> {
        //     dump_unit(buf, header, dwarf, dwo_parent_units, flags)?;
        //     if !flags
        //         .match_units
        //         .as_ref()
        //         .map(|r| r.is_match(buf))
        //         .unwrap_or(true)
        //     {
        //         buf.clear();
        //     }
        //     Ok(())
        // };
        Ok(())
    }
}

// Iterate over the Debugging Information Entries (DIEs) in the unit.
fn dump_unit(unit: gimli::UnitRef<CusReader>, name: &str) -> Result<(), gimli::Error> {
    let mut depth = 0;
    let mut entries = unit.entries();
    while let Some((delta_depth, entry)) = entries.next_dfs()? {
        depth += delta_depth;
        if entry.tag() == gimli::DW_TAG_member {
            // println!("<{}><{:06x}> {}", depth, entry.offset().0, entry.tag());
            let mut attrs = entry.attrs();
            let mut member = String::with_capacity(128);
            let mut is_match = false;
            while let Some(attr) = attrs.next()? {
                member += format!("  {}: {:?}", attr.name(), attr.value()).as_str();
                if let Ok(s) = unit.attr_string(attr.value()) {
                    let cow_str = s.to_string_lossy()?;
                    if cow_str == name {
                        // print!(" '{}'", cow_str);
                        // member += " ";
                        // member += cow_str.to_string();
                        member += format!(" {}", cow_str).as_str();
                        is_match = true;
                        // println!("  {}: {:?} {}", attr.name(), attr.value(), cow_str);
                    }
                }
                member += "\n";
                // println!();
            }
            if is_match {
                println!("{member}");
            }
        }
        // Iterate over the attributes in the DIE.
    }
    Ok(())
}