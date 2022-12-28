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

impl Direction {
    fn is_vertical(&self) -> bool {
        match self {
            Direction::U => true,
            Direction::D => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::L => true,
            Direction::R => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Move(Direction, i32);

impl Move {
    fn to_cartesian(&self) -> (i32, i32) {
        match self.0 {
            Direction::U => (self.1, 0),
            Direction::D => (-self.1, 0),
            Direction::R => (0, 1),
            Direction::L => (0, -1),
        }
    }
}

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
        let amount = movement.1;

        for _ in 0..amount {
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
            self.update_to(new_head_position, move_listener);            
        }        
    }

    fn update_to<T>(
        &mut self,
        new_head_position: (i32, i32), 
        move_listener: &mut T) 
        where T: MoveListener
        {
            let old_position = self.pos.clone();
            self.pos = new_head_position;
            if let Some(t) = self.tail.as_mut() {
                let move_deltas = (
                    new_head_position.0 - old_position.0,
                    new_head_position.1 - old_position.1
                );
                let new_head_vs_old_tail = (
                    new_head_position.0 - t.pos.0,
                    new_head_position.1 - t.pos.1
                );

                let new_row_delta = (t.pos.0 - new_head_position.0).abs();
                let new_col_delta = (t.pos.1 - new_head_position.1).abs();
                let distance = new_row_delta + new_col_delta;

                if distance <= 2 && new_row_delta.max(new_col_delta) <= 1 {
                    // still touching
                    return;
                }

                let new_tail_pos = if t.pos.0 == new_head_position.0 {
                    // still same height so just move width
                    println!("same height {:?}", t.pos);
                    (t.pos.0, t.pos.1 + move_deltas.1.signum())
                } else if t.pos.1 == new_head_position.1 {
                    // still same width so just move vertically
                    println!("same width {:?}", t.pos);
                    (t.pos.0 + move_deltas.0.signum(), t.pos.1)
                } else {
                    println!("diagonal {:?} by {:?}", t.pos, new_head_vs_old_tail);
                    (t.pos.0 + new_head_vs_old_tail.0.signum(), t.pos.1 + new_head_vs_old_tail.1.signum())
                };
                t.update_to(new_tail_pos, move_listener);
            } else {
                move_listener.on_tail_move(&old_position, &new_head_position);
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
fn test_left_many_from_origin() {
    let mut initial = Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::R, 5), &mut NOOP {});

    let expected = Head {
        pos: (0, 5),
        tail: Some(Box::new(Head { 
            pos: (0, 4),
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
    // .....
    // .H...
    // T....
    //--to--
    // .H...
    // .T...
    // .....    
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
    // ....T
    // ...H.
    // .....
    //  to
    // .....
    // ...T.
    // ...H.
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
fn test_move_up_from_right() {
    // .....
    // .....
    // TH...
    //  to
    // .....
    // .H...
    // T....
    let mut initial = Head {
        pos: (0, 1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
            tail: None
        }))
    };

    initial.apply(Move(Direction::U, 1), &mut NOOP {});

    let expected = Head {
        pos: (1, 1),
        tail: Some(Box::new(Head { 
            pos: (0, 0),
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

fn part2(input: &str) -> usize {
    let mut head = depth(10).unwrap();
    let mut move_listener = MoveTracker::default();
    input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|item| head.apply(item, &mut move_listener));

    move_listener.tail_positions.len()
}

fn depth(n: i32) -> Option<Head> {
    if n == 0 {
        None
    } else {
        Some(Head { 
            pos: (0, 0), 
            tail: depth(n - 1).map(|next| Box::new(next)) 
        })
    }
}

#[test]
fn test_pt1_example_m1() {
    let mut h = depth(2).unwrap();
    let mut ml = MoveTracker::default();

    let m: Move = "R 4".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "U 4".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "L 3".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "D 1".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "R 4".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "D 1".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "L 5".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);

    let m: Move = "R 2".parse().unwrap();
    h.apply(
        m.clone(), 
        &mut ml
    );
    println!("positions after {:?}:\n\t{:?}", m, h);
    print_seen(10, 10, &ml.tail_positions);
}

fn print_seen(w: i32, h: i32, seen: &HashSet<(i32, i32)>) {
    for i in (-h..h).rev() {
        for j in -w..w {
            if (0,0) == (i, j) {
                print!("s");
            } else if seen.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }            
        }
        println!("");
    }
}

fn print_trail(w: i32, h: i32, head: &Head, seen: &HashSet<(i32, i32)>) {
    for i in (-h..h).rev() {
        for j in -w..w {
            let mut depth = 0;
            let mut current_head = head;
            let mut found = false;
            loop {
                if current_head.pos == (i, j) {
                    if depth == 0 {
                        print!("H")
                    } else {
                        print!("{}", depth);
                    }
                    found = true;
                    break;
                }
                if let Some(h) = &current_head.tail {
                    depth = depth + 1;
                    current_head = h.as_ref();
                } else {
                    break;
                }
            }
            if !found {
                if (0,0) == (i, j) {
                    print!("s");
                } else if seen.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }            
        }
        println!("");
    }
}

#[test]
fn test_pt1_example() {
    let input = include_str!("input.example.txt");
    assert_eq!(part1(input), 13);
}

#[test]
fn depth_n() {
    let result = depth(2).unwrap();

    assert_eq!(result, Head {
        pos: (0, 0),
        tail: Some(Box::new(Head { 
            pos: (0, 0), 
            tail: None 
        }))
    })
}

fn debug_step(
    move_line: &str, 
    head: &mut Head, 
    move_listener: &mut MoveTracker
)
{
    let m = move_line.parse::<Move>().unwrap();
    head.apply(m.clone(), move_listener);
    println!("positions after {:?}", m);
    print_trail(6, 6, &head, &move_listener.tail_positions);
    println!("");
    println!("");
}

#[test]
fn test_smaller_pt2_example() {
    let mut head = depth(10).unwrap();
    let mut move_listener = MoveTracker::default();

    debug_step("R 1", &mut head, &mut move_listener);
    debug_step("R 1", &mut head, &mut move_listener);
    debug_step("R 1", &mut head, &mut move_listener);
    debug_step("R 1", &mut head, &mut move_listener);

    debug_step("U 1", &mut head, &mut move_listener);
    debug_step("U 1", &mut head, &mut move_listener);
    debug_step("U 1", &mut head, &mut move_listener);
    debug_step("U 1", &mut head, &mut move_listener);
}

#[test]
fn test_pt2_example_steps() {
    let mut head = depth(10).unwrap();
    let mut move_listener = MoveTracker::default();

    let m = "R 5".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);

    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
    
    let m = "U 1".parse::<Move>().unwrap();
    head.apply(m.clone(), &mut move_listener);
    println!("positions after {:?}", m);
    print_trail(10, 10, &head, &move_listener.tail_positions);
}

#[test]
fn test_pt2_example() {
    assert_eq!(part2(include_str!("input.example2.txt")), 36);
}