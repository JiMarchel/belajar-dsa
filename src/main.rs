mod basic_dsa;
use basic_dsa::par_checker3::par_checker3;

fn main() {
    let sa = "(2+3){func}[array]";
    let sb = "(1+2)(2-3";
    let res1 = par_checker3(sa);
    let res2 = par_checker3(sb);
    println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
}
