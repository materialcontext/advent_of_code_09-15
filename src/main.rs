use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// =========== Convenience Functions ========= //
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn aoc_09() {
    struct XY {x: i32, y: i32}

    impl XY {
        fn up(&self) {self.x; self.y + 1; }
        fn down(&self) {self.x; self.y - 1; }
        fn left(&self) {self.x - 1; self.y; }
        fn right(&self) {self.x + 1; self.y; }
    }

    #[derive(PartialEq)]
    enum Heading {
        North,
        East,
        South,
        West,
        NorthEast,
        SouthEast,
        SouthWest,
        NorthWest,
        Centered
    }

    struct Rope {
        head: XY,
        tail: XY,
        orientation: Heading
    }

    impl Rope {
        fn step(&mut self, direction: &str) { 
            match direction {
                "U" => {
                    match self.orientation {
                        Heading::North => {
                            self.tail.up();
                        }
                        Heading::NorthEast => {
                            self.tail.up();
                            self.tail.right();
                        }
                        Heading::NorthWest => {
                            self.tail.up();
                            self.tail.left();
                        }
                        _ => {}
                    }
                }
                "D" => { 
                    self.head.down();
                    match self.orientation {
                        Heading::South => {
                            self.tail.down();
                        }
                        Heading::SouthEast => {
                            self.tail.down();
                            self.tail.right();
                        }
                        Heading::SouthWest => {
                            self.tail.down();
                            self.tail.left();
                        }
                        _ => {}
                    }
                }
                "L" => { 
                    self.head.left();
                    match self.orientation {
                        Heading::West => {
                            self.tail.left();
                        }
                        Heading::NorthWest => {
                            self.tail.left();
                            self.tail.up();
                        }
                        Heading::SouthWest => {
                            self.tail.left();
                            self.tail.down();
                        }
                        _ => {}
                    }
                }
                "R" => { 
                    self.head.right();
                    match self.orientation {
                        Heading::East => {
                            self.tail.right();
                        }
                        Heading::NorthEast => {
                            self.tail.right();
                            self.tail.up();
                        }
                        Heading::SouthEast => {
                            self.tail.right();
                            self.tail.down();
                        }
                        _ => {}
                    }
                 }
                _ => {println!("error")}
            }
            self.update_orientation();
        }

        fn update_orientation(&mut self) {
            let mut x_pos = Heading::Centered;
            let mut y_pos = Heading::Centered;

            match &self.head.x.cmp(&self.tail.x) {
                Ordering::Greater => {
                    x_pos = Heading::East;
                }
                Ordering::Less => {
                    x_pos = Heading::West;
                }
                Ordering::Equal => {
                    x_pos = Heading::Centered;
                }
            }

            match &self.head.y.cmp(&self.tail.y) {
                Ordering::Greater => {
                    y_pos = Heading::North;
                }
                Ordering::Less => {
                    y_pos = Heading::South;
                }
                Ordering::Equal => {
                    y_pos = Heading::Centered;
                }
            }

            match (x_pos, y_pos) {
                (Heading::East, Heading::North) => {
                    self.orientation = Heading::NorthEast;
                }
                (Heading::East, Heading::South) => {
                    self.orientation = Heading::SouthEast;
                }
                (Heading::East, Heading::Centered) => {
                    self.orientation = Heading::East;
                }
                (Heading::West, Heading::North) => {
                    self.orientation = Heading::NorthWest;
                }
                (Heading::West, Heading::South) => {
                    self.orientation = Heading::SouthWest;
                }
                (Heading::West, Heading::Centered) => {
                    self.orientation = Heading::West;
                }
                (Heading::Centered, Heading::North) => {
                    self.orientation = Heading::North;
                }
                (Heading::Centered, Heading::South) => {
                    self.orientation = Heading::South;
                }
                _ => {
                    self.orientation = Heading::Centered;
                }
            }
        
        }
    }
 }
// =========== Main ========= //
fn main() {
    aoc_09();
}