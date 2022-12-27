use std::{str::FromStr, collections::HashSet};


fn main() {
    let input = include_str!("input.txt");
    let pt1_results = part1(input);
    println!("pt1: {}", pt1_results);
}

#[derive(Debug, PartialEq)]
struct Head {
    pos: (i32, i32),
    tail: Option<Box<Head>>
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    U,
    D,
    L,
    R
}

#[derive(Debug, PartialEq, Clone)]
struct Move(Direction, i32);

trait MoveListener {
    fn on_tail_move(&mut self, from: &(i32, i32), to: &(i32, i32));
}

struct NOOP { }

struct MoveTracker {
    tail_positions: HashSet<(i32, i32)>
}

impl Default for MoveTracker {
    fn default() -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert((0, 0));
        Self { tail_positions }
    }
}

impl MoveListener for MoveTracker {
    fn on_tail_move(&mut self, _: &(i32, i32), to: &(i32, i32)) {
        self.tail_positions.insert((to.0, to.1));
    }
}

impl MoveListener for NOOP {
    fn on_tail_move(&mut self, _: &(i32, i32), _: &(i32, i32)) {}
}

impl Head {
    fn apply<T>(
        &mut self, movement: Move, 
        move_listener: &mut T
    )
    where T: MoveListener 
    {
        for _ in 0..movement.1 {
            let old_head_position = self.pos.clone();
            let new_head_position = match movement.0 {
                Direction::U => {
                    (self.pos.0 + 1, self.pos.1)
                },
                Direction::D => {
                    (self.pos.0 - 1, self.pos.1)
                },
                Direction::L => {
                    (self.pos.0, self.pos.1 - 1)
                },
                Direction::R => {
                    (self.pos.0, self.pos.1 + 1)
                }
            };

            if let Some(t) = self.tail.as_mut() {
                let tail_deltas = (
                    t.pos.0 - new_head_position.0,
                    t.pos.1 - new_head_position.1
                );
                let max_delta = tail_deltas.0.abs().max(tail_deltas.1.abs());
                let new_tail_pos = if max_delta == 2 {
                    move_listener.on_tail_move(&t.pos, &old_head_position);
                    old_head_position
                } else {
                    t.pos
                };
        
                self.pos = new_head_position;
                t.pos = new_tail_pos;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParseMoveError {
    BadInput(String)
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(ParseMoveError::BadInput(s.to_string()));
        }
        let direction = match parts[0] {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err(ParseMoveError::BadInput(s.to_string()))
        }?;

        let qty = parts[1]
            .parse::<i32>()
            .map_err(|_|ParseMoveError::BadInput(s.to_string()))?;

        Ok(Move(direction, qty))
    }
}

#[test]
fn test_up_from_origin() {
    let mut initial = Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::U, 1), &mut NOOP {});

    let expected = Head {
        pos: (1, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}


#[test]
fn test_down_from_origin() {
    let mut initial = Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::D, 1), &mut NOOP {});

    let expected = Head {
        pos: (-1, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_right_from_origin() {
    let mut initial = Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::R, 1), &mut NOOP {});

    let expected = Head {
        pos: (0, 1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}


#[test]
fn test_left_from_origin() {
    let mut initial = Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::R, 1), &mut NOOP {});

    let expected = Head {
        pos: (0, 1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_up_from_same_horizontal() {
    let mut initial = Head {
        pos: (1, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::U, 1), &mut NOOP {});

    let expected = Head {
        pos: (2, 0),
        tail: Some(Box::new(Head { 
            pos: (1, 0),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_down_from_same_horizontal() {
    let mut initial = Head {
        pos: (-1, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::D, 1), &mut NOOP {});

    let expected = Head {
        pos: (-2, 0),
        tail: Some(Box::new(Head { 
            pos: (-1, 0),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_right_from_same_vertical() {
    let mut initial = Head {
        pos: (0, 1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::R, 1), &mut NOOP {});

    let expected = Head {
        pos: (0, 2),
        tail: Some(Box::new(Head { 
            pos: (0, 1),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_left_from_same_vertical() {
    let mut initial = Head {
        pos: (0, -1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::L, 1), &mut NOOP {});

    let expected = Head {
        pos: (0, -2),
        tail: Some(Box::new(Head { 
            pos: (0, -1),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_up_from_below_left_diagonal() {
    let mut initial = Head {
        pos: (1, 1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::U, 1), &mut NOOP {});

    let expected = Head {
        pos: (2, 1),
        tail: Some(Box::new(Head { 
            pos: (1, 1),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_down_from_above_right_diagonal() {
    let mut initial = Head {
        pos: (-1, -1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::D, 1), &mut NOOP {});

    let expected = Head {
        pos: (-2, -1),
        tail: Some(Box::new(Head { 
            pos: (-1, -1),
            tail: None
        }))
    };

    assert_eq!(initial, expected);
}

#[test]
fn test_parse_move() {
    assert_eq!("R 2".parse(), Ok(Move(Direction::R, 2)));
    assert_eq!("L 1".parse(), Ok(Move(Direction::L, 1)));
    assert_eq!("U 10".parse(), Ok(Move(Direction::U, 10)));
    assert_eq!("D 5".parse(), Ok(Move(Direction::D, 5)));
}

fn part1(input: &str) -> usize {
    let mut head = Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };
    let mut move_listener = MoveTracker::default();
    input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|item| head.apply(item, &mut move_listener));

    move_listener.tail_positions.len()
}

#[test]
fn test_pt1_example() {
    let input = include_str!("input.example.txt");
    assert_eq!(part1(input), 13);
}