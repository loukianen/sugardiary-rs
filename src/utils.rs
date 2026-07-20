use chrono::{Local, NaiveDate};
use crate::get_diary_five_per_week::get_diary_five_per_week;
use crate::get_diary_five_per_week_22_work::get_diary_five_per_week_22_work;
use crate::get_diary_two_per_day::get_diary_two_per_day;
use crate::get_diary_two_per_day_22_work::get_diary_two_per_day_22_work;
use crate::consts::*;
use std::fs::{exists, read_to_string, write};
use rand;

pub trait DataForSelection {
  type ChoosingData;
  fn select(&self) -> u32;
  fn select_two(&self) -> [u32; 2];
}

impl DataForSelection for u32 {
  type ChoosingData = u32;
  fn select(&self) -> u32 {
    if self.clone() < 1 {
      panic!("Невозможно выбрать из полученных данных");
    }
    rand::random_range(1..=self.clone())
  }
  
  fn select_two(&self) -> [u32; 2] {
    if self.clone() < 2 {
      panic!("Невозможно выбрать из полученных данных");
    }
    let first = rand::random_range(1..=self.clone());
    let mut second_set= vec![];
    let mut cur_num: u32 = if &first % 2 == 0 {1} else {2};
    while cur_num <= self.clone() {
      second_set.push(cur_num.clone());
      cur_num += 2;
    }
    let index_for_second = rand::random_range(0..second_set.len());
    let second = second_set[index_for_second];
    [first, second]
  }
}

impl DataForSelection for Vec<u32> {
  type ChoosingData = Vec<u32>;
  fn select(&self) -> u32 {
    if self.len() < 1 {
      panic!("Невозможно выбрать из полученных данных");
    }
    let index= rand::random_range(0..self.len());
    self[index] as u32
  }

  fn select_two(&self) -> [u32; 2] {
    if self.len() < 2 {
      panic!("Невозможно выбрать из полученных данных");
    }
    let index_for_first = rand::random_range(0..self.len());
    let mut index_for_second = rand::random_range(0..self.len());
    while index_for_first == index_for_second {
        index_for_second = rand::random_range(0..self.len());
    }
    [*&self[index_for_first], *&self[index_for_second]]
  }
}


pub fn choose_one_from_amount<Amount: DataForSelection>(amount: &Amount) -> u32 { 
  amount.select()
}

pub fn choose_two_from_amount<Amount: DataForSelection>(amount: &Amount) -> [u32; 2] { 
  amount.select_two()
}


pub fn get_diary_creator(diary_type: DiaryTypes) -> DiaryCreator {
  match diary_type {
    DiaryTypes::FivePerWeek => get_diary_five_per_week,
    DiaryTypes::FivePerWeek22Work => get_diary_five_per_week_22_work,
    DiaryTypes::TwoPerDay => get_diary_two_per_day,
    DiaryTypes::TwoPerDay22Work => get_diary_two_per_day_22_work,
  }
}


const EMPTY_STR: &str = "";
pub fn get_empty_record(date: NaiveDate) -> Record {
  (date.format("%d.%m.%Y").to_string(), [EMPTY_STR; 6])
}


pub fn get_main_data () -> MainData {
  if exists("input.json").unwrap() {
    println!("File not expect but branch is working");
    let res = read_to_string("input.json").expect("Can't read file");
    let res = serde_json::from_str::<ParsedData>(&res).expect("Incorrect data in input.json");
    MainData::from(res)
  } else {
    MainData {
      start_date: Local::now().date_naive(),
      diary_length: 92,
      diary_type: DiaryTypes::FivePerWeek,
    }
  }
}

pub fn mark_measurement(mut record: Record, measurements: Vec<u32>) -> Record {
  for measurement in measurements {
    record.1[(measurement - 1) as usize] = MARK;
  }
  record
}


pub fn write_diary(diary: Vec<Record>) {
  let mut content = TABLE_HEADER.to_string();
  for (date, marks) in diary.into_iter() {
      let rec = "\n".to_string() + &date.clone() + "|" + &marks.join("|");
      content.push_str(&rec);
  }
  let _ = write("output.txt", content).unwrap();
}
