use chrono::{NaiveDate, Weekday};
use rand;
use serde::Serialize;
use crate::consts::{MARK, Record};

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

pub fn choose_one_from_amount<Amount: DataForSelection>(amount: &Amount) -> u8 { 
  amount.select()
}

#[cfg(test)]

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

const EMPTY_STR: &str = "";
pub fn get_empty_record(date: NaiveDate) -> Record {
  (date.format("%d.%m.%Y").to_string(), EMPTY_STR, EMPTY_STR, EMPTY_STR, EMPTY_STR, EMPTY_STR, EMPTY_STR)
}

#[test]
fn test_get_empty_record() {
    let date: NaiveDate = NaiveDate::from_ymd_opt(2026, 07, 02).unwrap();
    let record: Record = get_empty_record(date);
    assert_eq!(record, ("02.07.2026".to_string(), "", "", "", "", "", "",));
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

#[test]
fn test_choose_two_days_number_of_week() {
    let days= choose_two_days_number_of_week();
    assert_eq!(days.len(), 2);
    assert_in_range!(days[0], 1..=7);
    assert_in_range!(days[1], 1..=7);
    assert!(days[0] != days[1]);
}

pub fn mark_measurement(mut record: Record, measurements: Vec<u8>) -> Record {
  for measurement in measurements {
    match measurement{
      1 => record.1 = MARK,
      2 => record.2 = MARK,
      3 => record.3 = MARK,
      4 => record.4 = MARK,
      5 => record.5 = MARK,
      6 => record.6 = MARK,
      _ => (),
    }
  }
  record
}

#[test]
fn test_mark_measurement() {
    let date: NaiveDate = NaiveDate::from_ymd_opt(2026, 07, 02).unwrap();
    let record: Record = get_empty_record(date);
    let measurement = vec![2, 5];
    assert_eq!(mark_measurement(record, measurement), ("02.07.2026".to_string(), "", "●", "", "", "●", "",));
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