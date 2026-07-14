use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

pub type Record = (String, [&'static str; 6]); // &'static str, &'static str, &'static str, &'static str, &'static str);
pub type DiaryCreator = fn (start_date: NaiveDate, diary_length: u16) -> Vec<Record>;
pub const MARK: &str = "●";
pub const DAYS_IN_WEEK: u32 = 7;
pub const DAYS_IN_WORK_CYCLE: u16 = 4;
pub const AVAILABLE_MEASUREMENTS_IN_WORK_DAY: [u8; 3] = [1, 5, 6]; //can be mesuried b.breakfast, b.dinner, a.dinner
pub const NUMBER_OF_MEASUREMENTS_IN_NIGHT_WORK_DAY: u8 = 4; //can be mesuried b.breakfast, a.breakfast, b.dinner, a.dinner
pub const NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY: u8 = 6; //all mesurement
pub const TABLE_HEADER: &str = concat!("    Дата|Измерение сахара крови, ммоль/л\n",
                                  "|Завтрак||Обед||Ужин\n",
                                  "|До|После|До|После|До|После");

#[derive(Deserialize, Serialize, Debug)]
pub enum DiaryTypes {
    TwoPerDay22Work, // two measurements per day with a 2/2 work schedule
    TwoPerDay, // two measurements per day without limits
    FivePerWeek22Work, // five measurements per week with a 2/2 work schedule
    FivePerWeek, // five measurements per week without limits
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParsedData {
    start_date: String,
    diary_length: u16,
    diary_type: DiaryTypes,
}

#[derive(Debug)]
pub struct MainData {
  pub start_date: NaiveDate,
  pub diary_length: u16,
  pub diary_type: DiaryTypes,
}

impl From<ParsedData> for MainData {
    fn from(s: ParsedData) -> MainData {
        MainData {
            start_date: NaiveDate::parse_from_str(&s.start_date, "%Y-%m-%d").expect("Неверный формат даты"),
            diary_length: s.diary_length,
            diary_type: s.diary_type,
        }
    }
}