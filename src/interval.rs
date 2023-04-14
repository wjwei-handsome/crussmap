use crate::{
    parser::{ChainRecords, Strand},
    utils::get_data_from_input,
};
use log::{error, warn};
use rust_lapper::{Interval, Lapper};
use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt,
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

impl fmt::Display for Region<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.chrom, self.start, self.end, self.strand,
        )
    }
}

pub type BlockIvl = Interval<usize, Block>;

pub fn get_block_ivl(block_target: Block, block_query: Block) -> BlockIvl {
    BlockIvl {
        start: block_target.start,
        stop: block_target.end,
        val: block_query,
    }
}

pub fn get_lapper_hashmap(input: &Option<String>) -> HashMap<String, Lapper<usize, Block>> {
    let data = get_data_from_input(input);
    let chain_record_iter = ChainRecords(&data);
    let mut chrom_ivls_hashmap: HashMap<String, Lapper<usize, Block>> = HashMap::new();
    let mut chrom_ivls_vec_hashmap: HashMap<String, Vec<Interval<usize, Block>>> = HashMap::new();
    for chain_record in chain_record_iter {
        let chain_record = chain_record.unwrap();
        let target_chrom = chain_record.header.target.name;
        let block_ivls = chain_record.block_ivls;
        // combine interval vecs when target_chroms are same:
        if chrom_ivls_vec_hashmap.contains_key(&target_chrom) {
            let chrom_ivls_vec = chrom_ivls_vec_hashmap.get_mut(&target_chrom).unwrap();
            chrom_ivls_vec.extend(block_ivls);
        } else {
            chrom_ivls_vec_hashmap.insert(target_chrom.clone(), block_ivls);
        }
    }
    for (chrom, ivls) in chrom_ivls_vec_hashmap {
        let lapper = Lapper::new(ivls);
        chrom_ivls_hashmap.insert(chrom, lapper);
    }
    chrom_ivls_hashmap
}

fn intersect_two_region(
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
) -> Option<(usize, usize)> {
    if start1 > end2 || start2 > end1 {
        return None;
    }
    let final_start = max(start1, start2);
    let final_end = min(end1, end2);
    Some((final_start, final_end))
}

pub fn find_in_lapper<'a>(
    lapper_hashmap: &'a HashMap<String, Lapper<usize, Block>>,
    q_region: &Region<'a>,
) -> Vec<Region<'a>> {
    let lapper = match lapper_hashmap.get(q_region.chrom) {
        Some(lapper) => lapper,
        None => {
            warn!("chrom:{} not found in chain file", q_region.chrom);
            return Vec::new();
        }
    };
    // info!("get chrom: {} lapper: {:?}", q_chrom, lapper);
    let targets = lapper
        .find(q_region.start, q_region.end)
        .collect::<Vec<&BlockIvl>>();
    // info!("get targets: {:?}", targets);
    let mut matches: Vec<Region> = Vec::new();
    for target in targets {
        let target_region = Region {
            chrom: &target.val.name,
            start: target.val.start,
            end: target.val.start,
            strand: target.val.strand,
        };
        let (real_start, real_end) =
            match intersect_two_region(q_region.start, q_region.end, target.start, target.stop) {
                Some((start, end)) => (start, end),
                None => {
                    error!(
                        "intersect_two_region error in {}:{}{}",
                        q_region.chrom, q_region.start, q_region.end
                    );
                    continue;
                }
            };
        let l_offset = real_start.abs_diff(target.start);
        let size = real_end.abs_diff(real_start);
        matches.push(Region {
            chrom: q_region.chrom,
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
            chrom: target_region.chrom,
            start: i_start,
            end: i_start + size,
            strand: apdx_strand,
        });
    }
    matches
}
