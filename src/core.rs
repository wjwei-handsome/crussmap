use std::{collections::HashMap, fs::File, io::Read};

use rust_lapper::{Interval, Lapper};

use crate::interval::fill_intersecter;
use crate::parser::{ChainRecord, ChainRecords, Strand};

fn read_test() -> Vec<Result<ChainRecord, String>> {
    let mut f = File::open("test/test.chain").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = ChainRecords(&data).into_iter().collect();
    // println!("{:?}", a);
    a
}

pub fn test() {
    let a = read_test();
    let maps = fill_intersecter(a);
    println!("{:?}", maps)
}
