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

named!(parse_shape_coords<CompleteStr, ParsedSlice>,
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

fn inc_matrix_slice(matrix: &mut DMatrixi32, shape_coords: ParsedSlice) {
    let ParsedSlice { xpos, ypos, width, height, .. } = shape_coords;
    matrix.slice_mut((xpos, ypos), (width, height)).apply(|v| v + 1);
}

pub fn main() {
    let text = include_str!("../resources/day3.txt");
    let mut matrix = DMatrixi32::zeros(1000,1000);
    for line in text.lines() {
        parse_shape_coords(CompleteStr(line))
            .map(|(_, pslice)| inc_matrix_slice(&mut matrix, pslice))
            .ok();
    }
    let result = matrix.iter().filter(|&&x| x > 1).count();
    println!("{}", result);
}
