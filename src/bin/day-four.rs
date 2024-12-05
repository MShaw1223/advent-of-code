use std::io::BufRead;

use regex::Regex;

fn main() {
    part_one();
}

fn process_input(input: String) {
    println!("{input}");

    let mut sum = 0;
    sum += horizontal(&input);
    sum += diagonal(&input);
    println!("h: {sum}");
}
fn horizontal(input: &String) -> i32 {
    let re = Regex::new(r"XMAS").expect("Error Creating Regex");
    if re.is_match(&input) {
        return 1;
    } else {
        return 0;
    }
}
fn diagonal(input: &String) -> i32 {
    // lock to a value and look at next depth of the matrix?
    // so nest a for loop check i against i+1 k ; then i + 2, k + 1
    // i being position in line, k being line num
    println!("{input}");
    return 0;
}

fn part_one() {
    let mut i = 0;
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(ln) => {
                print!("{i}|");
                process_input(ln)
            }
            Err(e) => eprintln!("ERR: {e:?}"),
        }
        i += 1
    }
}
