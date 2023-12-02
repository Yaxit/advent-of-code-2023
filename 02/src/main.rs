use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {},
        _ => {
            println!("Usage: main <file-path> <part> (1 or 2, default 1)");
            return;
        },
    }
    let path = &args[1];

    if let Ok(lines) = read_lines(path) {
        let mut sum = 0;
        let mut power = 0;
        for line in lines {
            if let Ok(l) = line {
                if let Some((id_part, data_slices)) = parse_sequences(&l) {
                    let id = parse_game_id(id_part);
                    let data = parse_data(data_slices);
                    if game_possible(&data, 12, 13, 14){
                        // println!("Game {} is valid", id);
                        sum += id
                    }
                    power += game_power(&data);
                }
            }
        }
        println!("Sum: {}", sum);
        println!("Power: {}", power);
    }
}

fn parse_sequences(line: &String) -> Option<(&str, &str)> {
    let parts = line.split_once(": ");
    // match the contents of the Option using match
    if let Some((sub_id, sub_data)) = parts {
        return Some((sub_id, sub_data));
    }
    None
}

// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_data(data: &str) -> Vec<(u32, u32, u32)> {
    let game_set: Vec<(u32,u32,u32)> = data.split(";")
            .map(|s| s.trim())
            // [3 blue, 4 red][1 red, 2 green, 6 blue][2 green]
            .map(|s| s.split(",")
            .map(|s| s.trim())
            // [[3 blue][4 red]][[1 red][2 green][6 blue]][[2 green]]
            .map(|s| {
                let (num,color) = s.split_once(" ")
                                .map(|(n,c)| (n.parse::<u32>().unwrap(), c))
                                .unwrap();
                match color {
                    "red" => {
                        (num,0,0)
                    },
                    "green" => {
                        (0,num,0)
                    },
                    "blue" => {
                        (0,0,num)
                    },
                    _ => {
                        println!("Unknown color: {}", color);
                        (0,0,0)
                    }
                }
            }).collect::<Vec<(u32,u32,u32)>>()).flatten().collect();
    game_set
}


fn parse_game_id(line: &str) -> u32 {
    // match the contents of the Option using match
    return line.split_whitespace().nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
}

fn game_possible(game_set: &Vec<(u32,u32,u32)>, r:u32, g:u32, b:u32) -> bool {
    for v in game_set {
        if v.0 > r || v.1 > g || v.2 > b {
            return false;
        }
    }
    true
}

fn game_power(game_set: &Vec<(u32,u32,u32)>) -> u32 {
    let max: (u32,u32,u32) = game_set.iter().fold((0,0,0), |acc, v| {
        (acc.0.max(v.0), acc.1.max(v.1), acc.2.max(v.2))
    });
    max.0 * max.1 * max.2
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
