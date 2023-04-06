use std::{fs::File, io::Read};

use crussmap::parser::ChainRecords;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    multi::fold_many1,
    sequence::{delimited, terminated},
    IResult,
};

fn main() {
    // let mut f = File::open("test/test.fa").unwrap();
    // let mut data = String::with_capacity(512);
    // f.read_to_string(&mut data).unwrap();
    // let a: Vec<_> = FastaRecords(&data).into_iter().collect();
    // println!("{:?}", a);
    let mut f = File::open("test/test.chain").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = ChainRecords(&data).into_iter().collect();
    println!("{:?}", a);
    // let tmp = "a\tb\tc";
    // let a = parser(tmp);
    // println!("{:?}", a);
}

// a parser to get fields from \t split string using delimited
