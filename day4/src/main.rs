use std::collections::HashMap;
use std::{fs::File, io::Read};
use regex::Regex;

fn get_wins(winning: &Vec<i32>, mine: &Vec<i32>, i: usize, cards: &Vec<(usize, Vec<i32>, Vec<i32>)>, memo: &mut HashMap<usize, Vec<usize>>) -> (i32, usize, Vec<usize>) {
    if memo.contains_key(&i) {
        // println!("Memo hit");
        let vec = memo.get(&i).unwrap();
        return (0, vec.len(), vec.clone());
    }

    let mut win_pts = 0;
    let mut num_matches = 0;
    for num in mine.iter() {
        if winning.contains(num) {
            num_matches += 1;
            if win_pts == 0 {
                win_pts = 1;
            } else {
                win_pts *= 2;
            }
        }
    }

    let mut copies: Vec<usize> = Vec::new();

    let mut j = i + 1;
    let next_index: usize = num_matches.try_into().unwrap();

    while j < i + next_index + 1 {
        // if (j + 1) >= cards.len() {
        //     break;
        // }
        copies.push(j + 1);
        j += 1;
    }

    memo.insert(i, copies.clone());

    return (win_pts, num_matches, copies.clone());
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut cards: Vec<(usize, Vec<i32>, Vec<i32>)> = Vec::new();
    let mut totalPts = 0;

    for (i, line) in lines.iter().enumerate() {
        let id = line.split(":").nth(0);
        let re = Regex::new(r"\s{2,}").unwrap();
        let relevant = line.split(":").skip(1).nth(0).unwrap().trim();
        let processed = re.replace_all(relevant, " ");
        let splits: Vec<&str>  = processed.split("|").collect();

        let mut winning: Vec<i32> = splits.iter()
            .nth(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap()).collect();


        let mine:Vec<i32> = splits.iter().nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap()).collect();

        cards.push((i + 1, winning, mine));
    }

    let mut memo: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut to_process: Vec<usize> = cards.iter().map(|x| x.0).collect();
    let mut final_index = to_process.len();
    let mut i = 0;

    while i < final_index {
        let cardIdx = to_process.get(i).unwrap().to_owned();
        let (cardNb, winning, mine) = cards.get(cardIdx-1).unwrap();
        let sized = cardNb - 1;

        let memoized = memo.get(&sized);
        // let process: Vec<String> = to_process.iter().enumerate().map(|(j, x)| {
        //     if i == j.to_owned() {
        //         return format!("[{}]", x.to_string().as_str().to_owned());
        //     }
        //     return x.to_string().as_str().to_owned();
        // }).collect();
        // println!("Processing: {}", process.join(", "));

        // println!("Card {} {:?}", cardNb, mine);

        if memoized.is_some() {
            let vec = memoized.unwrap();
            to_process.append(&mut vec.clone());
            final_index += vec.len();
            i += 1;
            continue;
        }

        let (_, _, copies) = get_wins(
            &winning, 
            &mine, 
            cardNb-1, 
            &cards,
            &mut memo);
    
        to_process.append(&mut copies.clone());

        final_index += copies.len();
        i += 1;
    }

    println!("----------------");
    println!("Cards: {:?}", to_process.len());
}
