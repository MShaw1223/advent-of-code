use std::{
    collections::HashMap,
    i32,
    io::{self, Write},
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    // declarations
    let mut total_distance: i32 = 0;
    println!("Enter List of pairs:");
    let (mut left_list, mut right_list) = lists();

    // sort lists, calc total distance
    left_list.sort();
    right_list.sort();

    // just wants distance from x so find absolute val of difference ll[i] & rl[i]
    for i in 0..left_list.len() {
        total_distance += (left_list[i] - right_list[i]).abs()
    }

    println!("Dist: {}", total_distance);
}

// add up each num in LL and multiply by times it appears in RL
// worked for small example, didnt work to solve prob.
// solution as shown below requires hashmap of rl and * occurences in ll
// this leaves ans too low?
// was doing occ * val square but its value times the occurrence (see i)
fn part_two() {
    println!("Enter list here: ");

    let (ll, rl) = lists();

    let mut occurrences_map = HashMap::new();
    for val in rl {
        // instead of just .insert(1) presence check val and assign/inc as necessary
        // (issues from this)
        *occurrences_map.entry(val).or_insert(0) += 1;
    }
    println!("occurences initialised: {:?}", occurrences_map);

    let mut similarity: i32 = 0;
    for val_l in ll {
        // i -- I was taking a mutable ref and altering it then in a sep loop working out similarity as deref occ * val_l^2 which was incorrect.
        // did occ times val as the examples threes confused me on what is multiplied by what.
        if let Some(occ) = occurrences_map.get(&val_l) {
            similarity += val_l * *occ;
        }
    }
    println!("occurences altered: {:?}", occurrences_map);

    println!("Similarity score: {}", similarity);
}

fn lists() -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input...");

        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let nums: Vec<i32> = input
            .split_whitespace()
            .filter_map(|st| st.parse::<i32>().ok())
            .collect();

        if nums.len() == 2 {
            left_list.push(nums[0]);
            right_list.push(nums[1]);
        } else {
            println!("Input invalid, enter list again: ");
        }
    }
    return (left_list, right_list);
}
