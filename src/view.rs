use crate::{
    parser::{ChainRecords, Strand},
    utils::{get_data_from_input, get_output_writer},
};
use std::io::Write;

pub fn view_chain(input: &Option<String>, output: &Option<String>, csv: bool, rewrite: bool) {
    let data = get_data_from_input(input);
    // let mut output_file: Box<dyn Write> = match output {
    //     Some(output_file) => {
    //         outfile_exist(output_file, rewrite);
    //         Box::new(File::create(output_file).unwrap())
    //     }
    //     None => Box::new(io::stdout()),
    // };
    let (mut output_file, _) = get_output_writer(output, rewrite);
    // info!("start parse");
    let chain_record_iter = ChainRecords(&data);
    // info!("get iteror");

    if csv {
        output_file
            .write_all(b"target_name,target_start,target_end,target_strand,query_name,query_start,query_end,query_strand\n")
            .unwrap();
        for chain_record in chain_record_iter {
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
        for chain_record in chain_record_iter {
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
    // info!("write done");
}
