//=======
// This is totally unfinished. I got sidetracked and decided to draw shapes in
// matrices instead of solving the problem. Oops.
//=======

use na::{Dynamic, MatrixVec, Matrix};
use nom::types::{CompleteStr};

type DMatrixVeci32 = MatrixVec<i32, Dynamic, Dynamic>;
type DMatrixi32 = Matrix<i32, Dynamic, Dynamic, DMatrixVeci32>;

trait Draw {
    fn draw_rect<F>(
        &self,
        xpos: usize,
        ypos: usize,
        width: usize,
        height: usize,
        func: F
    ) -> DMatrixi32 where F: Fn(i32) -> i32;
}

impl Draw for DMatrixi32 {
    fn draw_rect<F>(
        &self,
        xpos: usize,
        ypos: usize,
        width: usize,
        height: usize,
        func: F
    ) -> DMatrixi32 where F: Fn(i32) -> i32 {
        self.map_with_location(|y, x, v| {
            let in_x_bounds = x >= xpos && x <= xpos + width - 1;
            let in_y_bounds = y >= ypos && y <= xpos + height - 1;
            if in_x_bounds && in_y_bounds { func(v) } else { v }
        })
    }
}



struct ShapeCoords {
    id: i32,
    xpos: usize,
    ypos: usize,
    width: usize,
    height: usize
}

fn i32_of_str(input: CompleteStr) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(&input, 10)
}

fn usize_of_str(input: CompleteStr) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(&input, 10)
}

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

named!(parse_i32<CompleteStr, i32>, map_res!(take_while!(is_digit), i32_of_str));
named!(parse_usize<CompleteStr, usize>, map_res!(take_while!(is_digit), usize_of_str));

named!(parse_shape_coords<CompleteStr, ShapeCoords>,
  do_parse!(
            tag!("#")   >>
    id:     parse_i32   >>
            tag!(" @ ") >>
    xpos:   parse_usize >>
            tag!(",")   >>
    ypos:   parse_usize >>
            tag!(": ")  >>
    width:  parse_usize >>
            tag!("x")   >>
    height: parse_usize >>
    (ShapeCoords { id, xpos, ypos, width: 0, height: 0 })
  )
);

pub fn main() {
    let (
        _, ShapeCoords { xpos, ypos, .. }
    ) = parse_shape_coords(CompleteStr("#1 @ 850,301: 23x12")).unwrap();
    println!("{}, {}", xpos, ypos);
    // let mut matrix = DMatrixi32::zeros(10,10);
    // for i in 0..8 {
    //     matrix = matrix.draw_rect(i, i, 3, 3, |v| v + 1)
    // }
    // println!("{}", matrix);
    // =>
    // ┌                     ┐
    // │ 1 1 1 0 0 0 0 0 0 0 │
    // │ 1 2 2 1 0 0 0 0 0 0 │
    // │ 1 2 3 2 1 0 0 0 0 0 │
    // │ 0 1 2 3 2 1 0 0 0 0 │
    // │ 0 0 1 2 3 2 1 0 0 0 │
    // │ 0 0 0 1 2 3 2 1 0 0 │
    // │ 0 0 0 0 1 2 3 2 1 0 │
    // │ 0 0 0 0 0 1 2 3 2 1 │
    // │ 0 0 0 0 0 0 1 2 2 1 │
    // │ 0 0 0 0 0 0 0 1 1 1 │
    // └                     ┘
}

