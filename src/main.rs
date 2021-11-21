extern crate chrono;
extern crate rexif;

use chrono::NaiveDateTime;
use glob::glob;
use rexif::ExifTag::DateTime as ExifDateTime;
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

fn main() -> Result<(), std::io::Error> {
    for entry in glob("*.jpg").unwrap() {
        match entry.unwrap().to_str() {
            // Get a file entry or just continue
            Some(f) => {
                // Get a NaiveDateTime or if not just continue to the next file
                let date = match get_date_from_photo(f) {
                    Some(date) => date,
                    None => continue,
                };

                let new_name = format!("./{}", date.format("%Y/%m_%B/%Y_%m_%d_%H_%M_%S.jpg"));
                let dirname = Path::new(&new_name).parent().unwrap();
                fs::create_dir_all(dirname)?;
                fs::rename(f, &new_name)?;
                eprintln!("moved file to {}", &new_name);
            }

            None => continue,
        }
    }

    Ok(())
}
