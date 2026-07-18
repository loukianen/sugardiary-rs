use crate::utils::{
  choose_two_from_amount,
  get_empty_record,
  mark_measurement
};
use crate::consts::*;
use chrono::{NaiveDate};

pub fn get_diary_two_per_day_22_work (start_date: NaiveDate, diary_length: u32) -> Vec<Record> {
  let mut cur_date = start_date;
  let mut diary = Vec::new();

  let mut index = 0;
  while index < diary_length {
    // record = [date, before breakfast, after breakfast, b.lunch, a.lunch, b.dinner, a.dinner]
    let mut record: Record = get_empty_record(cur_date);
    let day_in_work_cycle = index % DAYS_IN_WORK_CYCLE;
    let selected_indexes = match day_in_work_cycle {
      0 => choose_two_from_amount(&AVAILABLE_MEASUREMENTS_IN_WORK_DAY.to_vec()),
      1 => choose_two_from_amount(&NUMBER_OF_MEASUREMENTS_IN_NIGHT_WORK_DAY),
      _ => choose_two_from_amount(&NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY),
    };
    // day without limitation
    record = mark_measurement(record, selected_indexes.to_vec());
    diary.push(record);
    cur_date = cur_date.succ_opt().unwrap();
    index += 1;
  }
  diary
}

#[cfg(test)]

#[test]
fn test_get_diary_two_per_day_22_work() {
    let date= NaiveDate::from_ymd_opt(2026, 07, 05).unwrap();
    let len: u32 = 10;
    let diary: Vec<Record> = get_diary_two_per_day_22_work(date, len);
    let diary_len = diary.len() as u32;
    assert_eq!(diary_len, len);
    assert_eq!(diary[2].0, "07.07.2026");
    println!("Diary from test get_diary_two_per_day_22_work: {:?}", diary);
}
