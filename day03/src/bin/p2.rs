use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
  let path = Path::new("src/bin/in.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  let re = Regex::new(r"(mul\((-?[0-9]*),(-?[0-9]*)\)|do\(\)|don't\(\))").unwrap();

  let mut res = 0;
  let mut enabled = true;

  for capture in re.captures_iter(&input)
  {
    if let Some(full_match) = capture.get(0)
    {
      println!("{}", full_match.as_str());
      if full_match.as_str().starts_with("don")
      {
        enabled = false;
      }
      else if full_match.as_str().starts_with("do")
      {
        enabled = true;
      }
      else if enabled
      {
        let val1 = capture.get(2).map_or(0,|c| c.as_str().parse::<i32>().unwrap());
        let val2 = capture.get(3).map_or(0,|c| c.as_str().parse::<i32>().unwrap());
        
        res += val1 * val2;
      }
    }     
  }

  println!("Result: {}", res);
}

