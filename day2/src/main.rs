use std::fs;

// you can find the invalid IDs by looking for any ID which is made only
// of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice),
// and 123123 (123 twice) would all be invalid IDs.
fn part1(input_string: &str) -> usize {
    let mut invalid_ids: Vec<usize> = vec![];

    input_string.split(",").for_each(|id_range| {
        let (start, end) = id_range.split_once("-").unwrap();

        let mut start_int: usize = start.parse().unwrap();
        let end_int: usize = end.parse().unwrap();

        let start_len = start.len();
        let end_len = end.len();
        let mid_len = end_len / 2;

        // no possible ranges less than 10
        if end_int < 10 {
            return;
        }

        // this means we can ignore any ranges that are only odd in length, e.g. 100 to 999
        if start_len % 2 > 0 && end_len % 2 > 0 {
            // println!("skipped range, {} {}", start_int, end_int);
            return;
        }

        // if an end range is even but its starting range is odd,
        // the starting range can be bumped to the start of the next
        // even range
        if start_len % 2 > 0 {
            // println!("odd start range! start {}", start);
            // start_int = 10_usize.pow(start_len);
            start_int = 10;
        }

        for x in start_int..end_int {
            let x_str = x.to_string();
            let x_split = x_str.split_at(mid_len);
            if x_split.0 == x_split.1 {
                // println!("match! x={} x.1:{} == x.2:{}", x, x_split.0, x_split.1);
                invalid_ids.push(x);
            }
        }
    });

    invalid_ids.iter().sum()
}

fn part2(input_string: &str) -> usize {
    let mut invalid_ids: Vec<usize> = vec![];

    input_string.split(",").for_each(|id_range| {
        let (start, end) = id_range.split_once("-").unwrap();

        let mut start_int: usize = start.parse().unwrap();
        let end_int: usize = end.parse().unwrap();

        let start_len = start.len();
        let end_len = end.len();
        let mid_len = end_len / 2;

        // no possible ranges less than 10
        if end_int < 10 {
            return;
        }

        if start_int < 10 {
            start_int = 10;
        }

        // get the chunk lengths
        let mut chunk_lengths: Vec<usize> = vec![];
        for i in 1..mid_len + 1 {
            if end_len % i == 0 || start_len % i == 0 {
                chunk_lengths.push(i);
            }
        }
        // println!(
        //     "range {} to {} - group check {:?}",
        //     start, end, chunk_lengths
        // );

        for x in start_int..end_int {
            let x_str = x.to_string();

            // now check if number is divisible by i
            for cl in &chunk_lengths {
                let x_split = x_str.chars().collect::<Vec<char>>();
                let x_chunks: Vec<&[char]> = x_split.chunks(*cl).collect();
                let first_chunk = x_chunks[0];
                let invalid = x_chunks.iter().all(|a| *a == first_chunk);
                if invalid {
                    // println!("invalid id {} - chunk size: {} - {:?}", x, *cl, x_chunks);
                    invalid_ids.push(x);
                    break;
                }
            }
        }
    });

    invalid_ids.iter().sum()
}

fn main() {
    let input_string =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let part1_result = part1(&input_string);
    let part2_result = part2(&input_string);

    println!("part1: {}", part1_result);

    //73,837,149,171 high
    //73,694,270,733 not right
    //73,694,270,688 correct!
    //73,694,194,377 not right?
    //73,694,194,332 low
    println!("part2: {}", part2_result);
}
