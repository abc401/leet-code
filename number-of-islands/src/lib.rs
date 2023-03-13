use std::collections::HashSet;


pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut explored = HashSet::new();
    let mut n_islands = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '0' {
                continue;
            }

            let start = (row, col);
            
            if explored.contains(&start) {
                continue;
            }

            n_islands += 1;
            explore(&grid, start, &mut explored);
        }
    }
    return n_islands;
}

fn neighbors(point: (usize, usize)) -> Vec<(isize, isize)> {
    let row = point.0 as isize;
    let col = point.1 as isize;
    return Vec::from([
        (row - 1, col),
        (row, col - 1),
        (row + 1, col),
        (row, col + 1)
    ]);
}

fn explore(grid: &Vec<Vec<char>>, start: (usize, usize), explored: &mut HashSet<(usize, usize)>) {
    explored.insert(start);
    for (row, col) in neighbors(start){
        if 
            row < 0
            || col < 0
            || row >= grid.len() as isize
            || col >= grid[0].len() as isize
        {
            continue;
        }
        let row = row as usize;
        let col = col as usize;

        if explored.contains(&(row, col)) {
            continue;
        }

        if grid[row][col] == '1' {
            explore(grid, (row, col), explored);
        }
    }
}