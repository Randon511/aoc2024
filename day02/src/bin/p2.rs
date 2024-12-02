use std::fs;
use std::path::Path;

fn main() {
  let path = Path::new("src/bin/in2.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  let mut res = 0;
      
  input.split("\r\n")
        .for_each(|report|
        {
          let report_nums : Vec<i32> = report.split(" ")
                                      .filter_map(|num| num.parse::<i32>().ok())
                                      .collect();

          if valid_report(report_nums, false)
          {
            println!("{:?}", report);
            res += 1;
          }
        });

  println!("Result: {}", res);
}

fn valid_report(report : Vec<i32>, damped: bool) -> bool
{
  let mut inc = false;

  if report.len() > 1
  {
    for i in 1..report.len()
    {
      let diff = report[i] - report[i - 1];

      if i == 1
      {
        if diff > 0
        {
          inc = true;
        }
      }
      
      if (diff == 0) || 
          (diff > 0 && !inc) || 
          (diff < 0 && inc) ||
          (diff.abs() > 3)
      {
        if damped == true
        {
          return false;
        }
        else
        {
          for j in 0..report.len()
          {
            let mut mod_report = report.clone();
            mod_report.remove(j);
            if valid_report(mod_report, true)
            {
              return true;
            } 
          }
          return false;
        }
      }
      
      if i == report.len() - 1
      {
        return true;
      }
    }
  }
  
  return false;
}