use std::io;

use log::error;
use nom::{
    bytes::complete::{is_not, tag, take_while},
    character::complete::{line_ending, not_line_ending},
    multi::fold_many1,
    sequence::terminated,
    IResult,
};

use crate::interval::{get_block_ivl, Block, BlockIvl};

#[derive(Debug, Clone, PartialEq)]
pub enum Strand {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
pub struct SeqInfo {
    pub name: String,
    pub size: usize,
    pub strand: Strand,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]

pub struct Header {
    score: f64,
    pub target: SeqInfo,
    pub query: SeqInfo,
    chain_id: usize,
}

#[derive(Debug)]
pub struct Alignment {
    pub size: usize,        //the size of the ungapped alignment
    pub target_diff: usize, //the difference between the end of this block and the beginning of the next block
    pub query_diff: usize,
}

#[derive(Debug)]
pub struct ChainRecord {
    pub header: Header,
    pub block_ivls: Vec<BlockIvl>,
}

pub struct ChainRecords<'a>(pub &'a str);

impl<'a> Iterator for ChainRecords<'a> {
    type Item = Result<ChainRecord, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        match chain_parser(self.0) {
            Ok((i, r)) => {
                self.0 = i;
                return Some(Ok(r));
            }
            Err(e) => {
                let mut msg = format!("{:?}", e);
                msg.push_str(self.0);
                return Some(Err(msg));
            }
        }
    }
}

fn parse_header(header_line: &str) -> Result<Header, io::Error> {
    // println!("parse header here");
    let header_vec: Vec<&str> = header_line.split_whitespace().collect();
    if header_vec.len() != 12 {
        error!("invalid header: {}", header_line);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid header"));
    }
    let score = header_vec[0].parse::<f64>().unwrap();
    let target = SeqInfo {
        name: header_vec[1].to_string(),
        size: header_vec[2].parse::<usize>().unwrap(),
        strand: if header_vec[3] == "+" {
            Strand::Positive
        } else {
            Strand::Negative
        },
        start: header_vec[4].parse::<usize>().unwrap(),
        end: header_vec[5].parse::<usize>().unwrap(),
    };
    let query = SeqInfo {
        name: header_vec[6].to_string(),
        size: header_vec[7].parse::<usize>().unwrap(),
        strand: if header_vec[8] == "+" {
            Strand::Positive
        } else {
            Strand::Negative
        },
        start: header_vec[9].parse::<usize>().unwrap(),
        end: header_vec[10].parse::<usize>().unwrap(),
    };
    let chain_id = header_vec[11].parse::<usize>().unwrap();
    let header = Header {
        score,
        target,
        query,
        chain_id,
    };
    Ok(header)
}

fn line_not_chain(i: &str) -> IResult<&str, &str> {
    terminated(is_not("chain\n"), line_ending)(i)
}

fn blocks(i: &str, header: Header) -> IResult<&str, Vec<BlockIvl>> {
    let target_name = &header.target.name;
    let target_start = header.target.start;
    let target_strand = &header.target.strand;
    let query_name = &header.query.name;
    let query_start = header.query.start;
    let query_strand = &header.query.strand;
    let mut target_current_cursor = target_start;
    let mut query_current_cursor = query_start;
    let query_size = header.query.size;
    let x = fold_many1(line_not_chain, Vec::new, |mut acc: Vec<_>, x| {
        let mut block_vec: Vec<&str> = x.split_whitespace().collect();
        block_vec.push("0");
        block_vec.push("0");
        let size = block_vec[0].parse::<usize>().unwrap();
        let target_diff = block_vec[1].parse::<usize>().unwrap();
        let query_diff = block_vec[2].parse::<usize>().unwrap();
        let alignment = Alignment {
            size,
            target_diff,
            query_diff,
        };

        let t2 = target_current_cursor;
        let t3 = target_current_cursor + alignment.size;
        let (q2, q3) = match query_strand {
            Strand::Positive => (
                (query_current_cursor),
                (query_current_cursor + alignment.size),
            ),
            Strand::Negative => (
                (query_size - (query_current_cursor + alignment.size)),
                (query_size - query_current_cursor),
            ),
        };
        target_current_cursor += alignment.size + alignment.target_diff;
        query_current_cursor += alignment.size + alignment.query_diff;
        // println!(
        //     "{}\t{}\t{}\t{:?}\t{}\t{}\t{}\t{:?}",
        //     target_name,
        //     t2,
        //     t3,
        //     target_strand.clone(),
        //     query_name,
        //     q2,
        //     q3,
        //     query_strand.clone()
        // );
        let block_target = Block {
            name: target_name.clone(),
            start: t2,
            end: t3,
            strand: target_strand.clone(),
        };
        let block_query = Block {
            name: query_name.clone(),
            start: q2,
            end: q3,
            strand: query_strand.clone(),
        };
        let block_ivl = get_block_ivl(&block_target, block_query);
        acc.push(block_ivl);
        acc
    })(i);
    x
}

pub fn chain_parser(input: &str) -> nom::IResult<&str, ChainRecord> {
    let (input, _) = tag("chain")(input)?;
    let (input, header_line) = not_line_ending(input)?;
    let header = parse_header(header_line).unwrap();
    let (input, _) = line_ending(input)?;
    let (input, blocks) = blocks(input, header.clone())?;
    let (input, _) = take_while(|x| x != 'c')(input)?; // should better
                                                       // let (_, title_vec) = tab_parser(title)?;
                                                       // let alignments = parse_blocks(blocks).unwrap();

    let chainrecord = ChainRecord {
        block_ivls: blocks,
        header: header.clone(),
    };
    // print_chain_record(&chainrecord);

    Ok((input, chainrecord))
}

// fn fa_start_tag(i: &str) -> IResult<&str, char> {
//     char('>')(i)
// }
// fn line_seq(i: &str) -> IResult<&str, &str> {
//     terminated(is_not(">\r\n"), line_ending)(i)
// }
// fn seq(i: &str) -> IResult<&str, String> {
//     fold_many1(line_seq, String::new, |mut acc: String, x| {
//         acc.push_str(x);
//         acc
//     })(i)
// }

// pub fn fasta_parse(i: &str) -> IResult<&str, FastaRecord> {
//     let (i, _) = fa_start_tag(i)?;
//     let (i, title) = not_line_ending(i)?;
//     let (i, _) = line_ending(i)?;
//     let (i, seq) = seq(i)?;
//     let (i, _) = take_while(|x| x != '>')(i)?;
//     let mut header_fields = title.trim_end().splitn(2, char::is_whitespace);
//     let id = header_fields.next().unwrap();
//     let desc = header_fields.next();
//     Ok((i, FastaRecord { id, desc, seq }))
// }

// #[derive(Default, Clone, Debug)]
// pub struct FastaRecord<'a> {
//     id: &'a str,
//     desc: Option<&'a str>,
//     seq: String,
// }

// pub struct FastaRecords<'a>(pub &'a str);

// impl<'a> Iterator for FastaRecords<'a> {
//     type Item = Result<FastaRecord<'a>, String>;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0.is_empty() {
//             return None;
//         }
//         match fasta_parse(self.0) {
//             Ok((i, r)) => {
//                 self.0 = i;
//                 return Some(Ok(r));
//             }
//             Err(e) => {
//                 let mut msg = format!("{:?}", e);
//                 msg.push_str(self.0);
//                 return Some(Err(msg));
//             }
//         }
//     }
// }
