use std::fmt;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;

pub type Record = (String, [&'static str; 6]); // &'static str, &'static str, &'static str, &'static str, &'static str);
pub type DiaryCreator = fn (start_date: NaiveDate, diary_length: u32) -> Vec<Record>;
pub const MARK: &str = "●";
pub const DAYS_IN_WEEK: u32 = 7;
pub const DAYS_IN_WORK_CYCLE: u32 = 4;
pub const AVAILABLE_MEASUREMENTS_IN_WORK_DAY: [u32; 3] = [1, 5, 6]; //can be mesuried b.breakfast, b.dinner, a.dinner
pub const NUMBER_OF_MEASUREMENTS_IN_NIGHT_WORK_DAY: u32 = 4; //can be mesuried b.breakfast, a.breakfast, b.dinner, a.dinner
pub const NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY: u32 = 6; //all mesurement
pub const TABLE_HEADER: &str = concat!("    Дата|Измерение сахара крови, ммоль/л\n",
                                  "|Завтрак||Обед||Ужин\n",
                                  "|До|После|До|После|До|После");

#[derive(Clone, Copy)]
pub enum TermTextColors {
    Blue,
    Green,
    Red,
    Reset,
}

impl fmt::Display for TermTextColors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermTextColors::Blue =>write!(f, "\x1b[36m"),
            TermTextColors::Green => write!(f, "\x1b[32m"),
            TermTextColors::Red => write!(f, "\x1b[31m"),
            TermTextColors::Reset => write!(f, "\x1b[0m"),
        }
    }
}


#[repr(usize)]
#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Serialize, IntoPrimitive, TryFromPrimitive,
    PartialEq, Eq,     // Реализует операции ==, !=
    PartialOrd, Ord,)] // Реализует остальные операции сравнения, в т.ч. используется при сортировке
pub enum DiaryTypes {
    TwoPerDay22Work = 3, // two measurements per day with a 2/2 work schedule
    FivePerWeek22Work = 1, // five measurements per week with a 2/2 work schedule
    TwoPerDay = 2, // two measurements per day without limits
    FivePerWeek = 0, // five measurements per week without limits
}

impl fmt::Display for DiaryTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiaryTypes::FivePerWeek => write!(f, "Пять измерений в неделю без ограничений"),
            DiaryTypes::FivePerWeek22Work => write!(f, "Пять измерений в неделю c рабочим графиком 2/2 с ночной сменой"),
            DiaryTypes::TwoPerDay => write!(f, "Два измерения в день без ограничений"),
            DiaryTypes::TwoPerDay22Work => write!(f, "Два измерения в день с рабочим графиком 2/2 с ночной сменой"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParsedData {
    start_date: String,
    diary_length: u32,
    diary_type: DiaryTypes,
}

#[derive(Clone, Debug)]
pub struct MainData {
  pub start_date: NaiveDate,
  pub diary_length: u32,
  pub diary_type: DiaryTypes,
}

impl From<ParsedData> for MainData {
    fn from(s: ParsedData) -> MainData {
        MainData { 
            start_date: NaiveDate::parse_from_str(&s.start_date, "%d.%m.%Y").expect("Неверный формат даты в input.json"),
            diary_length: s.diary_length,
            diary_type: s.diary_type,
        }
    }
}

impl Into<ParsedData> for MainData {
    fn into(self) -> ParsedData {
        ParsedData {
            start_date: self.start_date.format("%d.%m.%Y").to_string(),
            diary_length: self.diary_length,
            diary_type: self.diary_type
        }
    }
}