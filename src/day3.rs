extern crate nalgebra as na;

use self::na::{Dynamic, MatrixVec, Matrix};

type DMatrixi32 = Matrix<i32, Dynamic, Dynamic, MatrixVec<i32, Dynamic, Dynamic>>;

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


pub fn main() {
    let mut matrix = DMatrixi32::zeros(10,10);
    for i in 0..8 {
        matrix = matrix.draw_rect(i, i, 3, 3, |v| v + 1)
    }
    println!("{}", matrix);
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
