extern crate chrono;
extern crate rexif;

use chrono::NaiveDateTime;
use chrono::ParseError;
use rexif::ExifError;
use rexif::ExifTag::DateTime as ExifDateTime;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Error {
    Exif(ExifError),
    Parse(ParseError),
    Io(std::io::Error)
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    let exif = rexif::parse_file(&filename).map_err(Error::Exif)?;

    for entry in &exif.entries {
        if entry.tag == ExifDateTime {
            let dt = NaiveDateTime::parse_from_str(
                &entry.value_more_readable,
                "%Y:%m:%d %H:%M:%S",
            ).map_err(Error::Parse)?;

            let new_name = format!("./{}", dt.format("%Y/%m_%B/%Y_%m_%d_%H_%M_%S.jpg"));
            let dirname = Path::new(&new_name).parent().unwrap();
            fs::create_dir_all(dirname).map_err(Error::Io)?;
            fs::rename(filename, &new_name).map_err(Error::Io)?;
            eprintln!("moved file to {}", &new_name);

            break;
        }
    
    }

    Ok(())
}
