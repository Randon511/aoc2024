use std::collections::{HashMap, HashSet};
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

    let mut dir_index: HashMap<char, usize> = HashMap::new();
    dir_index.insert('^', 0);
    dir_index.insert('>', 1);
    dir_index.insert('v', 2);
    dir_index.insert('<', 3);
    let direction: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut res = 0;

    let points = find_points(
        &grid,
        start_pos,
        &dir_index,
        &direction,
        &start_char,
        num_rows,
        num_cols,
    );

    for point in points {
        if grid[point.0][point.1] == '.' {
            let mut new_grid = grid.clone();
            new_grid[point.0][point.1] = 'O';

            if find_loop(
                &new_grid,
                start_pos,
                &dir_index,
                &direction,
                &start_char,
                num_rows,
                num_cols,
            ) {
                res += 1;
            }
        }
    }

    println!("Result :{}", res);
}

fn find_points(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    dir_index: &HashMap<char, usize>,
    direction: &Vec<(isize, isize)>,
    start_char: &char,
    num_rows: usize,
    num_cols: usize,
) -> HashSet<(usize, usize)> {
    let mut cur_pos = start_pos;
    let mut cur_dir_index = *dir_index.get(&start_char).unwrap();
    let start_dir: (isize, isize) = direction[cur_dir_index];
    let mut cur_dir = start_dir;
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    loop {
        // Bounds checks
        if (cur_pos.0 as isize + cur_dir.0 == -1)
            || (cur_pos.0 as isize + cur_dir.0 == num_rows as isize)
            || (cur_pos.1 as isize + cur_dir.1 == -1)
            || (cur_pos.1 as isize + cur_dir.1 == num_cols as isize)
        {
            break;
        }

        // Check for wall or obstruction
        if ['#', 'O'].contains(
            &grid[(cur_pos.0 as isize + cur_dir.0) as usize]
                [(cur_pos.1 as isize + cur_dir.1) as usize],
        ) {
            // Change dir
            cur_dir_index = (cur_dir_index + 1) % 4;
            cur_dir = direction[cur_dir_index];
        }
        // Take a step
        else {
            cur_pos.0 = (cur_pos.0 as isize + cur_dir.0) as usize;
            cur_pos.1 = (cur_pos.1 as isize + cur_dir.1) as usize;
            path.insert(cur_pos);
        }
    }
    return path;
}

fn find_loop(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    dir_index: &HashMap<char, usize>,
    direction: &Vec<(isize, isize)>,
    start_char: &char,
    num_rows: usize,
    num_cols: usize,
) -> bool {
    let mut cur_pos = start_pos;
    let mut cur_dir_index = *dir_index.get(&start_char).unwrap();
    let start_dir: (isize, isize) = direction[cur_dir_index];
    let mut cur_dir = start_dir;
    let mut tracker: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    tracker.insert((start_pos, start_dir));

    loop {
        // Bounds checks
        if (cur_pos.0 as isize + cur_dir.0 == -1)
            || (cur_pos.0 as isize + cur_dir.0 == num_rows as isize)
            || (cur_pos.1 as isize + cur_dir.1 == -1)
            || (cur_pos.1 as isize + cur_dir.1 == num_cols as isize)
        {
            return false;
        }

        // Check for wall or obstruction
        if ['#', 'O'].contains(
            &grid[(cur_pos.0 as isize + cur_dir.0) as usize]
                [(cur_pos.1 as isize + cur_dir.1) as usize],
        ) {
            // Change dir
            cur_dir_index = (cur_dir_index + 1) % 4;
            cur_dir = direction[cur_dir_index];
        }
        // Take a step
        else {
            cur_pos.0 = (cur_pos.0 as isize + cur_dir.0) as usize;
            cur_pos.1 = (cur_pos.1 as isize + cur_dir.1) as usize;

            if !tracker.insert((cur_pos, cur_dir)) {
                return true;
            }
        }
    }
}
