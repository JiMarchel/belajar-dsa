use searching::sequential_search::sequential_search;

mod basic_data_structure;
mod searching;
fn main() {
    let wk = [5, 8, 9];
    println!("{}", sequential_search(&wk, 5));
}
