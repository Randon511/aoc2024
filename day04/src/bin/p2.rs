
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
  let path = Path::new("src/bin/in2.txt");
  let input = fs::read_to_string(path).expect("Failed to open file");

  let mut input_vec : Vec<Vec<char>> = Vec::new();

  for row in input.split("\r\n")
  {
    input_vec.push(row.chars().collect())
  }
  
   let num_rows = input_vec.len();
   let num_cols = input_vec[0].len();
   let dirs : Vec<(isize, isize)> = vec![(1,1),(-1,-1),(1,-1),(-1,1)];
   
   let mut coord_counts : HashMap<(usize, usize), usize> = HashMap::new();

   for row in 0..num_rows
   {
    for col in 0..num_cols  
    {
      for &(row_dir, col_dir) in &dirs
      {
        let coords = is_match(col as isize, row as isize, row_dir, col_dir, &input_vec);
        if coords != (-1,-1) 
        {
          *coord_counts.entry((coords.0 as usize, coords.1 as usize)).or_insert(0) += 1;
        }
      }
    }
   }
   
   let res = coord_counts.values().filter(|&&count| count == 2).count();
   
  println!("Result: {}", res);
}

fn is_match(s_col: isize, s_row: isize, col_dir: isize, row_dir: isize, input : &Vec<Vec<char>>) -> (isize, isize)
{
  let target = vec!['M', 'A', 'S'];
  let mut a_coordinate = (-1,-1);

  if input[s_row as usize][s_col as usize] != target[0]
  {
    return (-1,-1);
  }

  for i in 0..target.len()
  {
    let next_row = s_row + (i as isize * row_dir);
    let next_col = s_col + (i as isize * col_dir);

    if next_col < 0 || next_col >= input[0].len() as isize|| next_row < 0 || next_row >= input.len() as isize
    {
      return (-1,-1);
    }
    
    if input[next_row as usize][next_col as usize] != target[i as usize]
    {
      return (-1,-1); 
    }
    
    if input[next_row as usize][next_col as usize] == 'A'
    {
      a_coordinate = (next_row, next_col);
    }
  } 
  
  return a_coordinate;
}