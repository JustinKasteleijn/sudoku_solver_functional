pub fn print_sudoku(grid: &[[usize; 9]; 9]) {
    grid.iter().for_each(|row| println!("{:?}", row));
}

pub fn solve_sudoku(grid: &mut [[usize; 9]; 9]) -> bool {
    let (row, col) = find_empty_cell(&grid);
    if row == 9 {
        return true;
    }

    for n in 1..10 {
        if no_conflict(grid, row, col, n) {
            grid[row][col] = n;
            if solve_sudoku(grid) {
                return true;
            }
            grid[row][col] = 0
        }
    }

    false
}

pub fn no_conflict(grid: &[[usize; 9]; 9], row: usize, col: usize, val: usize) -> bool {
    let (r, c) = ((row / 3) * 3, (col / 3) * 3);
    let matrix_check: bool = (r..r + 3).all(|x: usize| (c..c + 3).all(|y: usize| grid[x][y] != val));
    let row_col_check: bool = (0..9).all(|x: usize| (grid[x][col] != val) && (grid[row][x] != val));
    matrix_check && row_col_check
}

pub fn find_empty_cell(grid: &[[usize; 9]; 9]) -> (usize, usize) {
    for row in 0..9 {
        for col in 0..9 {
            if grid[row][col] == 0 {
                return (row, col);
            }
        }
    }
    (9, 9)
}

// pub fn find_empty_cell(grid: &[[usize; 9]; 9]) -> (usize, usize) {
//     let pos: usize = grid.into_iter()
//         .flatten()
//         .position(|x| x == &0)
//         .unwrap_or(82);
//     (pos / 9, pos % 9)
// }

pub fn test_sudoku_solver(grid: &[[usize; 9]; 9]) -> bool {
    let result: [[usize; 9]; 9] = [
        [4, 8, 3, 9, 2, 1, 6, 5, 7],
        [9, 6, 7, 3, 4, 5, 8, 2, 1],
        [2, 5, 1, 8, 7, 6, 4, 9, 3],
        [5, 4, 8, 1, 3, 2, 9, 7, 6],
        [7, 2, 9, 5, 6, 4, 1, 3, 8],
        [1, 3, 6, 7, 9, 8, 2, 4, 5],
        [3, 7, 2, 6, 8, 9, 5, 1, 4],
        [8, 1, 4, 2, 5, 3, 7, 6, 9],
        [6, 9, 5, 4, 1, 7, 3, 8, 2]
    ];

    result.iter()
        .zip(grid.iter())
        .all(|(x, y)| x == y)
}
