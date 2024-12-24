use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("src/bin/in1.txt");
    let input = fs::read_to_string(path).expect("Failed to open file");

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];

    for row in input.split("\r\n") {
        if row.contains("|") {
            let nums: Vec<&str> = row.split("|").collect();
            let num1 = nums[0].parse::<u32>().ok().unwrap();
            let num2 = nums[1].parse::<u32>().ok().unwrap();

            rules.entry(num1).or_insert_with(Vec::new).push(num2);
        }

        if row.contains(",") {
            let nums: Vec<u32> = row
                .replace("\r\n", "")
                .split(",")
                .filter_map(|c| c.parse::<u32>().ok())
                .collect();
            updates.push(nums);
        }
    }

    let mut res = 0;

    let mut wrong_updates: Vec<Vec<u32>> = vec![];

    'update_loop: for update in updates {
        for i in (0..update.len()).rev() {
            if !rules.contains_key(&update[i]) {
                continue;
            }

            let after_pages: &Vec<u32> = rules.get(&update[i]).unwrap();

            for j in 0..i {
                if after_pages.contains(&update[j]) {
                    wrong_updates.push(update);
                    continue 'update_loop;
                }
            }
        }
    }

    while !wrong_updates.is_empty() {
        let mut new_wrong_updates: Vec<Vec<u32>> = vec![];

        'check_loop: for mut update in wrong_updates.drain(..) {
            for i in (0..update.len()).rev() {
                if !rules.contains_key(&update[i]) {
                    continue;
                }

                let after_pages: &Vec<u32> = rules.get(&update[i]).unwrap();

                for j in 0..i {
                    if after_pages.contains(&update[j]) {
                        let num = update.remove(j);
                        update.insert(i as usize, num);
                        new_wrong_updates.push(update);
                        continue 'check_loop;
                    }
                }
            }

            res += update[(update.len() - 1) / 2];
        }

        wrong_updates = new_wrong_updates;
    }

    println!("Result: {}", res);
}
