use std::{
    fs::File,
    io::{self, Read, Write},
};

use crate::{
    parser::{ChainRecords, Strand},
    utils::{input_files_exist, outfile_exist, read_file_to_string},
};

pub fn view_chain(input: &Option<String>, output: &Option<String>, csv: bool, rewrite: bool) -> () {
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
    let mut output_file: Box<dyn Write> = match output {
        Some(output_file) => {
            outfile_exist(output_file, rewrite);
            Box::new(File::create(output_file).unwrap())
        }
        None => Box::new(io::stdout()),
    };
    let chain_record_vecs: Vec<_> = ChainRecords(&data).into_iter().collect();

    if csv {
        output_file
            .write_all(b"target_name,target_start,target_end,target_strand,query_name,query_start,query_end,query_strand\n")
            .unwrap();
        for chain_record in chain_record_vecs {
            let chain_record = chain_record.unwrap();
            let target_chrom = chain_record.header.target.name;
            for block in chain_record.block_ivls {
                let query_strand = match block.val.strand {
                    Strand::Positive => "+",
                    Strand::Negative => "-",
                };
                let line = format!(
                    "{},{},{},{},{},{},{},{}\n",
                    target_chrom,
                    block.start,
                    block.stop,
                    '+',
                    block.val.name,
                    block.val.start,
                    block.val.end,
                    query_strand
                );
                output_file.write_all(line.as_bytes()).unwrap();
            }
        }
    } else {
        output_file
                .write_all(b"target_name\ttarget_start\ttarget_end\ttarget_strand\tquery_name\tquery_start\tquery_end\tquery_strand\n")
                .unwrap();
        for chain_record in chain_record_vecs {
            let chain_record = chain_record.unwrap();
            let target_chrom = chain_record.header.target.name;
            for block in chain_record.block_ivls {
                let query_strand = match block.val.strand {
                    Strand::Positive => "+",
                    Strand::Negative => "-",
                };
                let line = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    target_chrom,
                    block.start,
                    block.stop,
                    '+',
                    block.val.name,
                    block.val.start,
                    block.val.end,
                    query_strand
                );
                output_file.write_all(line.as_bytes()).unwrap();
            }
        }
    }
}
