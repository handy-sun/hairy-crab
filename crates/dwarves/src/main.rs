mod dw_info;

use getopts::Options;
use std::env::*;
use std::fs;
use std::io::{self, Write};

// use object::{Object, ObjectSection};
use goblin::container::{Container, Ctx};
use goblin::elf::*;
// use std::ffi::CStr;

use std::io::{Read, Seek, SeekFrom};

#[allow(dead_code)]
const ELF64_HDR_SIZE: usize = 64;

#[allow(dead_code)]
fn parse_bin(file_path: &str) -> Result<(), &'static str> {
    let mut file = fs::File::open(file_path).map_err(|_| "open file error")?;
    let file_len = file.metadata().map_err(|_| "get metadata error")?.len();

    // init the content vec
    let mut contents = vec![0; file_len as usize];

    // read in header only
    file.read_exact(&mut contents[..ELF64_HDR_SIZE])
        .map_err(|_| "read header error")?;

    // parse header
    let header = Elf::parse_header(&contents).map_err(|_| "parse elf header error")?;
    if header.e_phnum == 0 {
        return Err("ELF doesn't have any program segments");
    }

    // read in program header table
    let program_hdr_table_size = header.e_phnum * header.e_phentsize;
    file.seek(SeekFrom::Start(header.e_phoff))
        .map_err(|_| "seek error")?;
    file.read_exact(
        &mut contents[ELF64_HDR_SIZE..ELF64_HDR_SIZE + (program_hdr_table_size as usize)],
    )
    .map_err(|_| "read program header table error")?;

    // dummy Elf with only header
    // let mut elf = Elf::lazy_parse(header).map_err(|_| "cannot parse ELF file")?;

    let ctx = Ctx {
        le: scroll::Endian::Little,
        container: Container::Big,
    };

    // parse and assemble the program headers
    // elf.program_headers = ProgramHeader::parse(
    let sec_headers = SectionHeader::parse(
        &contents,
        header.e_phoff as usize,
        header.e_phnum as usize,
        ctx,
    )
    .map_err(|_| "parse section headers error")?;

    // let mut intepreter_count = 0;
    // let mut intepreter_offset = 0;
    for sh in sec_headers {
        println!("{:?}", sh);
    }

    // assemble the interpreter
    // elf.interpreter = if intepreter_count == 0 {
    //     None
    // } else {
    //     let cstr: &CStr = CStr::from_bytes_with_nul(
    //         &contents[intepreter_offset..intepreter_offset + intepreter_count],
    //     )
    //     .map_err(|_| "invalid interpreter path")?;
    //     cstr.to_str().ok()
    // };

    Ok(())
}

fn print_usage(opts: &Options) -> ! {
    let brief = format!("Usage: {} <options> <file>", args().next().unwrap());
    write!(&mut io::stderr(), "{}", opts.usage(&brief)).ok();
    std::process::exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut opts = Options::new();

    opts.optflag("i", "", "print .debug_info and .debug_types sections");
    opts.optflag("l", "", "print .debug_line section");
    opts.optflag("p", "", "print .debug_pubnames section");
    opts.optflag("r", "", "print .debug_aranges section");
    opts.optflag(
        "",
        "dwp",
        "print the .dwp versions of the selected sections",
    );

    #[cfg(debug_assertions)]
    let arg_str_vec = if cfg!(debug_assertions) {
        // vec![]
        vec!["-l", "-i", "/usr/local/bin/ntf"]
    } else {
        vec!["", "-i", "/usr/local/bin/ntf", "-l"]
    };

    let matches = match opts.parse(arg_str_vec) {
        Ok(m) => m,
        Err(e) => {
            writeln!(&mut io::stderr(), "{:?}\n", e).ok();
            print_usage(&opts);
        }
    };
    // if matches.opt_present("l") {
    //     writeln!(&mut io::stdout(), "short: l").ok();
    // }
    // if matches.opt_present("i") {
    //     writeln!(&mut io::stdout(), "{:?}", matches.free).ok();
    // }

    if matches.free.len() != 1 {
        writeln!(&mut io::stderr(), "err len: {:?}", matches.free.len()).ok();
        std::process::exit(1);
    }

    let file_path = matches.free.first().unwrap();
    let bin_bytes = fs::read(file_path)?;
    let _dw_mgr = dw_info::DwarfMgr::parse(bin_bytes.as_ref())?;
    _dw_mgr.dump_deubg_info("StateList")
    // io::stdout().write_fmt(format_args!("{:?}\n", _dw_mgr)).ok();
    // parse_bin(file_path.as_str()).ok();

    // Ok(())
}
