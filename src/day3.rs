/// Advent of Code Day 3
/// https://adventofcode.com/2018/day/3
use na::{Dynamic, MatrixVec, Matrix};
use nom::types::{CompleteStr};

type DMatrixVeci32 = MatrixVec<i32, Dynamic, Dynamic>;
type DMatrixi32 = Matrix<i32, Dynamic, Dynamic, DMatrixVeci32>;

struct ParsedSlice {
    id:     i32,
    xpos:   usize,
    ypos:   usize,
    width:  usize,
    height: usize
}

fn is_digit(c: char) -> bool { c.is_digit(10) }

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

named!(parse_slice<CompleteStr, ParsedSlice>,
    do_parse!(
        tag!("#")           >> id:     parse_i32   >>
        whitespace          >>
        tag!("@")           >>
        whitespace          >> xpos:   parse_usize >>
        tag!(",")           >> ypos:   parse_usize >>
        tag!(":")           >>
        whitespace          >> width:  parse_usize >>
        tag!("x")           >> height: parse_usize >>
        (ParsedSlice { id, xpos, ypos, width, height })
    )
);

fn parse_slices(text: &str) -> Vec<ParsedSlice> {
    text
        .lines()
        .filter_map(|line| parse_slice(CompleteStr(line)).ok())
        .map(|(_, pslice)| pslice)
        .collect::<Vec<ParsedSlice>>()

}

fn inc_matrix_slice(matrix: &mut DMatrixi32, pslice: &ParsedSlice) {
    let ParsedSlice { xpos, ypos, width, height, .. } = *pslice;
    matrix.slice_mut((xpos, ypos), (width, height)).apply(|v| v + 1);
}

fn matrix_slice_all_eq(matrix: &DMatrixi32, pslice: &ParsedSlice, i: i32) -> bool {
    let ParsedSlice { xpos, ypos, width, height, .. } = *pslice;
    matrix.slice((xpos, ypos), (width, height)).iter().all(|&v| v == i)
}

pub fn main() {
    let text = include_str!("../resources/day3.txt");
    let mut matrix = DMatrixi32::zeros(1000,1000);
    let parsed_slices = parse_slices(text);
    for pslice in parsed_slices.iter() {
        inc_matrix_slice(&mut matrix, pslice)
    }
    let overlaps = matrix.iter().filter(|&&x| x > 1).count();
    println!("Overlapping cells: {}", overlaps);
    let no_overlap = parsed_slices.iter().find_map(|pslice| {
        if matrix_slice_all_eq(&matrix, pslice, 1) {
            Some(pslice.id)
        } else {
            None
        }
    });
    match no_overlap {
        Some(id) => println!("Slice ID without overlap: {}", id),
        None     => println!("No slice found without overlap")
    }
}
