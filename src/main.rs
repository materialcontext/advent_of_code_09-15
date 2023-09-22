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
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct XY {x: i32, y: i32}

    impl XY {
        fn up(&self) -> XY {XY {x: self.x, y: self.y + 1 } }
        fn down(&self) -> XY {XY {x: self.x, y: self.y - 1 } }
        fn left(&self) -> XY {XY {x: self.x - 1, y: self.y } }
        fn right(&self) -> XY {XY {x: self.x + 1, y: self.y } }
    }

    #[derive(PartialEq, Debug)]
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

    struct Segment {
        head: XY,
        tail: XY,
        orientation: Heading
    }

    impl Segment {
        fn step(&mut self, direction: &str) -> &str { 
            let next_direction;
            match direction {
                "U" => {
                    self.head = self.head.up();
                    match self.orientation {
                        Heading::North => {
                            self.tail = self.tail.up();
                            next_direction = "U";
                        }
                        Heading::NorthEast => {
                            self.tail = self.tail.up();
                            self.tail = self.tail.right();
                            next_direction = "UR";
                        }
                        Heading::NorthWest => {
                            self.tail = self.tail.up();
                            self.tail = self.tail.left();
                            next_direction = "UL";
                        }
                        _ => {
                            next_direction = "C";
                        }
                    }
                }
                "D" => { 
                    self.head = self.head.down();
                    match self.orientation {
                        Heading::South => {
                            self.tail = self.tail.down();
                            next_direction = "D";
                        }
                        Heading::SouthEast => {
                            self.tail = self.tail.down();
                            self.tail = self.tail.right();
                            next_direction = "DR";
                        }
                        Heading::SouthWest => {
                            self.tail = self.tail.down();
                            self.tail = self.tail.left();
                            next_direction = "DL";
                        }
                        _ => {
                            next_direction = "C";
                        }
                    }
                }
                "L" => { 
                    self.head = self.head.left();
                    match self.orientation {
                        Heading::West => {
                            self.tail = self.tail.left();
                            next_direction = "L";
                        }
                        Heading::NorthWest => {
                            self.tail = self.tail.left();
                            self.tail = self.tail.up();
                            next_direction = "UL";
                        }
                        Heading::SouthWest => {
                            self.tail = self.tail.left();
                            self.tail = self.tail.down();
                            next_direction = "DL";
                        }
                        _ => {
                            next_direction = "C";
                        }
                    }
                }
                "R" => { 
                    self.head = self.head.right();
                    match self.orientation {
                        Heading::East => {
                            self.tail = self.tail.right();
                            next_direction = "R";
                        }
                        Heading::NorthEast => {
                            self.tail = self.tail.right();
                            self.tail = self.tail.up();
                            next_direction = "UR";
                        }
                        Heading::SouthEast => {
                            self.tail = self.tail.right();
                            self.tail = self.tail.down();
                            next_direction = "DR";
                        }
                        _ => {
                            next_direction = "C";
                        }
                    }
                 }
                 "UL" => {
                    self.head = self.head.left();
                    self.head = self.head.up();
                    match self.orientation {
                        Heading::East | Heading::SouthEast | Heading::South | Heading::Centered => {
                            next_direction = "C";
                        }
                        Heading::NorthWest | Heading::West | Heading::North => {
                            self.tail = self.tail.up();
                            self.tail = self.tail.left();
                            next_direction = "UL";
                        }
                        Heading::SouthWest => {
                            self.tail = self.tail.left();
                            next_direction = "L";
                        }
                        Heading::NorthEast => {
                            self.tail = self.tail.up();
                            next_direction = "U";
                        }
                    }
                 }
                 "UR" => {
                    self.head = self.head.right();
                    self.head = self.head.up();
                    match self.orientation {
                        Heading::West | Heading::SouthWest | Heading::South | Heading::Centered => {
                            next_direction = "C";
                        }
                        Heading::NorthEast | Heading::East | Heading::North => {
                            self.tail = self.tail.up();
                            self.tail = self.tail.right();
                            next_direction = "UR";
                        }
                        Heading::SouthEast => {
                            self.tail = self.tail.right();
                            next_direction = "R";
                        }
                        Heading::NorthWest => {
                            self.tail = self.tail.up();
                            next_direction = "U";
                        }
                    }
                 }
                 "DL" => {
                    self.head = self.head.left();
                    self.head = self.head.down();
                    match self.orientation {
                        Heading::East | Heading::NorthEast | Heading::North | Heading::Centered => {
                            next_direction = "C";
                        }
                        Heading::SouthWest | Heading::West | Heading::South => {
                            self.tail = self.tail.down();
                            self.tail = self.tail.left();
                            next_direction = "DL";
                        }
                        Heading::NorthWest => {
                            self.tail = self.tail.left();
                            next_direction = "L";
                        }
                        Heading::SouthEast => {
                            self.tail = self.tail.down();
                            next_direction = "D";
                        }
                    }
                 }
                 "DR" => {
                    self.head = self.head.right();
                    self.head = self.head.down();
                    match self.orientation {
                        Heading::West | Heading::NorthWest | Heading::North | Heading::Centered => {
                            next_direction = "C";
                        }
                        Heading::SouthEast | Heading::East | Heading::South => {
                            self.tail = self.tail.down();
                            self.tail = self.tail.right();
                            next_direction = "DR";
                        }
                        Heading::NorthEast => {
                            self.tail = self.tail.right();
                            next_direction = "R";
                        }
                        Heading::SouthWest => {
                            self.tail = self.tail.down();
                            next_direction = "D";
                        }
                    }
                 }
                 "C" => {
                    next_direction = "C";
                 }
                 _ => {
                    next_direction = "C";
                    println!("error")
                }
            }
            self.update_orientation();
            next_direction
        }

        fn update_orientation(&mut self) {
            let (x_pos, y_pos);

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

    struct Rope {
        segments: Vec<Segment>
    }

    impl Rope {
        fn create(segments: u32) -> Rope {
            let mut rope = Rope {segments: Vec::new()};
            for _i in 0..segments {
                rope.segments.push(Segment {head: XY {x: 0, y: 0}, tail: XY {x: 0, y: 0}, orientation: Heading::Centered});
            }
            rope
        }

        fn step(&mut self, direction: &str, length: usize) {
            if length == 0 {
                return;
            } else {
                let next_direction = self.segments[length - 1].step(direction).to_owned();
                self.step(&next_direction, length - 1);
            }
        }
    }
    
    let mut tail_pos: Vec<XY> = Vec::new();
    let mut rope = Rope::create(9);

    if let Ok(lines) = read_lines("src\\input_09.txt") {
        println!("hello");
        for line in lines {
            if let Ok(ln) = line {
                let (direction, steps) = ln.split_once(" ").unwrap();
                for _i in 0..steps.parse::<i32>().unwrap() {
                    rope.step(direction, 9);
                    tail_pos.push(rope.segments[0].tail.clone());
                }
            }
        }
    }

    let total_unique = tail_pos.iter().collect::<HashSet<&XY>>().len();
    println!("{}", total_unique)
}
// =========== Main ========= //
fn main() {
    aoc_09();
}