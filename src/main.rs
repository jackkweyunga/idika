use idika::generators;

fn main() {
    let len = 30;
    let count = 100000000;

    // let cuid = generators::cuid(len);
    // println!("CUID: {}", cuid);

    let cuid = generators::n_cuid(count, len);
    println!("CUIDs: {}", cuid.len());

}
