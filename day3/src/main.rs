use std::{fs::File, io::Read};
use std::collections::HashMap;
use std::fmt::Debug;
use regex::{Match, Regex};

fn get_neighbors(content: &str, index: usize) -> ([char; 9], [Option<usize>; 9]) {
    let numCols = content.chars().position(|x| x == '\n').map(|x| x + 1).unwrap();
    let center = content.chars().nth(index);

    let mut topLeftIndex = None;
    let mut topIndex = None;
    let mut topRightIndex = None;
    let centerIndex = Some(index);
    let mut botIndex = None;
    let mut leftIndex = None;
    let rightIndex = Some(index + 1);
    let mut botLeftIndex = None;
    let mut botRightIndex = None;

    let mut left: Option<char> = None;
    if index > 1 {
        left = content.chars().nth(index-1);
        leftIndex.insert(index - 1);
    }
    let right = content.chars().nth(index+1);

    let mut top: Option<char> = None;
    let mut topLeft: Option<char> = None;
    let mut topRight: Option<char> = None;
    let mut botLeft: Option<char> = None;


    let rowIndex = index / numCols;
    let indexInRow = index % numCols;

    if index > numCols {
        topIndex = Some(numCols * (rowIndex - 1) + indexInRow);
        top = content.chars().nth(topIndex.unwrap());
        if topIndex.unwrap() > 0 {
            topLeft = content.chars().nth(topIndex.unwrap() - 1);
            topLeftIndex = Some(topIndex.unwrap() - 1);
        }
        topRight = content.chars().nth(topIndex.unwrap() + 1);
        topRightIndex = Some(topIndex.unwrap() + 1);
    }

    botIndex = Some(numCols * (rowIndex + 1) + indexInRow);

    if botIndex.unwrap() > 0 {
        botLeft = content.chars().nth(botIndex.unwrap() - 1);
        botLeftIndex = Some(botIndex.unwrap() - 1);
    }
    let bot = content.chars().nth(botIndex.unwrap());
    let botRight = content.chars().nth(botIndex.unwrap() + 1);
    botRightIndex = Some(botIndex.unwrap() + 1);

    let all = [
        topLeft.or(Some('.')).unwrap(),
        top.or(Some('.')).unwrap(),
        topRight.or(Some('.')).unwrap(),
        left.or(Some('.')).unwrap(),
        center.unwrap(),
        right.or(Some('.')).unwrap(),
        botLeft.or(Some('.')).unwrap(),
        bot.or(Some('.')).unwrap(),
        botRight.or(Some('.')).unwrap()
    ];


    let indices = [
        topLeftIndex,
        topIndex,
        topRightIndex,
        leftIndex,
        centerIndex,
        rightIndex,
        botLeftIndex,
        botIndex,
        botRightIndex
    ];
    return (all, indices);
}

fn has_valid_neighbors(content: &str, index: usize) -> (bool, [char; 9], [Option<usize>; 9]) {
    let neighbors = get_neighbors(content, index);
    let all = neighbors.0;
    let indices = neighbors.1;
    let neighbors_valid =  all
        .iter()
        .any(|x| !x.is_digit(10) && x.to_owned() != '.' && x.to_owned() != '\n');
    return (all[4].is_digit(10) && neighbors_valid, all, indices);
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let re = Regex::new(r"\d+").unwrap();
    let iter = re.find_iter(contents.as_str());
    let mut sum = 0;
    let mut numbers: Vec<Match> = Vec::new();

    let mut starmap: HashMap<usize, Vec<(i32, usize)>> = HashMap::new();

    for (_, m) in iter.enumerate() {
        let mut i = m.start();

        let mut anyValid = false;
        let mut anyStar = false;
        // let mut starIndex = None;
        let number = m.as_str().parse::<i32>().expect("Unable to parse");

        while i < m.end() {
            let valid = has_valid_neighbors(contents.as_str(), i);
            // let stars = valid.1.iter().filter(|x| x.to_owned() == '*');


            for (index, star) in valid.1.iter().enumerate() {
                if (*star != '*') {
                    continue;
                }
                let idx = valid.2.iter().nth(index);
                let realIndex = idx.unwrap().to_owned();

                let mut vec = starmap.entry(realIndex.unwrap()).or_insert(Vec::new());

                if !vec.iter().any(|x| x.0 == number && x.1 == m.start()) {
                    vec.push((number, m.start()));
                }
            }

            if valid.0 {
                anyValid = true;
            }
            i = i + 1;
        }

        // println!("Star {} - {} - {}", m.as_str(), anyStar, starIndex.or(Some(1000 as usize)).unwrap());

        if anyValid {
            sum += number;
        }
        // println!("{} - {} at {}-{}", anyValid,  m.as_str(), m.start(), m.end());
    }

    let mut p2sum = 0;
    for key in starmap.keys() {
        let vec: &Vec<(i32, usize)> = starmap.get(key).unwrap();
        let vecSum = vec.iter().map(|x| x.0).product::<i32>();
        if (vec.len() == 2) {
            p2sum += vecSum;
        }
        println!("{} - {} - {}", key, vec.len(), vecSum);
    }


    println!("{}", sum);
    println!("{}", p2sum);


    // let lines: Vec<&str> = contents.split("\n").collect();
}
