use std::io::BufRead;

use regex::Regex;

fn main() {
    //part_one();
    part_two();
}

fn part_one() {
    let mut sum: i32 = 0;

    for line in std::io::stdin().lock().lines() {
        let re = Regex::new(r"mul\((\d+),(\d+)\)+");
        match line {
            Ok(line) => {
                for cap in re.expect("Func Regex Error").captures_iter(&line) {
                    let first_val = cap.get(1).unwrap().as_str();
                    let second_val = cap.get(2).unwrap().as_str();
                    let fv_integer: i32 = first_val.parse().unwrap();
                    let sv_integer: i32 = second_val.parse().unwrap();

                    let sum_mul: i32 = fv_integer * sv_integer;

                    sum += sum_mul
                }
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }
    println!("{sum}")
}
// not working yet
fn part_two() {
    let mut act: bool = true;

    let mut sum: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)+");
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_do_not = Regex::new(r"don't\(\)").unwrap();

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                if re_do.is_match(&line) {
                    act = true
                } else if re_do_not.is_match(&line) {
                    act = false
                }

                if act {
                    for cap in re.clone().expect("re Regex Error").captures_iter(&line) {
                        let first_val = cap.get(1).unwrap().as_str();
                        let second_val = cap.get(2).unwrap().as_str();
                        let fv_integer: i32 = first_val.parse().unwrap();
                        let sv_integer: i32 = second_val.parse().unwrap();

                        let sum_mul: i32 = fv_integer * sv_integer;

                        sum += sum_mul
                    }
                }
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }
    println!("{sum}")
}
