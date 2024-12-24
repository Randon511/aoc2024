use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("src/bin/in1_real.txt");
    let input = fs::read_to_string(path).expect("Failed to open file");

    let grid: Vec<Vec<char>> = input
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let string_index = input
        .replace("\r\n", "")
        .find(|c| ['^', '>', 'v', '<'].contains(&c))
        .unwrap();
    let start_pos: (usize, usize) = ((string_index / num_cols) as usize, string_index % num_cols);
    let start_char = grid[start_pos.0][start_pos.1];

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut dir_index: HashMap<char, usize> = HashMap::new();
    dir_index.insert('^', 0);
    dir_index.insert('>', 1);
    dir_index.insert('v', 2);
    dir_index.insert('<', 3);

    let direction: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut done = false;
    let mut cur_pos = start_pos;
    let mut cur_dir_index = *dir_index.get(&start_char).unwrap();
    let mut cur_dir: (isize, isize) = direction[cur_dir_index];

    while !done {
        visited.insert(cur_pos);

        // Bounds checks
        if (cur_pos.0 as isize + cur_dir.0 == -1)
            || (cur_pos.0 as isize + cur_dir.0 == num_rows as isize)
            || (cur_pos.1 as isize + cur_dir.1 == -1)
            || (cur_pos.1 as isize + cur_dir.1 == num_cols as isize)
        {
            break;
        }

        // Check for wall
        if grid[(cur_pos.0 as isize + cur_dir.0) as usize]
            [(cur_pos.1 as isize + cur_dir.1) as usize]
            == '#'
        {
            // Change dir
            cur_dir_index = (cur_dir_index + 1) % 4;
            cur_dir = direction[cur_dir_index];
        }
        // Take a step
        else {
            cur_pos.0 = (cur_pos.0 as isize + cur_dir.0) as usize;
            cur_pos.1 = (cur_pos.1 as isize + cur_dir.1) as usize;
        }
    }

    println!("Result: {}", visited.len());
}
