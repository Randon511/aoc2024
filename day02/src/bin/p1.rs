use std::fs;
use std::path::Path;

fn main() {
  let path = Path::new("src/bin/in1.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  let mut res = 0;
  
  input.split("\r\n")
        .for_each(|report|
        {
          let nums : Vec<i32> = report.split(" ")
                                      .filter_map(|num| num.parse::<i32>().ok())
                                      .collect();

          let mut inc = false;

          if nums.len() > 1
          {
            for i in 1..nums.len()
            {
              let diff = nums[i] - nums[i - 1];

              if i == 1
              {
                if diff > 0
                {
                  inc = true;
                }
                else 
                {
                  inc = false;
                }
              }
              
              if (diff == 0) || 
                  (diff > 0 && !inc) || 
                  (diff < 0 && inc) ||
                  (diff.abs() > 3)
              {
                break;
              }
              
              if i == nums.len() - 1
              {
                res +=1;
              }
            }
          }
        });

  println!("Result: {}", res);
}
