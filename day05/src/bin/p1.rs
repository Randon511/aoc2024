use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
  let path = Path::new("src/bin/in1.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");
  
  let mut rules : HashMap<u32, Vec<u32>> = HashMap::new();
  let mut updates : Vec<Vec<u32>> = vec![];

  for row in input.split("\r\n")
  {
    if row.contains("|")
    {
      let nums : Vec<&str> = row.split("|").collect();     
      let num1 = nums[0].parse::<u32>().ok().unwrap();
      let num2 = nums[1].parse::<u32>().ok().unwrap();
      
      rules.entry(num1).or_insert_with(Vec::new).push(num2);
    }
    
    if row.contains(",")
    {
      let nums : Vec<u32> = row.replace("\r\n","").split(",").filter_map(|c| c.parse::<u32>().ok()).collect();     
      updates.push(nums);
    }
  }

  let mut res = 0; 

  'update_loop: for update in updates
  {
    for i in (0..update.len()).rev()
    {
      if !rules.contains_key(&update[i])
      {
        continue;
      }
      
      let after_pages : &Vec<u32> = rules.get(&update[i]).unwrap();

      for j in 0..i
      {
        if after_pages.contains(&update[j])
        {
          continue 'update_loop;
        }
      }
    }
    
    res += update[(update.len() - 1) /2];
  }
  
  println!("Result: {}", res);
}
