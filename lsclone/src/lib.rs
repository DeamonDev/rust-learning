extern crate chrono;
extern crate libc;
#[macro_use]
extern crate structopt;

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;

use chrono::{DateTime, Local};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    pub path: PathBuf,
    #[structopt(short = "a", long = "all")]
    pub show_hidden_files: bool,
}

fn parse_permissions(mode: u32) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    [user, group, other].join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }
    .to_string()
}

pub fn run(dir: &PathBuf, show_hidden_files: bool) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            let is_hidden_file = file_name.starts_with(".");
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let mode = metadata.permissions().mode();

            if is_hidden_file && !show_hidden_files {
                continue;
            } else {
                println!(
                    "{} {:>5} {} {}",
                    parse_permissions(mode),
                    size,
                    modified.format("%_d %b %H:%M").to_string(),
                    file_name
                );
            }
        }
    }
    Ok(())
}
