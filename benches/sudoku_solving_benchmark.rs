use sudoku_solver_functional::{
    solve_sudoku,
    find_empty_cell,
    no_conflict,
};

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

fn solve_sudoku_benchmark(c: &mut Criterion) -> &mut Criterion {
    let mut sudoku: [[usize; 9]; 9] = black_box(
        [
            [0, 0, 3, 0, 2, 0, 6, 0, 0],
            [9, 0, 0, 3, 0, 5, 0, 0, 1],
            [0, 0, 1, 8, 0, 6, 4, 0, 0],
            [0, 0, 8, 1, 0, 2, 9, 0, 0],
            [7, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 0, 6, 7, 0, 8, 2, 0, 0],
            [0, 0, 2, 6, 0, 9, 5, 0, 0],
            [8, 0, 0, 2, 0, 3, 0, 0, 9],
            [0, 0, 5, 0, 1, 0, 3, 0, 0]
        ]
    );
    c.bench_function(
        "solver",
        |b| b.iter(|| solve_sudoku(&mut sudoku)),
    )
}

fn find_empty_cell_benchmark(c: &mut Criterion) -> &mut Criterion {
    let result: [[usize; 9]; 9] = black_box([
        [4, 8, 3, 9, 2, 1, 6, 5, 7],
        [9, 6, 7, 3, 4, 5, 8, 2, 1],
        [2, 5, 1, 8, 7, 6, 4, 9, 3],
        [5, 4, 8, 1, 3, 2, 9, 7, 6],
        [7, 2, 9, 5, 6, 4, 1, 3, 8],
        [1, 3, 6, 7, 9, 8, 2, 4, 5],
        [3, 7, 2, 6, 8, 9, 5, 1, 4],
        [8, 1, 4, 2, 5, 3, 7, 6, 9],
        [6, 9, 5, 4, 1, 7, 3, 8, 2]
    ]);
    c.bench_function(
        "finder",
        |b| b.iter(|| find_empty_cell(&result)),
    )
}

fn no_conflict_benchmark(c: &mut Criterion) -> &mut Criterion {
    let result: [[usize; 9]; 9] = black_box([
        [4, 8, 3, 9, 2, 1, 6, 5, 7],
        [9, 6, 7, 3, 4, 5, 8, 2, 1],
        [2, 5, 1, 8, 7, 6, 4, 9, 3],
        [5, 4, 8, 1, 3, 2, 9, 7, 6],
        [7, 2, 9, 5, 6, 4, 1, 3, 8],
        [1, 3, 6, 7, 9, 8, 2, 4, 5],
        [3, 7, 2, 6, 8, 9, 5, 1, 4],
        [8, 1, 4, 2, 5, 3, 7, 6, 9],
        [6, 9, 5, 4, 1, 7, 3, 8, 2]
    ]);
    c.bench_function(
        "conflict",
        |b| b.iter(|| no_conflict(&result, 0, 0, 4)),
    )
}

criterion_group!(benches, solve_sudoku_benchmark, find_empty_cell_benchmark, no_conflict_benchmark);
criterion_main!(benches);


