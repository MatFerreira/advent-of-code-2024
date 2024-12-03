fn main() {
    let input: Vec<(i32, i32)> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut line_split = line.split_whitespace();
            let left_number: i32 = line_split.next().unwrap().parse().unwrap();
            let right_number: i32 = line_split.next().unwrap().parse().unwrap();
            (left_number, right_number)
        })
        .collect();

    let (left_list, right_list): (Vec<i32>, Vec<i32>) = input.into_iter().unzip();

    let distances_sum = compute_distance(&left_list, &right_list);
    dbg!(&distances_sum);

    let similarity_score = compute_similarity_score(&left_list, &right_list);
    dbg!(&similarity_score);
}

// Part One
fn compute_distance(left_list: &[i32], right_list: &[i32]) -> i32 {
    let mut left_list = left_list.to_vec();
    let mut right_list = right_list.to_vec();

    left_list.sort();
    right_list.sort();

    let sorted_pairs: Vec<(i32, i32)> = left_list.into_iter().zip(right_list).collect();

    let distances: Vec<i32> = sorted_pairs
        .iter()
        .map(|(left, right)| {
            let distance = left - right;
            distance.abs()
        })
        .collect();

    distances.into_iter().sum()
}

// Part Two
fn compute_similarity_score(left_list: &[i32], right_list: &[i32]) -> i32 {
    use itertools::Itertools;

    let rl_counts = right_list.iter().counts();
    left_list
        .iter()
        .map(|n| {
            let count = rl_counts.get(n).unwrap_or(&0);
            n * (*count as i32)
        })
        .sum()
}
