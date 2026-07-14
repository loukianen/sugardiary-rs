use crate::utils;
use crate::consts::*;
use chrono::{Datelike, NaiveDate};

pub fn get_diary_five_per_week_22_work (start_date: NaiveDate, diary_length: u16) -> Vec<Record> {
  let mut cur_date = start_date;
  let mut diary = Vec::new();
  let mut number_day_of_week = cur_date.weekday().number_from_monday();
  let mut days_without_measuring = utils::choose_two_days_number_of_week();
  
  let mut index = 0;
  while index < diary_length {
    // record = [date, before breakfast, after breakfast, b.lunch, a.lunch, b.dinner, a.dinner]
    let mut record: Record = utils::get_empty_record(cur_date);
    
    if !days_without_measuring.contains(&number_day_of_week) {
      let day_in_work_cycle = index % DAYS_IN_WORK_CYCLE;
      let selected_index = match day_in_work_cycle {
        0 => utils::choose_one_from_amount(&AVAILABLE_MEASUREMENTS_IN_WORK_DAY.to_vec()),
        1 => utils::choose_one_from_amount(&NUMBER_OF_MEASUREMENTS_IN_NIGHT_WORK_DAY),
        _ => utils::choose_one_from_amount(&NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY),
      };
      record = utils::mark_measurement(record, vec![selected_index]);
    }
    
    if number_day_of_week == DAYS_IN_WEEK {
      number_day_of_week = 1;
      days_without_measuring = utils::choose_two_days_number_of_week();
    } else {
      number_day_of_week += 1;
    }
    
    diary.push(record);
    cur_date = cur_date.succ_opt().unwrap();
    index += 1;
  }
  diary
}


#[cfg(test)]

#[test]
fn test_get_diary_five_per_week_22_work() {
    let date= NaiveDate::from_ymd_opt(2026, 07, 05).unwrap();
    let len: u16 = 20;
    let diary: Vec<Record> = get_diary_five_per_week(date, len);
    let diary_len = diary.len() as u16;
    assert_eq!(diary_len, len);
    assert_eq!(diary[2].0, "07.07.2026");
    println!("Diary from test get_diary_five_per_week_22_work: {:?}", diary);
}
