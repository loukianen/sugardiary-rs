use crate::utils::{
  choose_two_from_amount,
  get_empty_record,
  mark_measurement
};
use crate::consts::{NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY, Record};
use chrono::NaiveDate;

pub fn get_diary_two_per_day (start_date: NaiveDate, diary_length: u16) -> Vec<Record> {
  let mut cur_date = start_date;
  let mut diary = Vec::new();

  let mut index = 0;
  while index < diary_length {
    // record = [date, before breakfast, after breakfast, b.lunch, a.lunch, b.dinner, a.dinner]
    let mut record: Record = get_empty_record(cur_date);
    // day without limitation
    let selected_indexes = choose_two_from_amount(&NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY);
    record = mark_measurement(record, selected_indexes.to_vec());
    diary.push(record);
    cur_date = cur_date.succ_opt().unwrap();
    index += 1;
  }
  diary
}

#[cfg(test)]

#[test]
fn test_get_diary_two_per_day() {
    let date= NaiveDate::from_ymd_opt(2026, 07, 05).unwrap();
    let len: u16 = 10;
    let diary: Vec<Record> = get_diary_two_per_day(date, len);
    let diary_len = diary.len() as u16;
    assert_eq!(diary_len, len);
    assert_eq!(diary[2].0, "07.07.2026");
    println!("Diary from test get_diary_five_per_week: {:?}", diary);
}
