use std::{
    fmt::Error,
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
    path::Path,
};

use derivative::*;

#[derive(Derivative)]
#[derivative(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Entry {
    pub filename: String,
    pub is_dir: bool,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub metadata: Metadata,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub entries: Option<Vec<Entry>>,
}

pub fn get_file_permission_string(file: &Entry) -> String {
    let per = file.metadata.permissions().mode() & 0b111111111; // filter only 3x3

    let per_string = "rwxrwxrwx";
    let mut output = String::from("");

    for (i, c) in per_string.chars().enumerate() {
        let bit_value = (per >> (per_string.len() - i - 1)) & 1;
        let output_char = if bit_value == 1 { c } else { '-' };
        output.push(output_char);
    }

    output
}

pub fn read_dir(path: &Path, depth: Option<usize>) -> Result<Vec<Entry>, Error> {
    let paths = fs::read_dir(path);

    let mut return_vec = Vec::<Entry>::new();

    for file in paths.unwrap() {
        let fi = file.unwrap();
        let filename = fi.file_name().into_string().unwrap();
        let is_dir = fi.file_type().unwrap().is_dir();
        let metadata = fi.metadata().unwrap();
        let mut entries = None;

        if is_dir {
            if depth.is_some() {
                if depth > Some(0) {
                    entries = read_dir(&fi.path(), Some(depth.unwrap() - 1)).ok();
                }
            }
        }

        let new_entry = Entry {
            is_dir,
            filename,
            metadata,
            entries,
        };
        return_vec.push(new_entry)
    }

    Ok(return_vec)
}
