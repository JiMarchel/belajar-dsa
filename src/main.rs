use searching::interpolation_search::interpolation_search;

mod basic_data_structure;
mod searching;
fn main() {
    let wk = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
    println!("{}", interpolation_search(&wk, 30));
}
