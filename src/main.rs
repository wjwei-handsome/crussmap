use std::{fs::File, io::Read};

use crussmap::parser::ChainRecords;

fn main() {
    let mut f = File::open("test/test.chain").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = ChainRecords(&data).into_iter().collect();
    println!("{:?}", a);
}
