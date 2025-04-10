use chapter4::queue::{hot_potato::hot_potato, queue::Queue};

mod chapter4;

fn main() {
    let names = vec!["Mon", "Tom", "Kew", "Lisa", "Marry", "bob"];
    let survivor = hot_potato(names, 8);
    println!("The survival person is {survivor}");
}
