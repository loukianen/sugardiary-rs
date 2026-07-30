use crate::consts::{DiaryTypes, MainData};
use chrono::NaiveDate;
use dialoguer::{Input, Select};
use strum::IntoEnumIterator;

pub fn get_data_from_user(data: MainData) -> MainData {
    let date_str: String = Input::new()
        .with_prompt("\nВведите дату начала дневника.\nЕсли дневник с рабочим циклом, то начало должно совпадать с первым днем")
        .default(data.start_date.format("%d.%m.%Y").to_string())
        .interact_text()
        .unwrap();

    // println!("Inputed date: {}", date_str);

    let len: u32 = Input::new()
        .with_prompt("\nВведите длину дневника в днях")
        .default(data.diary_length.clone())
        .interact()
        .unwrap();

    // println!("Inputed length: {}", len);

    let mut list = vec![];
    for item in DiaryTypes::iter() {
        list.push(item);
    }
    list.sort();

    let selected = Select::new()
        .with_prompt("\nВыберите тип дневника")
        .default(0)
        .items(list)
        .interact()
        .unwrap();

    // println!("Selected: {}", DiaryTypes::try_from(selected).unwrap());
    MainData {
        start_date: NaiveDate::parse_from_str(&date_str, "%d.%m.%Y").expect("Неверный формат даты"),
        diary_length: len,
        diary_type: DiaryTypes::try_from(selected).unwrap()
    }
}
