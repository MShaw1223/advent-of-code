use std::io::BufRead;

use regex::Regex;

fn main() {
    //part_one();
    part_two();
}
fn process_mul(line: String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)+").expect("Regex creation failed");
    let mut line_sum = 0;

    for cap in re.captures_iter(&line) {
        let first_val = cap.get(1).unwrap().as_str();
        let second_val = cap.get(2).unwrap().as_str();
        let fv_integer: i32 = first_val.parse().unwrap();
        let sv_integer: i32 = second_val.parse().unwrap();

        let sum_mul: i32 = fv_integer * sv_integer;
        line_sum += sum_mul;
    }

    line_sum
}
fn part_one() {
    let mut sum = 0;
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => sum += process_mul(line),
            Err(e) => println!("Error: {e:?}"),
        }
    }
    println!("Sum: {sum}")
}

fn part_two() {
    let mut sum: i32 = 0;
    let re_do = Regex::new(r"^do\(\)").unwrap();
    //let re_do_not = Regex::new(r"don't\(\)").unwrap();

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                if re_do.is_match(&line) {
                    sum += process_mul(line)
                }
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }
    println!("{sum}")
}
