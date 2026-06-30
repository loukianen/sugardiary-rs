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

// export function getEmptyRecord(date) {
//   return [date.toLocaleDateString(), '', '', '', '', '', ''];
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