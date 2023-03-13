use std::collections::HashSet;

pub fn solve(board: &mut Vec<Vec<char>>) {
    let mut safe_cells = HashSet::new();
    let cols = board[0].len();
    let rows = board.len();

    // The only reigons of O's that will not be captured are the ones that are adjacent to O's on the boundary
    // Thus, we only have to search which reigons are adjacent to the O's on the boundary
    // Every cell that is not in one of these reigons will have its value changed to an X
    
    for col in 0..cols {
        // find reigons adjacent to O's in the following positions on the boundary
        //      # # # # # #
        //      . . . . . .
        //      . . . . . .
        //      . . . . . .
        //      . . . . . .
        //      # # # # # #

        search(board, &mut safe_cells, 0, col);
        search(board, &mut safe_cells, rows-1, col);
    }

    for row in 1..rows-1 {
        // find reigons adjacent to O's in the following positions on the boundary
        //      . . . . . .
        //      # . . . . #
        //      # . . . . #
        //      # . . . . #
        //      # . . . . #
        //      . . . . . .

        search(board, &mut safe_cells ,row, 0);
        search(board, &mut safe_cells ,row, cols-1)
    }

    for row in 0..rows {
        for col in 0..cols {
            if !safe_cells.contains(&(row, col)) {
                board[row][col] = 'X';
            }
        }
    }
}

fn neighbors(row: usize, col: usize) -> [(isize, isize); 4] {
    let row = row as isize;
    let col = col as isize;
    return [
        (row - 1, col),
        (row, col -1),
        (row + 1, col),
        (row, col + 1)
    ]
}

fn search(board: &mut Vec<Vec<char>>, safe_cells: &mut HashSet<(usize, usize)>, row: usize, col: usize) {
    safe_cells.insert((row, col));
    if board[row][col] == 'X' {
        return;
    }
    for (row, col) in neighbors(row, col) {
        if 
            row < 0 
            || row >= board.len() as isize 
            || col < 0 
            || col >= board[0].len() as isize 
        {
            continue;
        }
        let row = row as usize;
        let col = col as usize;
        if board[row][col] == 'O' {
            if safe_cells.contains(&(row, col)) {
                continue;
            }
            search(board, safe_cells, row, col)
        }
    }
}