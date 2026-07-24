mod consts;
mod get_diary_five_per_week;
mod get_diary_five_per_week_22_work;
mod get_diary_two_per_day;
mod get_diary_two_per_day_22_work;
mod utils;
mod utils_tests;

use utils::{get_diary_creator, get_main_data, write_config_file, write_diary};

#[cfg(test)]
#[macro_use] extern crate assertables;

fn main() {
    let main_data = get_main_data();
    let diary_creator = get_diary_creator(main_data.diary_type.clone());
    let diary = diary_creator(main_data.start_date.clone(), main_data.diary_length.clone());
    write_diary(diary);
    write_config_file(main_data);
}
