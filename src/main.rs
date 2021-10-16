extern crate chrono;
extern crate rexif;

use chrono::NaiveDateTime;
use rexif::ExifTag::DateTime as ExifDateTime;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    match rexif::parse_file(&filename) {
        Ok(exif) => {
            for entry in &exif.entries {
                if entry.tag == ExifDateTime {
                    let dt = NaiveDateTime::parse_from_str(
                        &entry.value_more_readable,
                        "%Y:%m:%d %H:%M:%S",
                    )
                    .unwrap();

                    let new_name = format!("./{}", dt.format("%Y/%m_%B/%Y_%m_%d_%H_%M_%S.jpg"));
                    let dirname = Path::new(&new_name).parent().unwrap();
                    fs::create_dir_all(dirname).unwrap();
                    fs::rename(filename, new_name).unwrap();
                }
            }
        }
        Err(e) => {
            println!("Error: {}.", &e);
        }
    }
}
