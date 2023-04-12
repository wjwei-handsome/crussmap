use log::{error, warn};
use std::{
    fs::{self, File},
    io::Error,
    io::{self, Read, Write},
    path::Path,
};

pub fn get_output_writer(output: &Option<String>, rewrite: bool) -> (Box<dyn Write>, bool) {
    let (output_file, stdout): (Box<dyn Write>, bool) = match output {
        Some(output_file) => {
            outfile_exist(output_file, rewrite);
            (Box::new(File::create(output_file).unwrap()), false)
        }
        None => (Box::new(io::stdout()), true),
    };
    (output_file, stdout)
}

pub fn get_file_reader(input_file: &String) -> Result<File, Error> {
    input_files_exist(input_file);
    fs::File::open(input_file)
}

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

pub fn get_data_from_input(input: &Option<String>) -> String {
    let data = match input {
        // input file
        Some(input_file) => {
            input_files_exist(input_file);
            read_file_to_string(input_file).unwrap()
        }
        // stdin
        None => {
            let mut data = String::with_capacity(512);
            std::io::stdin()
                .read_to_string(&mut data)
                .expect("failed to read from stdin");
            data
        }
    };
    data
}
