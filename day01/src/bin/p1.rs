use std::fs;
use std::path::Path;

fn main() {
  let path = Path::new("src/bin/in1.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  let mut left_list: Vec<i32> = vec![];
  let mut right_list: Vec<i32> = vec![];

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

  left_list.sort();
  right_list.sort();

  let mut res = 0;

  for (index, num) in left_list.iter().enumerate()
  {
    res += (num - right_list[index]).abs();
  }

  println!("Result: {}", res);
}
