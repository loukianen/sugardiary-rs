mod consts;
mod utils;
mod get_diary_five_per_week;
mod get_diary_five_per_week_22_work;


#[cfg(test)]
#[macro_use] extern crate assertables;

fn main() {
    let main_data = utils::get_main_data();
    let diary_creator = utils::get_diary_creator(main_data.diary_type);
    let diary = diary_creator(main_data.start_date, main_data.diary_length);
    utils::write_diary(diary);
}
