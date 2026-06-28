use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Deserialize, Serialize, Debug)]
pub enum DiaryTypes {
    TwoPerday22Work, // two measurements per day with a 2/2 work schedule
    TwoPerDay, // two measurements per day without limits
    FivePerWeek22Work, // five measurements per week with a 2/2 work schedule
    FivePerWeek, // five measurements per week without limits
}

const DAYS_IN_WEEK: u8 = 7;
const MILLISECONDS_PER_DAY: u32 = 60*60*24*1000;
const DAYS_IN_WORK_CYCLE: u8 = 4;
const AVAILABLE_MEASUREMENTS_IN_WORK_DAY: [u8; 3] = [1,5,6]; //can be mesuried b.breakfast, b.dinner, a.dinner
const NUMBER_OF_MEASUREMENTS_IN_NIGHT_WORK_DAY: u8 = 4; //can be mesuried b.breakfast, a.breakfast, b.dinner, a.dinner
const NUMBER_OF_MEASUREMENTS_IN_OTHER_DAY: u8 = 6; //all mesurement
const TABLE_HEADER: &str = concat!("   Дата|Измерение сахара крови, ммоль/л\n",
                                  "|Завтрак||Обед||Ужин\n",
                                  "|До|После|До|После|До|После\n");


#[derive(Deserialize, Serialize, Debug)]
pub struct ParsedData {
    start_date: String,
    diary_length: u16,
    diary_type: DiaryTypes,
}
#[derive(Debug)]

pub struct MainData {
  pub start_date: NaiveDate,
  diary_length: u16,
  diary_type: DiaryTypes,
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