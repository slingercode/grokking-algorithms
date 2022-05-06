use std::convert::TryInto;

fn binary_search(search_list: &[i8], value: i8) -> i8 {
  let mut low: usize = 0;
  let mut high: usize = search_list.len() - 1;

  while low <= high {
    let mid: usize = ((low + high) / 2).try_into().unwrap();
    let guess: i8 = search_list[mid];

    if guess == value {
      return mid.try_into().unwrap();
    }

    if guess < value {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  return -1;
}

fn main() {
  let search_list: &[i8] = &[1, 2, 3, 4, 5, 6, 7, 8];
  let search_value: i8 = 8;
  
  println!("Binary Search");
  println!("Search value: {}", search_value);

  let result = binary_search(search_list, search_value);
  let wrong = binary_search(search_list, 9);

  println!("Result: {}", result);
  println!("Wrong: {}", wrong);
}