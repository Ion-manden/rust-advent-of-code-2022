mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod reader;

use day_1::day_1::{day_1_part_1, day_1_part_2};
use day_2::day_2::{day_2_part_1, day_2_part_2};
use day_3::day_3::{day_3_part_1, day_3_part_2};
use day_4::day_4::{day_4_part_1, day_4_part_2};
use day_5::day_5::{day_5_part_1, day_5_part_2};
use reader::reader::create_reader_from_file;

fn main() {
    println!(
        "Day 1 Part 1:{}",
        day_1_part_1(create_reader_from_file("./src/day_1/input.txt".to_owned()))
    );
    println!(
        "Day 1 Part 2:{}",
        day_1_part_2(create_reader_from_file("./src/day_1/input.txt".to_owned()))
    );
    println!(
        "Day 2 Part 1:{}",
        day_2_part_1(create_reader_from_file("./src/day_2/input.txt".to_owned()))
    );
    println!(
        "Day 2 Part 2:{}",
        day_2_part_2(create_reader_from_file("./src/day_2/input.txt".to_owned()))
    );
    println!(
        "Day 3 Part 1:{}",
        day_3_part_1(create_reader_from_file("./src/day_3/input.txt".to_owned()))
    );
    println!(
        "Day 3 Part 2:{}",
        day_3_part_2(create_reader_from_file("./src/day_3/input.txt".to_owned()))
    );
    println!(
        "Day 4 Part 1:{}",
        day_4_part_1(create_reader_from_file("./src/day_4/input.txt".to_owned()))
    );
    println!(
        "Day 4 Part 2:{}",
        day_4_part_2(create_reader_from_file("./src/day_4/input.txt".to_owned()))
    );
    println!(
        "Day 5 Part 1:{}",
        day_5_part_1(create_reader_from_file("./src/day_5/input.txt".to_owned()))
    );
    println!(
        "Day 5 Part 2:{}",
        day_5_part_2(create_reader_from_file("./src/day_5/input.txt".to_owned()))
    );
}
