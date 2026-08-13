use crate::consts::{DiaryTypes, MainData};
use crate::consts::TermTextColors::{Blue, Red, Reset};
use chrono::{NaiveDate, ParseResult};
use dialoguer::{Error, Input, Result, Select};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use strum::IntoEnumIterator;

pub fn get_data_from_user(data: MainData) -> MainData {
    let date_str: String = Input::new()
        .validate_with(date_validator)
        .with_prompt(format!("{}{}{}", Blue, "\nВведите дату начала дневника.\nЕсли дневник с рабочим циклом, то начало должно совпадать с первым днем цикла", Reset))
        .default(data.start_date.format("%d.%m.%Y").to_string())
        .interact_text()
        .unwrap();

    let len: u32 = Input::new()
        .validate_with(diary_length_validator)
        .with_prompt(format!("{}{}{}", Blue, "\nВведите длину дневника в днях (от 1 до 1000)", Reset))
        .default(data.diary_length.to_string())
        .interact()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut list = vec![];
    for item in DiaryTypes::iter() {
        list.push(item);
    }
    list.sort();

    let selected = Select::new()
        .with_prompt(format!("{}{}{}", Blue, "\nВыберите тип дневника", Reset))
        .default(0)
        .items(list)
        .interact()
        .unwrap();

    MainData {
        start_date: NaiveDate::parse_from_str(&date_str, "%d.%m.%Y").expect(&format!("{}{}{}", Red, "Неверный формат даты", Reset)),
        diary_length: len,
        diary_type: DiaryTypes::try_from(selected).unwrap()
    }
}

fn date_validator(text: &String) -> Result<()> {
    let validation_error = Err(Error::from(IoError::new(IoErrorKind::Other, format!("{}{}{}", Red, "\nВведен неверный формат даты.\nВведите дату в формате 'дд.мм.гггг'", Reset))));
    if text.len() != 10 {
            return validation_error;
    }
    let date: ParseResult<NaiveDate> = NaiveDate::parse_from_str(&text, "%d.%m.%Y");
    match date {
        Ok(_) => Ok(()),
        Err(_) => validation_error,
    }
}

#[cfg(test)]
mod tests_date_validator {
    use super::*;
    // Вспомогательная функция, чтобы не дублировать код проверки результата
    fn is_valid(input: &String) -> bool {
        date_validator(input).is_ok()
    }

    #[test]
    fn valid_dates_are_accepted() {
        assert!(is_valid(&"01.01.2000".to_string()));
        assert!(is_valid(&"31.12.2023".to_string()));
        assert!(is_valid(&"29.02.2024".to_string())); // високосный год — дата существует
        assert!(is_valid(&"15.07.1999".to_string()));
    }

    #[test]
    fn invalid_dates_are_rejected_by_logic() {
        // Несуществующий день в месяце
        assert!(!is_valid(&"31.04.2023".to_string())); // в апреле нет 31 числа
        assert!(!is_valid(&"32.01.2023".to_string())); // день > 31
        // Неверная дата из-за високосного года
        assert!(!is_valid(&"29.02.2023".to_string())); // 2023 — не високосный
        // Месяц вне диапазона
        assert!(!is_valid(&"01.13.2023".to_string()));
        assert!(!is_valid(&"01.00.2023".to_string()));
    }

    #[test]
    fn wrong_format_is_rejected() {
        // Разделители не точки
        assert!(!is_valid(&"01/01/2000".to_string()));
        assert!(!is_valid(&"01-01-2000".to_string()));
        // Нет ведущих нулей
        assert!(!is_valid(&"1.01.2000".to_string()));
        assert!(!is_valid(&"01.1.2000".to_string()));
        // Год не 4 цифры
        assert!(!is_valid(&"01.01.20".to_string()));
        assert!(!is_valid(&"01.01.20233".to_string()));
        // Пустая строка
        assert!(!is_valid(&"".to_string()));
        // Мусор
        assert!(!is_valid(&"abc.def.ghij".to_string()));
        assert!(!is_valid(&"01.aa.2000".to_string()));
    }

    #[test]
    fn error_message_contains_expected_text() {
        let result = date_validator(&"bad-date".to_string());
        assert!(result.is_err(), "Ожидалась ошибка для невалидного ввода");

        let msg = result.err().unwrap().to_string();
        assert!(
            msg.contains("неверный формат"),
            "Сообщение об ошибке должно содержать фразу 'неверный формат', но было: {}",
            msg
        );
        assert!(
            msg.contains("дд.мм.гггг"),
            "Сообщение об ошибке должно подсказывать формат 'дд.мм.гггг', но было: {}",
            msg
        );
    }
}

fn diary_length_validator(text: &String) -> Result<()> {
    let validation_error = Err(Error::from(IoError::new(IoErrorKind::Other, format!("{}{}{}", Red, "\nВведена неверная длина дневника.\nВведите число дней от 1 до 1000", Reset))));
    let len = match text.parse::<u32>() {
        Ok(number) => number,
        Err(_) => 0,
    };
    if len < 1 || len > 1000 {
        validation_error
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests_diary_len_validator {
    use super::*;

    #[test]
    fn test_valid_length_min() {
        let input = String::from("1");
        let result = diary_length_validator(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_length_max() {
        let input = String::from("1000");
        let result = diary_length_validator(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_length_middle() {
        let input = String::from("500");
        let result = diary_length_validator(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_length_zero() {
        let input = String::from("0");
        let result = diary_length_validator(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_length_below_min() {
        let input = String::from("-5"); // парсинг в u32 даст ошибку, либо проверка диапазона
        let result = diary_length_validator(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_length_above_max() {
        let input = String::from("1001");
        let result = diary_length_validator(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_non_numeric_input() {
        let input = String::from("abc");
        let result = diary_length_validator(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_string() {
        let input = String::new();
        let result = diary_length_validator(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_message_contains_expected_text() {
        let input = String::from("0");
        let err = diary_length_validator(&input).unwrap_err();

        let msg = err.to_string();

        assert!(msg.contains("Введена неверная длина дневника."), "Сообщение об ошибке должно содержать ожидаемый текст");
        assert!(msg.contains("от 1 до 1000"), "Сообщение должно содержать диапазон");
    }
}
