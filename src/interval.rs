use std::{collections::HashMap, fs::File, io::Read};

use rust_lapper::{Interval, Lapper};

use crate::parser::{ChainRecord, ChainRecords, Strand};

pub fn fill_intersecter(
    a: Vec<Result<ChainRecord, String>>,
) -> HashMap<String, Lapper<usize, Query>> {
    let mut maps = HashMap::new();
    for item in a {
        let chain_record = item.unwrap();
        let (target_name, iv_vec) = parse_chain_record(chain_record);
        let mut lapper = Lapper::new(iv_vec);
        maps.insert(target_name, lapper);
    }
    maps
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    query_name: String,
    query_start: usize,
    query_end: usize,
    query_strand: Strand,
}

impl Eq for Query {}

fn parse_chain_record(chain_record: ChainRecord) -> (String, Vec<Interval<usize, Query>>) {
    let target_name = chain_record.header.target.name;
    let target_start = chain_record.header.target.start;
    let query_name = chain_record.header.query.name;
    let query_start = chain_record.header.query.start;
    let mut iv_vec = Vec::new();
    let mut target_current_cursor = target_start;
    let mut query_current_cursor = query_start;
    let query_size = chain_record.header.query.size;

    for block in chain_record.blocks.iter() {
        let query = match chain_record.header.query.strand {
            Strand::Positive => Query {
                query_name: query_name.clone(),
                query_start: query_current_cursor,
                query_end: query_current_cursor + block.size,
                query_strand: Strand::Positive,
            },
            Strand::Negative => Query {
                query_name: query_name.clone(),
                query_start: query_size - (query_current_cursor + block.size),
                query_end: query_size - query_current_cursor,
                query_strand: Strand::Negative,
            },
        };

        let iv = Interval {
            start: target_current_cursor,
            stop: target_current_cursor + block.size,
            val: query,
        };
        iv_vec.push(iv);
        target_current_cursor += block.size + block.target_diff;
        query_current_cursor += block.size + block.query_diff;
    }
    (target_name, iv_vec)
}
