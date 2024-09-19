fn search_min_dis(matrix: &[&[i32]]) -> Option<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return None;
    }

    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let mut cur_dist = vec![vec![0; col_len]; row_len];
    cur_dist[0][0] = matrix[0][0];

    for i in 1..row_len {
        cur_dist[i][0] = cur_dist[i - 1][0] + matrix[i][0];
    }

    for j in 1..col_len {
        cur_dist[0][j] = cur_dist[0][j - 1] + matrix[0][j];
    }

    for i in 1..row_len {
        for j in 1..col_len {
            cur_dist[i][j] = i32::min(cur_dist[i - 1][j], cur_dist[i][j - 1]) + matrix[i][j];
        }
    }

    println!("{:?}", &cur_dist);
    Some(cur_dist[row_len - 1][col_len - 1])
}

fn search_min_dis_plus(matrix: &[&[i32]]) -> Option<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return None;
    }

    let row_len = matrix.len();
    let col_len = matrix[0].len();
    let mut cur_dist = vec![0; col_len];
    cur_dist[0] = matrix[0][0];

    for j in 1..col_len {
        cur_dist[j] = cur_dist[j - 1] + matrix[0][j];
    }
    println!("{:?}", &cur_dist);

    for i in 1..row_len {
        cur_dist[0] += matrix[i][0];
        for j in 1..col_len {
            cur_dist[j] = i32::min(cur_dist[j], cur_dist[j - 1]) + matrix[i][j];
        }
    }

    println!("{:?}", &cur_dist);
    Some(cur_dist[col_len - 1])
}

fn main() {
    let input1: &[&[i32]] = &[&[1, 3, 1], &[1, 5, 1], &[4, 2, 1]];
    println!("{:?}", search_min_dis(input1));
    println!("{:?}", search_min_dis_plus(&input1));

    let input2: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6]];

    println!("{:?}", search_min_dis(input2));
    println!("{:?}", search_min_dis_plus(&input2));
}
