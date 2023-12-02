use std::{fs::File, io::Read};

const SPELLED: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGITS: [&str;10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn find_digits(str: &str, reverse: bool) -> Option<u32> {
    let mut matching_index: Option<usize> = None;

    let mut found_match: bool = false;

    let mut digit: Option<u32> = None;
    for (index, spelled) in SPELLED.iter().enumerate() {
        let mut found_index: Option<usize> = None;
        if (reverse) {
            found_index = str.rfind(spelled);
            found_match = found_index.is_some() && (matching_index.is_none() || found_index.unwrap() > matching_index.unwrap());
        } else {
            found_index = str.find(spelled);
            found_match = found_index.is_some() && (matching_index.is_none() || found_index.unwrap() < matching_index.unwrap());
        }

        if !found_index.is_none() && found_match {
            matching_index = found_index;
            digit.insert(index as u32);
        }
    }

    if reverse {
        let reversedIndex = str
            .chars()
            .rev()
            .position(|x| x.is_numeric());
        let index = reversedIndex.map_or(None, |x| Some(str.len() - x - 1));
        if index.is_some()
            && (matching_index.is_none() || index.unwrap() > matching_index.unwrap()) {
            matching_index = index;
            digit = str.chars().nth(index.unwrap()).map_or(None, |x| x.to_digit(10));
            found_match = true;
        }
    } else {
        let index = str
            .chars()
            .position(|x| x.is_numeric());
        if index.is_some()
            && (matching_index.is_none() || index.unwrap() < matching_index.unwrap()) {
            matching_index = index;
            digit = str.chars().nth(index.unwrap()).map_or(None, |x| x.to_digit(10));
            found_match = true;
        }
    }

    if !found_match {
        matching_index = None;
    }

    return digit;
}

fn sum_chars(first: Option<u32>, last: Option<u32>) -> u32 {
    let mut local_sum = 0;

    if !first.is_none() {
        local_sum += first.unwrap();
        if last.is_some() {
            local_sum *= 10;
        }
    }

    if last.is_some() {
        local_sum += last.unwrap();
    }

    println!("{}", local_sum);

    return local_sum;
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;
    let mut sumPart2 = 0;

    for (_, line) in lines.iter().enumerate() {
        print!("{} - ", line);
        let first = line
            .chars()
            .find(|c| c.is_numeric())
            .map_or(None, |c| c.to_digit(10));
        let last = line
            .chars()
            .rfind(|c| c.is_numeric())
            .map_or(None, |c| c.to_digit(10));;

        let spelledFirst = find_digits(line, false);
        let spelledLast = find_digits(line, true);

        sum += sum_chars(first, last);
        sumPart2 += sum_chars(spelledFirst, spelledLast);
    }


    println!("sum: {}", sum);
    println!("sump2: {}", sumPart2);
}
