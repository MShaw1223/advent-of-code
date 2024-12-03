use std::io::{self, BufRead};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut safe_report_count: i32 = 0;
    let data: Vec<Vec<i32>> = input_vec();

    // data is a matrix of 'reports'
    for report in data {
        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut is_safe = true;
        // each report has many level: check each report has all levels inc/decr
        for level in 0..report.len() - 1 {
            if report[level] < report[level + 1] {
                is_decreasing = false;
            } else if report[level] > report[level + 1] {
                is_increasing = false
            }

            let difference = (report[level] - report[level + 1]).abs();

            if difference < 1 || difference > 3 {
                is_safe = false
            }
        }
        if (is_increasing || is_decreasing) && is_safe {
            safe_report_count += 1
        }
    }
    println!("\n{safe_report_count}")
}

fn part_two() {
    let mut safe_report_count: i32 = 0;

    //make this mut and alter resuls??
    // no...
    let data: Vec<Vec<i32>> = input_vec();
    println!("{:?}", &data);

    for report in data {
        if is_safe_report(&report) || problem_dampener_mod(&report) {
            safe_report_count += 1
        }
    }
    println!("\n{safe_report_count}");
}

fn problem_dampener_mod(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        // new report...
        let mut modified_report = report.clone();
        // ...with one level removed
        modified_report.remove(i);

        if is_safe_report(&modified_report) {
            return true; // true if safe with removed level
        }
    }
    false
}

// shouldve broken away into sep func sm sooner
fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut is_safe = true;

    for level in 0..report.len() - 1 {
        if report[level] < report[level + 1] {
            is_decreasing = false;
        } else if report[level] > report[level + 1] {
            is_increasing = false
        }

        let difference = (report[level] - report[level + 1]).abs();

        if difference < 1 || difference > 3 {
            is_safe = false;
            println!("{:?}", report);
        }
    }

    let is_exclusive = is_increasing ^ is_decreasing;

    is_safe && is_exclusive
}

// this took painfully long reading bufread docs before realising you need an EOF :| (hence println)
fn input_vec() -> Vec<Vec<i32>> {
    println!("Press CTRL+D after pasting input.");
    let mut return_vec = Vec::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let mut temp_vec = Vec::new();

                for nums in line.split_whitespace() {
                    let num: i32 = nums.parse().unwrap();
                    temp_vec.push(num);
                }
                return_vec.push(temp_vec);
                //println!("\nln:{:?}", line);
            }
            Err(_) => {
                break;
            }
        }
    }
    return return_vec;
}
