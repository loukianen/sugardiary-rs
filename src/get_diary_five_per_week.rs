use crate::utils::{
  choose_one_from_amount,
  choose_two_from_amount,
  get_empty_record,
  mark_measurement
};
use crate::consts::{DAYS_IN_WEEK, NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY, Record};
use chrono::{Datelike, NaiveDate};

pub fn get_diary_five_per_week (start_date: NaiveDate, diary_length: u32) -> Vec<Record> {
  let mut cur_date = start_date;
  let mut diary = Vec::new();
  let mut number_day_of_week = cur_date.weekday().number_from_monday();
  let mut days_without_measuring = choose_two_from_amount(&DAYS_IN_WEEK);

  let mut index = 0;
  while index < diary_length {
    // record = [date, before breakfast, after breakfast, b.lunch, a.lunch, b.dinner, a.dinner]
    let mut record: Record = get_empty_record(cur_date);
    if !days_without_measuring.contains(&number_day_of_week) {
      // day without limitation
      let selected_index = choose_one_from_amount(&NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY);
      record = mark_measurement(record, vec![selected_index]);
    }
    diary.push(record);
    cur_date = cur_date.succ_opt().unwrap();
    
    if number_day_of_week == DAYS_IN_WEEK {
      number_day_of_week = 1;
      days_without_measuring = choose_two_from_amount(&DAYS_IN_WEEK);
    } else {
      number_day_of_week += 1;
    }
    index += 1;
  }
  diary
}

#[cfg(test)]

#[test]
fn test_get_diary_five_per_week() {
    let date= NaiveDate::from_ymd_opt(2026, 07, 05).unwrap();
    let len: u32 = 5;
    let diary: Vec<Record> = get_diary_five_per_week(date, len);
    let diary_len = diary.len() as u32;
    assert_eq!(diary_len, len);
    assert_eq!(diary[2].0, "07.07.2026");
    println!("Diary from test get_diary_five_per_week: {:?}", diary);
}
