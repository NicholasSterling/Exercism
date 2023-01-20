// Note that the exercise actually only wants row-max, col-min saddle points.
// I initially wrote this to look for the reverse as well.

pub fn transpose(m1: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let mut m2: Vec<Vec<u64>> = m1[0].iter().map( |_cols| Vec::new() ).collect();
    for row in m1.iter() {
        for (col_num, col) in row.iter().enumerate() {
            m2[col_num].push(*col);
        }
    }
    m2
}

pub fn max0(vec: &Vec<u64>) -> u64 { *vec.iter().max().unwrap() }
pub fn min0(vec: &Vec<u64>) -> u64 { *vec.iter().min().unwrap() }

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points:Vec<(usize, usize)> = Vec::new();
    let rows = input;
    if rows.is_empty() { return saddle_points; }
    let cols = transpose(input);
    if cols.is_empty() { return saddle_points; }
    let max_of_row: Vec<u64> = rows.iter().map(max0).collect();
    // let min_of_row: Vec<u64> = rows.iter().map(min0).collect();
    // let max_of_col: Vec<u64> = cols.iter().map(max0).collect();
    let min_of_col: Vec<u64> = cols.iter().map(min0).collect();
    for (row_num, row) in rows.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            let x = *col;
            if  x == max_of_row[row_num] && x == min_of_col[col_num]
            //  ||  x == min_of_row[row_num] && x == max_of_col[col_num]
            {
                saddle_points.push((row_num, col_num));
            }
        }
    }
    saddle_points
}

// Here is a functional solution from a mentor:

// pub fn find_saddle_points_mentor(input: &[Vec<u64>]) -> Vec<u64> {
//     iproduct!(0..input.len(),0..input[0].len())
//         .filter(|&(row, col)|
//             !input[row].iter().any(|&n| n > input[row][col])
//             && !input.iter().any(|n| n[col] < input[row][col]))
//         .collect()
// }

// iproduct! is a macro from the itertools crate. it is essentially the same as:
//
//  (0..input.len())
//  .flat_map(|row| iter::repeat(row).zip(0..input[0].len())}
