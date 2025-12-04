// we have a dial, with numbers 0 to 99
// going back from 0, 1 step will take you to 99
// going from 90, 15 steps will take you to 4
// the dial starts at 50
// each line of the input has L or R as the first char, with number of moves from 0 to 999
// we need to keep track of the amount of times that we reach 0 whilst turning the dial.

use std::fs;

fn move_dial_part_2(mut dial: i32, direction: char, mut steps: i32, mut hits: i32) -> (i32, i32) {
    let current_dial = dial;
    let current_steps = steps;
    let mut crossed_zero = false;
    let mut new_hits = 0;

    // if a number of steps is larger than 99 in either direction
    // we can ignore these full rotations (100 steps for 1 full rotation)
    if steps > 99 {
        // we're spinning past 0 here if we move 100
        new_hits += steps / 100;
        // with whatevers left, we might cross or land on 0 again
        steps = steps % 100;
    }

    // now match direction and go forwards and backwards on the dial
    match direction {
        'L' => {
            // println!("left | steps: {} dial: {}", steps, dial);
            // check if we are crossing the 0 mark on the dial
            if dial - steps < 0 {
                dial -= steps;
                dial += 100;
                crossed_zero = true;
                if current_dial != 0 {
                    new_hits += 1;
                }
            } else {
                dial -= steps;
            }
        }
        'R' => {
            // println!("right | steps: {} dial: {}", steps, dial);
            // check if we are crossing the 0 mark on the dial
            if dial + steps > 99 {
                dial += steps;
                dial -= 100; // we crossed it!
                crossed_zero = true;
                if current_dial != 0 {
                    new_hits += 1;
                }
            } else {
                dial += steps;
            }
        }
        _ => {}
    }

    // we moved and now the dial is on 0, we get to count a hit.
    if !crossed_zero && dial == 0 {
        new_hits += 1;
    }

    hits += new_hits;

    println!(
        "{} | dial: {} steps: {} hits: {}",
        direction, current_dial, current_steps, new_hits
    );

    return (dial, hits);
}

fn move_dial(mut dial: i32, direction: char, mut steps: i32, mut hits: i32) -> (i32, i32) {
    // if a number of steps is larger than 99 in either direction
    // we can ignore these full rotations (100 steps for 1 full rotation)
    if steps > 99 {
        steps = steps % 100
    }
    // now match direction and go forwards and backwards on the dial
    match direction {
        'L' => {
            // println!("left | steps: {} dial: {}", steps, dial);
            // check if we are crossing the 0 mark on the dial
            if dial - steps < 0 {
                dial -= steps;
                dial += 100;
            } else {
                dial -= steps;
            }
        }
        'R' => {
            // println!("right | steps: {} dial: {}", steps, dial);
            // check if we are crossing the 0 mark on the dial
            if dial + steps > 99 {
                dial += steps;
                dial -= 100; // we crossed it!
            } else {
                dial += steps;
            }
        }
        _ => {}
    }

    if dial == 0 {
        hits += 1;
    }

    return (dial, hits);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // dial starts at 50
    let mut dial_part1 = 50;
    let mut dial_part2 = 50;

    // count the number of times we hit 0
    let mut zero_hits_part1 = 0;
    let mut zero_hits_part2 = 0;

    // iterate lines in file
    contents.split('\n').for_each(|line| {
        // iterate line chars
        let mut line_chars = line.chars();

        // direction is L or R first char of line
        let direction = line_chars.nth(0).unwrap();

        // steps is the rest of the chars from 0 to 999
        let steps: i32 = line_chars.as_str().parse().unwrap();

        // move the dial
        let part1 = move_dial(dial_part1, direction, steps, zero_hits_part1);
        dial_part1 = part1.0;
        zero_hits_part1 = part1.1;

        let part2 = move_dial_part_2(dial_part2, direction, steps, zero_hits_part2);
        dial_part2 = part2.0;
        zero_hits_part2 = part2.1;
    });

    println!("password 1 is: {}", zero_hits_part1);
    println!("password 2 is: {}", zero_hits_part2);
}
