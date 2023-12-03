use std::{fs::File, io::Read};
use std::fmt::Debug;
use std::ops::Deref;
use regex::Regex;

fn get_max_shown(line: &str, cube_type: &str) -> Option<i32> {
    let re = Regex::new(format!(r"(?<num>\d+) {}", cube_type).as_str()).unwrap();

    let results: Vec<i32> = re.captures_iter(line)
        .map(|m|
            m.name("num").unwrap().as_str().parse::<i32>().unwrap()
        ).collect();

    return results.iter().max().map_or(None, |x| Some(x.to_owned()));
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let red = 12;
    let green = 13;
    let blue = 14;


    let mut possible = 0;
    let mut sumPowers = 0;
    for (i, line) in lines.iter().enumerate() {
        let id = line.split(":").nth(0);
        let relevant = line.split(":").skip(1).nth(0).unwrap().trim();
        let maxR = get_max_shown(relevant, "red");
        let maxG = get_max_shown(relevant, "green");
        let maxB = get_max_shown(relevant, "blue");

        let rValid = red - maxR.unwrap();
        let gValid = green - maxG.unwrap();
        let bValid = blue - maxB.unwrap();
        let power = maxR.unwrap() * maxG.unwrap() * maxB.unwrap();
        println!("{}", line);
        println!("r: {}", maxR.unwrap());
        println!("g: {}", maxG.unwrap());
        println!("b: {}", maxB.unwrap());
        println!("r: {}", rValid);
        println!("g: {}", gValid);
        println!("b: {}", bValid);
        println!("power: {}", power);
        sumPowers += power;
        if rValid < 0 || gValid < 0 || bValid < 0 {
            println!("{} invalid", (i + 1).to_string());
        } else {
            possible += i + 1;
        }
    }

    println!("{}", possible);
    println!("{}", sumPowers);
}
