use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
  let mut left_list: Vec<i32> = vec![];
  let mut right_list: Vec<i32> = vec![];

  let path = Path::new("src/bin/in2.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  input.replace("   ", ",")
        .replace("\r\n", ",")
        .split(",")
        .filter_map(|num| num.parse::<i32>().ok())
        .enumerate()
        .for_each(|(index, num)|
        {
          if (index % 2) == 0
          {
            left_list.push(num);
          }
          else 
          {
            right_list.push(num);
          }
        });

  let mut freq_map: HashMap<i32, i32> = HashMap::new();
  
  for num in right_list
  {
    *freq_map.entry(num).or_insert(0) += 1;
  }

  let mut res = 0;

  for num in left_list
  {
    match freq_map.entry(num)
    {
      std::collections::hash_map::Entry::Occupied(entry) =>
      {
        res += num *entry.get()
      }
      std::collections::hash_map::Entry::Vacant(_) =>
      {
        continue;
      }
    }
  }
  println!("Result: {}", res);
}
