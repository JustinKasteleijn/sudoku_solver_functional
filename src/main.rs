fn main() {
    let mut sudoku: [[usize; 9]; 9] = [
        [0, 0, 3, 0, 2, 0, 6, 0, 0],
        [9, 0, 0, 3, 0, 5, 0, 0, 1],
        [0, 0, 1, 8, 0, 6, 4, 0, 0],
        [0, 0, 8, 1, 0, 2, 9, 0, 0],
        [7, 0, 0, 0, 0, 0, 0, 0, 8],
        [0, 0, 6, 7, 0, 8, 2, 0, 0],
        [0, 0, 2, 6, 0, 9, 5, 0, 0],
        [8, 0, 0, 2, 0, 3, 0, 0, 9],
        [0, 5, 0, 0, 1, 0, 3, 0, 0]
    ];

    println!("{}", no_conflict(&sudoku, 0, 0, 4));
}

fn no_conflict(grid: &[[usize; 9]; 9], row: usize, col: usize, val: usize) -> bool {
    let (r, c) = ((row / 3) * 3, (col / 3) * 3);
    let matrix_check: bool = (r..r + 3).all(|x: usize| (c..c + 3).all(|y: usize| grid[x][y] != val));
    let row_col_check: bool = (0..9).all(|x: usize| (grid[x][col] != val) && (grid[row][x] != val));
    matrix_check && row_col_check
}
