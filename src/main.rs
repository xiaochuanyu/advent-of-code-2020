mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod util;

use std::env;


#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    match query.as_str() {
        "1-1" => day_1::part_1(),
        "1-2" => day_1::part_2(),
        "2-1" => day_2::part_1(),
        "2-2" => day_2::part_2(),
        "3-1" => day_3::part_1(),
        "3-2" => day_3::part_2(),
        "4-1" => day_4::part_1(),
        "4-2" => day_4::part_2(),
        "5-1" => day_5::part_1(),
        "5-2" => day_5::part_2(),
        "6-1" => day_6::part_1(),
        "6-2" => day_6::part_2(),
        "7-1" => day_7::part_1(),
        "7-2" => day_7::part_2(),
        _ => println!("invalid command: enter day#-part# e.g. 6-1"),
    }
}
