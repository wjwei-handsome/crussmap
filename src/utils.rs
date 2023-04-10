use log::{error, warn};
use std::{fs::File, io::Read, path::Path};

pub fn read_file_to_string(file_path: &String) -> Result<String, String> {
    let mut f = File::open(file_path).map_err(|e| e.to_string())?;
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).map_err(|e| e.to_string())?;
    Ok(data)
}

pub fn input_files_exist(input_file: &String) -> () {
    // check if input files exist
    let path = Path::new(input_file);
    if !path.exists() {
        error!("file {} does not exist", input_file);
        std::process::exit(1);
    }
}

pub fn outfile_exist(outputname: &String, rewrite: bool) -> () {
    // check if output file exists
    let path = Path::new(outputname);
    if path.exists() {
        if rewrite {
            // rewrite the file
            warn!("file {} exist, will rewrite it", outputname);
        } else {
            // exit
            error!("file {} exist, use -r to rewrite it", outputname);
            std::process::exit(1);
        }
    }
}