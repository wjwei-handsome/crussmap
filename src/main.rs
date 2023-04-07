use crussmap::core::test;
use rust_lapper::{Interval, Lapper};

type Iv = Interval<usize, Vec<String>>;

fn main() {
    // let mut f = File::open("test/test.fa").unwrap();
    // let mut data = String::with_capacity(512);
    // f.read_to_string(&mut data).unwrap();
    // let a: Vec<_> = FastaRecords(&data).into_iter().collect();
    // println!("{:?}", a);

    // let mut f = File::open("test/test.chain").unwrap();
    // let mut data = String::with_capacity(512);
    // f.read_to_string(&mut data).unwrap();
    // let a: Vec<_> = ChainRecords(&data).into_iter().collect();
    // println!("{:?}", a);

    // let data: Vec<Iv> = vec![
    //     Iv {
    //         start: 1,
    //         stop: 10,
    //         val: vec!["chr1".to_string(), "a".to_string(), "b".to_string()],
    //     },
    //     Iv {
    //         start: 2,
    //         stop: 16,
    //         val: vec!["chr1".to_string(), "a".to_string(), "b".to_string()],
    //     },
    // ];

    // let mut lapper = Lapper::new(data);
    // let finded = lapper.find(11, 15).collect::<Vec<&Iv>>();
    // println!("{:?}", finded)

    test();
}

// a parser to get fields from \t split string using delimited
