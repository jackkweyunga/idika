use idika::generators;

fn main() {
    let len = 30;
    let count = 100000000;

    // let cuid = generators::cuid(len);
    // let snowflake = generators::snowflake();
    // println!("CUID: {}", cuid);
    // println!("Snowflake: {}", snowflake);

    let cuid = generators::n_cuid(count, len);
    // let snowflake = generators::n_snowflake(count);
    println!("CUIDs: {}", cuid.len());
    // println!("Snowflakes: {}", snowflake.len());

}
