use std::collections::{HashSet, HashMap};
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

        let mut points:usize = 0;
        let mut cards:usize = 0;
        let mut map:HashMap<usize,usize> = HashMap::new();

        for line in lines {
            if let Ok(l) = line {
                let (id, s ) = l.split_once(": ").unwrap();
                let id = parse_id(id);
                let (win, num) = s.split_once("|").unwrap();

                let win = win.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<HashSet<usize>>();
                let num = num.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<HashSet<usize>>();

                let common = win.intersection(&num);
                let common = common.count();
                let set_points= 2usize.pow(common.try_into().unwrap())/2;
                points = points + set_points;

                map.insert(id, map.get(&id).unwrap_or(&0)+1);
                let copies_cnt = *map.get(&id).unwrap();

                for k in id+1..=id+common {
                    if let Some(prev) = map.insert(k, copies_cnt) {
                        map.insert(k, prev+copies_cnt);
                    }
                }
                cards = cards + map.get(&id).unwrap_or(&1);
            }
        }

        println!("Set points: {}", points);
        println!("Cards: {}", cards);
    }

}

fn parse_id(id: &str) -> usize {
    id.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
