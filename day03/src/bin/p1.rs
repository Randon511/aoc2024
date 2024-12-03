use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
  let path = Path::new("src/bin/in.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");
  
  let re = Regex::new(r"mul\((-?[0-9]*),(-?[0-9]*)\)").unwrap();

  let mut pairs = vec![];

  for (_, [num1, num2]) in re.captures_iter(&input).map(|c| c.extract())
  {
    pairs.push((num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()));
  }

  let mut res = 0;

  for (val1, val2) in pairs
  {
    res += val1 * val2;
  }  

  println!("Result: {}", res);
}