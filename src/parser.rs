use std::io;

use nom::{
    bytes::complete::{is_not, tag, take_while},
    character::complete::{char, line_ending, multispace0, not_line_ending},
    multi::fold_many1,
    sequence::{delimited, terminated},
    IResult,
};

fn line_not_chain(i: &str) -> IResult<&str, &str> {
    terminated(is_not("chain\n"), line_ending)(i)
}

fn blocks(i: &str) -> IResult<&str, Vec<&str>> {
    fold_many1(line_not_chain, Vec::new, |mut acc: Vec<_>, x| {
        acc.push(x);
        acc
    })(i)
}

fn tab_parser(i: &str) -> IResult<&str, Vec<&str>> {
    fold_many1(
        delimited(multispace0, is_not("\t"), multispace0),
        Vec::new,
        |mut acc: Vec<_>, x| {
            acc.push(x);
            acc
        },
    )(i)
}

#[derive(Debug)]

enum Strand {
    Positive,
    Negative,
}

#[derive(Debug)]

struct SeqInfo {
    name: String,
    size: usize,
    strand: Strand,
    start: usize,
    end: usize,
}

#[derive(Debug)]

struct Header {
    score: f64,
    target: SeqInfo,
    query: SeqInfo,
    chain_id: usize,
}

#[derive(Debug)]
struct Alignment {
    size: usize,        //the size of the ungapped alignment
    target_diff: usize, //the difference between the end of this block and the beginning of the next block
    query_diff: usize,
}

#[derive(Debug)]
pub struct ChainRecord {
    header: Header,
    blocks: Vec<Alignment>,
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
    let header_vec: Vec<&str> = header_line.split_whitespace().collect();
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
    let source = SeqInfo {
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
        query: source,
        chain_id,
    };
    Ok(header)
}

fn parse_blocks(blocks: Vec<&str>) -> Result<Vec<Alignment>, io::Error> {
    let mut alignments = Vec::new();
    for block in blocks {
        let block_vec: Vec<&str> = block.split_whitespace().collect();
        let size = block_vec[0].parse::<usize>().unwrap();
        let target_diff = block_vec[1].parse::<usize>().unwrap();
        let query_diff = block_vec[2].parse::<usize>().unwrap();
        let alignment = Alignment {
            size,
            target_diff,
            query_diff,
        };
        alignments.push(alignment);
    }
    Ok(alignments)
}
pub fn chain_parser(input: &str) -> nom::IResult<&str, ChainRecord> {
    let (input, _) = tag("chain\t")(input)?;
    let (input, header_line) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, blocks) = blocks(input)?;
    let (input, _) = take_while(|x| x != 'c')(input)?; // should better
                                                       // let (_, title_vec) = tab_parser(title)?;
    let header = parse_header(header_line).unwrap();
    let alignments = parse_blocks(blocks).unwrap();
    Ok((
        input,
        ChainRecord {
            header,
            blocks: alignments,
        },
    ))
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
