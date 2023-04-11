use std::{
    cmp::{max, min},
    collections::HashMap,
    io::Read,
};

use log::{error, info};
use rust_lapper::{Interval, Lapper};

use crate::{
    parser::{ChainRecord, ChainRecords, Strand},
    utils::{get_data_from_input, input_files_exist, read_file_to_string},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub strand: Strand,
}

impl Eq for Block {}

#[derive(Debug)]
pub struct Region<'a> {
    pub chrom: &'a String,
    pub start: usize,
    pub end: usize,
    pub strand: Strand,
}

pub type BlockIvl = Interval<usize, Block>;

pub fn get_block_ivl(block_target: &Block, block_query: Block) -> BlockIvl {
    BlockIvl {
        start: block_target.start,
        stop: block_target.end,
        val: block_query,
    }
}

fn get_lapper_hashmap(input: &Option<String>) -> HashMap<String, Lapper<usize, Block>> {
    let data = get_data_from_input(input);
    let chain_record_iter = ChainRecords(&data).into_iter();
    let mut chrom_ivls_hashmap: HashMap<String, Lapper<usize, Block>> = HashMap::new();
    for chain_record in chain_record_iter {
        let chain_record = chain_record.unwrap();
        let target_chrom = chain_record.header.target.name;
        let block_ivls = chain_record.block_ivls;
        let ivl_intersecter = Lapper::new(block_ivls);
        chrom_ivls_hashmap.insert(target_chrom, ivl_intersecter);
    }
    chrom_ivls_hashmap
}

fn intersect_two_region<'a>(
    region1: Region<'a>,
    region2: Region<'a>,
) -> Option<(&'a String, usize, usize)> {
    // it's ugly!
    let chr1 = region1.chrom;
    let chr2 = region2.chrom;
    let s1 = region1.start;
    let s2 = region2.start;
    let e1 = region1.end;
    let e2 = region2.end;
    if chr1 != chr2 {
        return None;
    }
    if s1 > e2 || s2 > e1 {
        return None;
    }
    if s1 > e1 || s2 > e2 {
        error!("sss");
        return None;
    }
    let final_start = max(s1, s2);
    let final_end = min(e1, e2);
    return Some((chr1, final_start, final_end));
}

pub fn test_bed_find(
    input: &Option<String>,
    chrom: &String,
    start: usize,
    end: usize,
    strand: &String,
) -> () {
    let lapper_hashmap = get_lapper_hashmap(input);
    let q_region = Region {
        chrom,
        start,
        end,
        strand: Strand::Positive,
    };
    find_in_lapper(&lapper_hashmap, q_region);
}

fn find_in_lapper(lapper_hashmap: &HashMap<String, Lapper<usize, Block>>, q_region: Region) -> () {
    let q_chrom = q_region.chrom;
    let lapper = lapper_hashmap.get(q_chrom).unwrap();
    info!("get chrom: {}", q_chrom);
    let targets = lapper
        .find(q_region.start, q_region.end)
        .collect::<Vec<&BlockIvl>>();
    info!("get targets");
    let matches = get_matches_from_targets(targets, &q_region);
    println!("{:?}", matches);
}

fn get_matches_from_targets<'a>(
    targets: Vec<&'a Interval<usize, Block>>,
    q_region: &'a Region<'a>,
) -> Vec<Region<'a>> {
    let mut matches: Vec<Region> = Vec::new();
    for target in targets {
        let source_start = target.start;
        let source_end = target.stop;
        let target_region = Region {
            chrom: &target.val.name,
            start: target.val.start,
            end: target.val.start,
            strand: target.val.strand,
        };
        let region1 = Region {
            chrom: q_region.chrom,
            start: q_region.start,
            end: q_region.end,
            strand: Strand::Positive,
        };
        let region2 = Region {
            chrom: q_region.chrom,
            start: source_start,
            end: source_end,
            strand: Strand::Positive,
        };
        let (real_chr, real_start, real_end) = intersect_two_region(region1, region2).unwrap();
        let l_offset = real_start.abs_diff(source_start);
        let size = real_end.abs_diff(real_start);
        matches.push(Region {
            chrom: real_chr,
            start: real_start,
            end: real_end,
            strand: q_region.strand,
        });
        let i_start = match target_region.strand {
            Strand::Positive => target_region.start + l_offset,
            Strand::Negative => target_region.end - l_offset - size,
        };
        let apdx_strand = match q_region.strand {
            Strand::Positive => target_region.strand,
            Strand::Negative => target_region.strand.reverse(),
        };
        matches.push(Region {
            chrom: &target_region.chrom,
            start: i_start,
            end: i_start + size,
            strand: apdx_strand,
        });
    }
    matches
}
