use chrono::NaiveDate;
use crate::get_diary_five_per_week::get_diary_five_per_week;
use crate::get_diary_five_per_week_22_work::get_diary_five_per_week_22_work;
use crate::consts::{DiaryCreator, DiaryTypes, MARK, MainData, ParsedData, Record, TABLE_HEADER};
use std::fs::{read_to_string, write};
use rand;

pub trait DataForSelection {
  type ChoosingData;
  fn select(&self) -> u8;
}

impl DataForSelection for u8 {
  type ChoosingData = u8;
  fn select(&self) -> u8 {
    rand::random_range(1..=self.clone())
  }
}

impl DataForSelection for Vec<u8> {
  type ChoosingData = Vec<u8>;
  fn select(&self) -> u8 {
    let index= rand::random_range(0..self.len());
    self[index] as u8
  }
}


pub fn choose_two_days_number_of_week() -> Vec<u32> {
  let mut days: Vec<u32> = vec![];
  while days.len() < 2 {
    let day = rand::random_range(1..=7);
    if !days.contains(&day) {
      days.push(day);
    }
  }
  days
}
#[cfg(test)]
#[test]
fn test_choose_two_days_number_of_week() {
    let days= choose_two_days_number_of_week();
    assert_eq!(days.len(), 2);
    assert_in_range!(days[0], 1..=7);
    assert_in_range!(days[1], 1..=7);
    assert!(days[0] != days[1]);
}


pub fn choose_one_from_amount<Amount: DataForSelection>(amount: &Amount) -> u8 { 
  amount.select()
}

#[test]
fn test_choose_one_from_amount_as_number() {
    let ex= 6 as u8;
    assert_in_range!(choose_one_from_amount(&ex), 1..=ex);
}
#[test]
fn test_choose_one_from_amount_as_vector() {
    let vector_data: Vec<u8> = [2, 7, 9, 1, 15].to_vec();
    let res = choose_one_from_amount(&vector_data);
    assert_contains!(vector_data, &res);
}


pub fn get_diary_creator(diary_type: DiaryTypes) -> DiaryCreator {
  match diary_type {
    DiaryTypes::FivePerWeek => get_diary_five_per_week,
    DiaryTypes::FivePerWeek22Work => get_diary_five_per_week_22_work,
    _ => get_diary_five_per_week,
  }
}


const EMPTY_STR: &str = "";
pub fn get_empty_record(date: NaiveDate) -> Record {
  (date.format("%d.%m.%Y").to_string(), [EMPTY_STR; 6]) //, EMPTY_STR, EMPTY_STR, EMPTY_STR, EMPTY_STR, EMPTY_STR)
}

#[test]
fn test_get_empty_record() {
    let date: NaiveDate = NaiveDate::from_ymd_opt(2026, 07, 02).unwrap();
    let record: Record = get_empty_record(date);
    assert_eq!(record, ("02.07.2026".to_string(), [""; 6])); // , "", "", "", "", "",));
}


pub fn get_main_data () -> MainData {
  let res = read_to_string("input.json").expect("Can't read file");
  let res = serde_json::from_str::<ParsedData>(&res).expect("Incorrect data in input.json");
  MainData::from(res)
}


pub fn mark_measurement(mut record: Record, measurements: Vec<u8>) -> Record {
  for measurement in measurements {
    record.1[(measurement - 1) as usize] = MARK;
  }
  record
}

#[test]
fn test_mark_measurement() {
    let date: NaiveDate = NaiveDate::from_ymd_opt(2026, 07, 02).unwrap();
    let record: Record = get_empty_record(date);
    let measurement = vec![2, 5];
    assert_eq!(mark_measurement(record, measurement), ("02.07.2026".to_string(), ["", "●", "", "", "●", "",]));
}


pub fn write_diary(diary: Vec<Record>) {
  let mut content = TABLE_HEADER.to_string();
  for (date, marks) in diary.into_iter() {
      let rec = "\n".to_string() + &date.clone() + "|" + &marks.join("|");
      content.push_str(&rec);
  }
  let _ = write("output.txt", content).unwrap();
}

// // function coose two indexes from 1 to 'amount'.
// // If first is even (after eating), then second should be uneven (before eating)
// export function chooseTwoFromAmount(amount) {
//   let first = Math.ceil(Math.random() * amount);
//   if (first === 0) {
//     first = 1;
//   }
//   const secondSet = [];
//   const startIndex = first % 2 === 0 ? 1 : 2;
//   for (let i = startIndex; i <= amount; i += 2) {
//     secondSet.push(i);
//   }
//   let secondIndex = Math.floor(Math.random() * secondSet.length);
//   if (secondIndex === secondSet.length) {
//     secondIndex = secondSet.length - 1;
//   }
//   const second = secondSet[secondIndex];
//   return [first, second];
// }



// /**
//  * Reads the file and parses the input data.
//  * @returns {Object} An object with the start date and the length of the diary.
//  */

// export function parseInput() {
//   try {
//     const content = fs.readFileSync('./src/input.json', 'utf-8');
//     const params = JSON.parse(content);
//     params.startDate = new Date(params.startDate);
//     return params;
//   } catch (e) {
//     return {};
//   }
// }