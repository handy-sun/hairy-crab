#![allow(dead_code)]

use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

/// Process ID and its Attributes
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcAttr {
    pub pid: usize,
    pub cmdline: String,
    /// Processed `/proc/self/status` Name's value
    status_name: String,
}

impl ProcAttr {
    pub fn try_new(value: PathBuf) -> Result<Self, io::Error> {
        let path_value = if value.is_symlink() {
            fs::read_link(value)?
        } else {
            value
        };

        let pid = path_value
            .iter()
            .last()
            .ok_or(io::ErrorKind::Other)?
            .to_str()
            .ok_or(io::ErrorKind::InvalidData)?
            .parse::<usize>()
            .map_err(|_| io::ErrorKind::InvalidData)?;

        let cmdline = fs::read_to_string(path_value.join("cmdline"))?
            .replace('\0', " ")
            .trim_end()
            .into();

        let status_file = fs::File::open(path_value.join("status"))?;
        // .map_err(|err| anyhow!("Problem open file {:?}: {}", proc_maps, err))?;
        let status_reader = io::BufReader::new(status_file);
        let first_line = status_reader.lines().next().unwrap()?;
        let status_name = String::from(first_line.split_once(':').unwrap_or_default().1.trim_ascii());

        Ok(Self {
            pid,
            cmdline,
            status_name,
        })
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
        .filter(move |it| it.path().is_dir())
        .flat_map(ProcAttr::try_from)
}

pub fn collect_matched_pids(program_name: &str) -> impl Iterator<Item = ProcAttr> + '_ {
    let iter = walk_process();
    iter.filter(move |attr| !attr.cmdline.is_empty() && attr.status_name == program_name)
}

fn get_executable_name(process: &mut ProcAttr) -> String {
    let binding = process.cmdline.split(' ').collect::<Vec<_>>();
    let mut path = binding.first().unwrap().to_string();

    if path.is_empty() {
        path.clone_from(&process.status_name);
    };

    PathBuf::from(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn join_result_pids(matches: &str) -> String {
    let mut collected: Vec<_> = collect_matched_pids(matches).collect();

    if collected.is_empty() {
        return "".to_string();
    };

    if collected.len() == 1 {
        return collected[0].pid.to_string();
    }

    collected.sort_by(|a, b| b.pid.cmp(&a.pid));
    collected
        .into_iter()
        .map(|it| it.pid.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
