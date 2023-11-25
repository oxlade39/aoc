use std::time::Instant;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.txt");

    let player_one = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    let player_two = input
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;

    let mut rolls = 0;
    let mut scores: [i64; 2] = [0, 0];
    let mut positions: [i64; 2] = [player_one, player_two];
    println!("starting on {:?}", positions);
    loop {
        rolls += 3;
        let max_die = ((rolls - 1) % 100) + 1;
        let roll_one_score = max_die + (max_die - 1) + (max_die - 2);
        positions[0] = (((positions[0] - 1) + roll_one_score) % 10) + 1;
        scores[0] += positions[0];
        println!("1 to pos. {} with score: {}", positions[0], roll_one_score);

        if scores[0] >= 1000 {
            break;
        }

        rolls += 3;
        let max_die = ((rolls - 1) % 100) + 1;
        let roll_two_score = max_die + (max_die - 1) + (max_die - 2);
        positions[1] = (((positions[1] - 1) + roll_two_score) % 10) + 1;
        scores[1] += positions[1];
        println!("2 to pos. {} with score: {}", positions[1], roll_two_score);

        if scores[1] >= 1000 {
            break;
        }

        println!("2 to {}", positions[1]);
        println!("rolls: {}: {:?}", rolls, scores);
    }

    println!("rolls: {}", rolls);
    let min_score = i64::min(scores[0], scores[1]);
    println!("min score: {}", min_score);
    println!("pt1: {}", min_score * rolls);
}

fn part2() {
    let input = include_str!("input.txt");
    let player_one = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;
    let player_two = input
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;

    let results = turn(true, [0, 0], [player_one, player_two], 1, 21);
    println!("pt2: {:?}", i64::max(results[0], results[1]));
}

fn turn(
    is_left_turn: bool,
    scores: [i32; 2],
    positions: [i32; 2],
    universes: i64,
    target: i32,
) -> [i64; 2] {
    if scores[0] >= target {
        [universes, 0]
    } else if scores[1] >= target {
        [0, universes]
    } else {
        // there are 7 possible unique sums from 3 3 sided dice, summing to 27 possible dice combination
        let sums = [
            1, // 1 way to make 3
            3, // 4
            6, // 5
            7, // 6
            6, // 7
            3, // 8
            1, // 9
        ];
        if is_left_turn {
            let mut children = [0, 0];
            for (i, n_left_rolls) in sums.iter().enumerate() {
                let left_dice_sum = (i + 3) as i32;
                let next_left_position = (((positions[0] - 1) + left_dice_sum) % 10) + 1;
                let next_left_score = scores[0] + next_left_position;
                let child = turn(
                    false,
                    [next_left_score, scores[1]],
                    [next_left_position, positions[1]],
                    universes * n_left_rolls,
                    target,
                );
                children[0] += child[0];
                children[1] += child[1];
            }
            children
        } else {
            let mut children = [0, 0];
            for (i, n_right_rolls) in sums.iter().enumerate() {
                let right_dice_sum = (i + 3) as i32;
                let next_right_position = (((positions[1] - 1) + right_dice_sum) % 10) + 1;
                let next_right_score = scores[1] + next_right_position;
                let child = turn(
                    true,
                    [scores[0], next_right_score],
                    [positions[0], next_right_position],
                    universes * n_right_rolls,
                    target,
                );
                children[0] += child[0];
                children[1] += child[1];
            }
            children
        }
    }
}

#[test]
fn test_pt2_example() {
    let result = turn(true, [0, 0], [4, 8], 1, 21);
    assert_eq!(result, [444356092776315, 341960390180808]);
}
