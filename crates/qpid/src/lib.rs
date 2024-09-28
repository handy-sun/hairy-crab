
// #![allow(unknown_lints)]
// #![allow(unused_imports)]
#![allow(dead_code)]
// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
// use std::borrow::{self, Cow};
// use std::error;
// use std::hash::Hash;
use std::{
    collections::HashMap,
    // fmt::{self, Display, Formatter},
    fs, io,
    path::PathBuf,
    rc::Rc,
};
use walkdir::{DirEntry, WalkDir};

// use clap::{crate_version, Arg, ArgAction, ArgMatches, Command};
// use uu_pgrep::process::{walk_process, ProcAttr};
// use uucore::{error::UResult, format_usage, help_about, help_usage};


/// Process ID and its Attributes
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcAttr {
    pub pid: usize,
    pub cmdline: String,

    inner_status: String,
    inner_stat: String,

    /// Processed `/proc/self/status` file
    cached_status: Option<Rc<HashMap<String, String>>>,
    /// Processed `/proc/self/stat` file
    cached_stat: Option<Rc<Vec<String>>>,

    cached_start_time: Option<u64>,
}

impl ProcAttr {
    pub fn try_new(value: PathBuf) -> Result<Self, io::Error> {
        let dir_append = |mut path: PathBuf, s: &str| {
            path.push(s);
            path
        };

        let path_value = if value.is_symlink() {
            fs::read_link(value)?
        } else {
            value
        };

        let pid = {
            path_value
                .iter()
                .last()
                .ok_or(io::ErrorKind::Other)?
                .to_str()
                .ok_or(io::ErrorKind::InvalidData)?
                .parse::<usize>()
                .map_err(|_| io::ErrorKind::InvalidData)?
        };
        let cmdline = fs::read_to_string(dir_append(path_value.clone(), "cmdline".into()))?
            .replace('\0', " ")
            .trim_end()
            .into();

        Ok(Self {
            pid,
            cmdline,
            inner_status: fs::read_to_string(dir_append(path_value.clone(), "status"))?,
            inner_stat: fs::read_to_string(dir_append(path_value, "stat"))?,
            ..Default::default()
        })
    }

    /// Collect information from `/proc/<pid>/status` file
    pub fn status(&mut self) -> Rc<HashMap<String, String>> {
        if let Some(c) = &self.cached_status {
            return Rc::clone(c);
        }

        let result = self
            .inner_status
            .lines()
            .filter_map(|it| it.split_once(':'))
            .map(|it| (it.0.to_string(), it.1.trim_start().to_string()))
            .collect::<HashMap<_, _>>();

        let result = Rc::new(result);
        self.cached_status = Some(Rc::clone(&result));
        Rc::clone(&result)
    }
}

impl TryFrom<DirEntry> for ProcAttr {
    type Error = io::Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let value = value.into_path();

        Self::try_new(value)
    }
}

/// Iterating pid in current system
fn walk_process() -> impl Iterator<Item = ProcAttr> {
    WalkDir::new("/proc/")
        .max_depth(1)
        .follow_links(false)
        .into_iter()
        .flatten()
        .filter(|it| it.path().is_dir() )
        .flat_map(ProcAttr::try_from)
}

pub fn collect_matched_pids(program_name: &str) -> Vec<ProcAttr> {

    let collected = walk_process().collect::<Vec<_>>();

    // let arg_omit_pid = matches
    //     .get_many::<usize>("o")
    //     .unwrap_or_default()
    //     .copied()
    //     .collect::<Vec<_>>();

    // let filter = |program_name| {
            let mut processed = Vec::new();
            for mut process in collected.clone() {
                let contains = program_name == get_executable_name(&mut process);

                if contains 
 
                {
                    processed.push(process);
                }
            }

            processed.sort_by(|a, b| b.pid.cmp(&a.pid));

            // let flag_s = matches.get_flag("s");
            // if flag_s {
            //     match processed.first() {
            //         Some(first) => vec![first.clone()],
            //         None => Vec::new(),
            //     }
            // } else {
                processed
            // }
        // };
        // .collect()
}

fn get_executable_name(process: &mut ProcAttr) -> String {
    let binding = process.cmdline.split(' ').collect::<Vec<_>>();
    let mut path = binding.first().unwrap().to_string();

    if path.is_empty() {
        path.clone_from(&process.status()["Name"]);
    };

    PathBuf::from(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}


pub fn select_single(matches: &str) -> String {
    let collected = collect_matched_pids(matches);

    if collected.is_empty() {
        // uucore::error::set_exit_code(1);
        return "".to_string();
    };

    let output = collected
        .into_iter()
        .map(|it| it.pid.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    output

    // let flag_quiet = matches.get_flag("q");
    // if !flag_quiet {
    //     println!("{output}");
    // }

    // Ok(())
}
