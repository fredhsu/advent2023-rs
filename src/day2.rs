use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug, Copy, Clone)]
struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}
impl Game {
    fn is_possible(&self) -> bool {
        if self.red <= 12 && self.green <= 13 && self.blue <= 14 {
            return true;
        }
        false
    }
    fn set_blue(&mut self, blue: u32) {
        if blue > self.blue {
            self.blue = blue
        }
    }
    fn set_green(&mut self, green: u32) {
        if green > self.green {
            self.green = green
        }
    }
    fn set_red(&mut self, red: u32) {
        if red > self.red {
            self.red = red
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_set(set: &str) -> (u32, u32, u32) {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    let colors = set.split(", ");
    for color in colors {
        if color.ends_with("red") {
            red = color.strip_suffix(" red").unwrap().parse().unwrap();
        }
        if color.ends_with("green") {
            green = color.strip_suffix(" green").unwrap().parse().unwrap();
        }
        if color.ends_with("blue") {
            blue = color.strip_suffix(" blue").unwrap().parse().unwrap();
        }
    }
    (red, green, blue)
}
fn parse_line(line: &str) -> Game {
    let (g, r) = line.split_once(": ").unwrap();
    let id = g.strip_prefix("Game ").unwrap().parse().unwrap();
    let mut game = Game {
        id,
        blue: 0,
        red: 0,
        green: 0,
    };
    let sets = r.split("; ");
    for set in sets {
        let (red, green, blue) = parse_set(set);
        game.set_blue(blue);
        game.set_red(red);
        game.set_green(green);
    }
    game
}
fn main() {
    let tests = lines_from_file("day2input.txt");
    let mut total = 0;
    let mut total_power = 0;
    for t in tests {
        let game = parse_line(&t);
        dbg!(game);
        if game.is_possible() {
            total = total + game.id;
        }
        let power = game.red * game.blue * game.green;
        total_power = total_power + power;
    }
    println!("possible {total}");
    println!("power {total_power}")
}
