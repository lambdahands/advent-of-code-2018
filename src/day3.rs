/// Advent of Code Day 3
/// https://adventofcode.com/2018/day/3
use na::{Dynamic, Matrix, MatrixVec};
use nom::types::CompleteStr;
use nom::*;

type DMatrixVeci32 = MatrixVec<i32, Dynamic, Dynamic>;
type DMatrixi32 = Matrix<i32, Dynamic, Dynamic, DMatrixVeci32>;

struct Claim {
    id: i32,
    coords: (usize, usize),
    shape: (usize, usize),
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(parse_i32<CompleteStr, i32>,
    map_res!(take_while1!(is_digit), |input: CompleteStr| {
        i32::from_str_radix(&input, 10)
    })
);

named!(parse_usize<CompleteStr, usize>,
    map_res!(take_while1!(is_digit), |input: CompleteStr| {
        usize::from_str_radix(&input, 10)
    })
);

named!(
    whitespace<CompleteStr, CompleteStr>,
    take_while!(|c: char| c.is_whitespace())
);

named!(parse_slice<CompleteStr, Claim>,
    do_parse!(
        tag!("#")  >> id:     parse_i32   >>
        whitespace                        >>
        tag!("@")                         >>
        whitespace >> xpos:   parse_usize >>
        tag!(",")  >> ypos:   parse_usize >>
        tag!(":")                         >>
        whitespace >> width:  parse_usize >>
        tag!("x")  >> height: parse_usize >>
        (Claim { id, coords:(xpos, ypos), shape:(width, height) })
    )
);

fn parse_slices(text: &str) -> Vec<Claim> {
    text.lines()
        .filter_map(|line| parse_slice(CompleteStr(line)).ok())
        .map(|(_, pslice)| pslice)
        .collect::<Vec<Claim>>()
}

fn inc_matrix_slice(matrix: &mut DMatrixi32, pslice: &Claim) {
    matrix
        .slice_mut(pslice.coords, pslice.shape)
        .apply(|v| v + 1);
}

fn matrix_slice_all_eq(matrix: &DMatrixi32, pslice: &Claim, i: i32) -> bool {
    matrix
        .slice(pslice.coords, pslice.shape)
        .iter()
        .all(|&v| v == i)
}

pub fn main() {
    let text = include_str!("../resources/day3.txt");
    let mut matrix = DMatrixi32::zeros(1000, 1000);
    let parsed_slices = parse_slices(text);
    for pslice in parsed_slices.iter() {
        inc_matrix_slice(&mut matrix, pslice)
    }
    // Part 1
    let overlaps = matrix.iter().filter(|&&x| x > 1).count();
    println!("Overlapping cells: {}", overlaps);
    // Part 2
    let no_overlap = parsed_slices.iter().find_map(|pslice| {
        if matrix_slice_all_eq(&matrix, pslice, 1) {
            Some(pslice.id)
        } else {
            None
        }
    });
    match no_overlap {
        Some(id) => println!("Slice ID without overlap: {}", id),
        None => println!("No slice found without overlap"),
    }
}
