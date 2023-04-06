use nom::{
    bytes::complete::{is_not, tag, take_while},
    character::complete::{char, line_ending, not_line_ending},
    multi::fold_many1,
    sequence::terminated,
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

pub fn chain_parser(input: &str) -> nom::IResult<&str, ChainRecord> {
    let (input, _) = tag("chain\t")(input)?;
    let (input, title) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, blocks) = blocks(input)?;
    // let blocks_str = blocks.join("\t");
    Ok((
        input,
        ChainRecord {
            header: title.to_string(),
            blocks,
        },
    ))
}

fn fa_start_tag(i: &str) -> IResult<&str, char> {
    char('>')(i)
}
fn line_seq(i: &str) -> IResult<&str, &str> {
    terminated(is_not(">\r\n"), line_ending)(i)
}
fn seq(i: &str) -> IResult<&str, String> {
    fold_many1(line_seq, String::new, |mut acc: String, x| {
        acc.push_str(x);
        acc
    })(i)
}

pub fn fasta_parse(i: &str) -> IResult<&str, FastaRecord> {
    let (i, _) = fa_start_tag(i)?;
    let (i, title) = not_line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, seq) = seq(i)?;
    let (i, _) = take_while(|x| x != '>')(i)?;
    let mut header_fields = title.trim_end().splitn(2, char::is_whitespace);
    let id = header_fields.next().unwrap();
    let desc = header_fields.next();
    Ok((i, FastaRecord { id, desc, seq }))
}

#[derive(Default, Clone, Debug)]
pub struct ChainRecord<'a> {
    header: String,
    blocks: Vec<&'a str>,
}

pub struct ChainRecords<'a>(pub &'a str);

impl<'a> Iterator for ChainRecords<'a> {
    type Item = Result<ChainRecord<'a>, String>;
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

#[derive(Default, Clone, Debug)]
pub struct FastaRecord<'a> {
    id: &'a str,
    desc: Option<&'a str>,
    seq: String,
}

pub struct FastaRecords<'a>(pub &'a str);

impl<'a> Iterator for FastaRecords<'a> {
    type Item = Result<FastaRecord<'a>, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        match fasta_parse(self.0) {
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
