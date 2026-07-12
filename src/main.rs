mod consts;
mod utils;
mod get_diary_five_per_week;
use consts::{MainData, ParsedData};

use crate::get_diary_five_per_week::get_diary_five_per_week;

#[cfg(test)]
#[macro_use] extern crate assertables;

fn main() {
    let main_data = {
        let res = std::fs::read_to_string("input.json").expect("Can't read file");
        let res = serde_json::from_str::<ParsedData>(&res).expect("Incorrect data in input.json");
        MainData::from(res)
    };
    let diary = get_diary_five_per_week(main_data.start_date, main_data.diary_length);

    println!("Diary: {:?}", diary);
}
