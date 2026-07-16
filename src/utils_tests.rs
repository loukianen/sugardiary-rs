#[cfg(test)]
use crate::utils::*;
#[cfg(test)]
use chrono::NaiveDate;
#[cfg(test)]
use crate::consts::Record;

#[test]
fn test_choose_two_days_number_of_week() {
    let days= choose_two_days_number_of_week();
    assert_eq!(days.len(), 2);
    assert_in_range!(days[0], 1..=7);
    assert_in_range!(days[1], 1..=7);
    assert!(days[0] != days[1]);
}


#[test]
fn test_choose_one_from_amount_as_number() {
  let ex= 6 as u8;
  assert_in_range!(choose_one_from_amount(&ex), 1..=ex);
  assert_eq!(choose_one_from_amount(&1), 1);
}
#[test]
fn test_choose_one_from_amount_as_vector() {
  let vector_data: Vec<u8> = [2, 7, 9, 1, 15].to_vec();
  let res = choose_one_from_amount(&vector_data);
  assert_contains!(vector_data, &res);
  assert_eq!(choose_one_from_amount(&[2].to_vec()), 2
);
}

#[test]
#[should_panic(expected = "Невозможно выбрать из полученных данных")]
fn test_panic_in_choose_one_from_amount_as_number() {
  choose_one_from_amount(&0);
}
#[test]
#[should_panic(expected = "Невозможно выбрать из полученных данных")]
fn test_panic_in_choose_one_from_amount_as_vector() {
  choose_one_from_amount(&[].to_vec());
}


#[test]
fn test_choose_two_from_amount_as_number() {
  let ex= 6 as u8;
  let res = choose_two_from_amount(&ex);
  assert_contains!(1..=ex, &res[0]);
  assert_contains!(1..=ex, &res[1]);
  assert_ne!(&res[0], &res[1]);
  let res_from_2 = choose_two_from_amount(&2);
  assert_contains!(res_from_2, &1);
  assert_contains!(res_from_2, &2);
}
#[test]
fn test_choose_two_from_amount_as_vector() {
  let vector_data: Vec<u8> = [2, 7, 9, 1, 15, 36, 159, 0].to_vec();
  let res = choose_two_from_amount(&vector_data);
  assert_eq!(res.len(), 2);
  assert_contains!(vector_data, &res[0]);
  assert_contains!(vector_data, &res[1]);
  assert_ne!(&res[0], &res[1]);
}

#[test]
#[should_panic(expected = "Невозможно выбрать из полученных данных")]
fn test_panic_in_choose_two_from_amount_as_number() {
  choose_two_from_amount(&1);
}
#[test]
#[should_panic(expected = "Невозможно выбрать из полученных данных")]
fn test_panic_in_choose_two_from_amount_as_vector() {
  choose_two_from_amount(&[45].to_vec());
}


#[test]
fn test_get_empty_record() {
    let date: NaiveDate = NaiveDate::from_ymd_opt(2026, 07, 02).unwrap();
    let record: Record = get_empty_record(date);
    assert_eq!(record, ("02.07.2026".to_string(), [""; 6]));
}


#[test]
fn test_mark_measurement() {
    let date: NaiveDate = NaiveDate::from_ymd_opt(2026, 07, 02).unwrap();
    let record: Record = get_empty_record(date);
    let measurement = vec![2, 5];
    assert_eq!(mark_measurement(record, measurement), ("02.07.2026".to_string(), ["", "●", "", "", "●", "",]));
}