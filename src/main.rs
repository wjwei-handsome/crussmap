use crussmap::core::{read_test, test_print_table, tmp_test};

fn main() {
    // tmp_test();
    // test_print_table();
    let a = read_test();
    println!("{:?}", a)
}

// a parser to get fields from \t split string using delimited
