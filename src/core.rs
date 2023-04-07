use std::{fs::File, io::Read};

use crate::interval::fill_intersecter;
use crate::parser::{ChainRecord, ChainRecords, Strand};

pub fn read_test() -> Vec<Result<ChainRecord, String>> {
    let mut f = File::open("test/test.chain").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = ChainRecords(&data).into_iter().collect();
    // println!("{:?}", a);
    a
}

pub fn tmp_test() {
    let a = read_test();
    let maps = fill_intersecter(a);
    println!("{:?}", maps)
}

fn print_chain_record(chain_record: ChainRecord) {
    let target_name = chain_record.header.target.name;
    let target_start = chain_record.header.target.start;
    let target_strand = chain_record.header.target.strand;
    let query_name = chain_record.header.query.name;
    let query_start = chain_record.header.query.start;
    let query_strand = chain_record.header.query.strand;
    let mut target_current_cursor = target_start;
    let mut query_current_cursor = query_start;
    let query_size = chain_record.header.query.size;

    for block in chain_record.blocks.iter() {
        // let t1 = target_name;
        let t2 = target_current_cursor;
        let t3 = target_current_cursor + block.size;
        // let t4 = target_strand;
        // let q1 = query_name;
        // let q2 = match query_strand {
        //     Strand::Positive => query_current_cursor,
        //     Strand::Negative => query_size - (query_current_cursor + block.size),
        // };
        // let q3 = match query_strand {
        //     Strand::Positive => query_current_cursor + block.size,
        //     Strand::Negative => query_size - query_current_cursor,
        // };
        let (q2, q3) = match query_strand {
            Strand::Positive => ((query_current_cursor), (query_current_cursor + block.size)),
            Strand::Negative => (
                (query_size - (query_current_cursor + block.size)),
                (query_size - query_current_cursor),
            ),
        };
        // let q4 = query_strand;
        println!(
            "{}\t{}\t{}\t{:?}\t{}\t{}\t{}\t{:?}",
            target_name,
            t2,
            t3,
            target_strand.clone(),
            query_name,
            q2,
            q3,
            query_strand.clone()
        );
        target_current_cursor += block.size + block.target_diff;
        query_current_cursor += block.size + block.query_diff;
    }
}

fn print_table(a: Vec<Result<ChainRecord, String>>) {
    for item in a {
        let chain_record = item.unwrap();
        print_chain_record(chain_record);
    }
}

pub fn test_print_table() {
    let a = read_test();
    print_table(a);
}
