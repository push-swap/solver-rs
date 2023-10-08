mod strategy;
mod stream;

use std::{
    env,
    io::{self, Read},
};
use stream::{Operation, Stream};

const SXS_0: &[u8] = include_bytes!("../data/sxs_0.bin");
const SXS_1: &[u8] = include_bytes!("../data/sxs_1.bin");
const SXS_2: &[u8] = include_bytes!("../data/sxs_2.bin");
const SXS_3: &[u8] = include_bytes!("../data/sxs_3.bin");
const SXS_4: &[u8] = include_bytes!("../data/sxs_4.bin");
const SXS_5: &[u8] = include_bytes!("../data/sxs_5.bin");
const SXS_6: &[u8] = include_bytes!("../data/sxs_6.bin");
const SXS_7: &[u8] = include_bytes!("../data/sxs_7.bin");
const SXS_8: &[u8] = include_bytes!("../data/sxs_8.bin");
const SXS_9: &[u8] = include_bytes!("../data/sxs_9.bin");
const SXS_10: &[u8] = include_bytes!("../data/sxs_10.bin");

fn factorial(n: usize) -> usize {
    if n > 0 {
        n * factorial(n - 1)
    } else {
        1
    }
}

fn ps_hardcoded_find_index(arr: &[i32], len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    let mut result = 0;
    for i in 0..len {
        if arr[0] > arr[i] {
            result += 1;
        }
    }

    (result * factorial(len - 1)) + ps_hardcoded_find_index(&arr[1..], len - 1)
}

fn load_precomputed_optimal_solution_common(
    data: &[u8],
    index: usize,
    length: usize,
) -> Vec<Operation> {
    let mut reader = io::Cursor::new(data);
    reader.set_position(4 * index as u64);
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).unwrap();
    let offset = u32::from_le_bytes(buf);
    let offset_part_size = 4 * factorial(length);
    reader.set_position(offset_part_size as u64 + offset as u64);
    let mut result = Vec::new();
    loop {
        let mut byte = [0];
        reader.read_exact(&mut byte).unwrap();
        if byte[0] == 11 {
            break; // Stop when we encounter 11
        }
        let operation = match byte[0] {
            0 => Operation::PA,
            1 => Operation::PB,
            2 => Operation::SA,
            3 => Operation::SB,
            4 => Operation::SS,
            5 => Operation::RA,
            6 => Operation::RB,
            7 => Operation::RR,
            8 => Operation::RRA,
            9 => Operation::RRB,
            10 => Operation::RRR,
            _ => panic!("Invalid data: {}", byte[0]),
        };
        result.push(operation);
    }
    result
}

fn load_precomputed_optimal_solution_sxs(input: &Vec<i32>) -> Vec<Operation> {
    let data = match input.len() {
        0 => SXS_0,
        1 => SXS_1,
        2 => SXS_2,
        3 => SXS_3,
        4 => SXS_4,
        5 => SXS_5,
        6 => SXS_6,
        7 => SXS_7,
        8 => SXS_8,
        9 => SXS_9,
        10 => SXS_10,
        _ => panic!("Error"),
    };
    load_precomputed_optimal_solution_common(
        data,
        ps_hardcoded_find_index(&input, input.len()),
        input.len(),
    )
}

fn solve(stream: &mut Stream, input: Vec<i32>) {
    if input.len() <= 10 {
        for op in load_precomputed_optimal_solution_sxs(&input) {
            stream.add(op);
        }
    } else {
        // TODO: implement
        panic!("not implemented yet");
    }
}

fn has_duplicates<T: PartialEq>(vec: &[T]) -> bool {
    for (i, item) in vec.iter().enumerate() {
        if vec.iter().skip(i + 1).any(|x| x == item) {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<i32> = env::args()
        .skip(1)
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse arguments as i32");
    if has_duplicates(&args) {
        panic!("duplicate arguments given")
    }
    let mut stream = Stream::new();
    solve(&mut stream, args);
    stream.print(&mut io::stdout()).expect("Failed to print");
}
