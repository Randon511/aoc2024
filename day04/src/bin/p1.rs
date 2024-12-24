use std::fs;
use std::path::Path;

fn main() {
  let path = Path::new("src/bin/in1.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  let mut input_vec : Vec<Vec<char>> = Vec::new();

  for row in input.split("\r\n")
  {
    input_vec.push(row.chars().collect())
  }
  
   let num_rows = input_vec.len();
   let num_cols = input_vec[0].len();
   let dirs : Vec<(isize, isize)> = vec![(0,1),(0,-1),(1,0),(-1,0),(1,1),(-1,-1),(1,-1),(-1,1)];
   
   let mut res = 0;

   for row in 0..num_rows
   {
    for col in 0..num_cols  
      {
       for &(row_dir, col_dir) in &dirs
       {
          if is_match(col as isize, row as isize, row_dir, col_dir, &input_vec)
          {
            res += 1;
          }
       }
      }
   }

  println!("Result: {}", res);
}

fn is_match(s_col: isize, s_row: isize, col_dir: isize, row_dir: isize, input : &Vec<Vec<char>>) -> bool
{
  let target = vec!['X', 'M', 'A', 'S'];

  if input[s_row as usize][s_col as usize] != target[0]
  {
    return false;
  }

  for i in 0..4
  {
    let next_row = s_row + (i * row_dir);
    let next_col = s_col + (i * col_dir);
    
    if next_col < 0 || next_col >= input[0].len() as isize|| next_row < 0 || next_row >= input.len() as isize
    {
      return false;
    }
    
    if input[next_row as usize][next_col as usize] != target[i as usize]
    {
      return false
    }
  } 
  
  return true;
}