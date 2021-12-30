extern crate chrono;
extern crate rexif;

use anyhow::Result;
use chrono::NaiveDateTime;
use getopts::Options;
use glob::glob;
use rexif::ExifTag::DateTime as ExifDateTime;
use std::env;
use std::fs;
use std::path::Path;

/// Return an option on NaiveDateTime, if for whatever reason this fails
/// we want to just return None and carry on to the next file
fn get_date_from_photo(filename: &str) -> Option<NaiveDateTime> {
    let exif = rexif::parse_file(&filename).ok()?;

    for entry in &exif.entries {
        if entry.tag == ExifDateTime {
            // As soon as we find ExifDateTime, just return immediately
            return NaiveDateTime::parse_from_str(&entry.value_more_readable, "%Y:%m:%d %H:%M:%S")
                .ok();
        }
    }

    None
}

fn sort_photos(create_dirs: bool) -> Result<()> {
    for entry in glob("*.[jJ][pP]*[gG]").unwrap() {
        match entry.unwrap().to_str() {
            // Get a file entry or just continue
            Some(f) => {
                // Get a NaiveDateTime or if not just continue to the next file
                let date = match get_date_from_photo(f) {
                    Some(date) => date,
                    None => continue,
                };

                let fmt_string = if create_dirs {
                    "%Y/%m_%B/%Y_%m_%d_%H_%M_%S.jpg"
                } else {
                    "%Y_%m_%d_%H_%M_%S.jpg"
                };

                let new_name = format!("./{}", date.format(fmt_string));
                let dirname = Path::new(&new_name).parent().unwrap();

                if create_dirs {
                    fs::create_dir_all(dirname)?;
                }

                fs::rename(f, &new_name)?;
                eprintln!("moved file to {}", &new_name);
            }

            None => continue,
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("d", "dirs", "create directories");
    opts.optflag("h", "help", "show these options");
    let matches = opts.parse(&args[1..])?;
    let help = matches.opt_present("h");
    let dirs = matches.opt_present("d");

    if help {
        let brief = format!("Usage: {} [options]", program_name);
        print!("{}", opts.usage(&brief));
    } else {
        sort_photos(dirs)?;
    }

    Ok(())
}
