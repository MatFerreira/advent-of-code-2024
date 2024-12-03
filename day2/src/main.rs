fn main() {
    let input: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let safe_reports: Vec<&Vec<i32>> = input
        .iter()
        .filter(|report| verify_safety(report))
        .collect();

    dbg!(safe_reports.len());

    let safe_reports_fault_tolerant: Vec<&Vec<i32>> = input
        .iter()
        .filter(|report| verify_safety_fault_tolerant(report))
        .collect();

    dbg!(safe_reports_fault_tolerant.len());

    // dbg!(&input);
}

// Part One
fn verify_safety(report: &[i32]) -> bool {
    let mut prev_lvl = report[0];
    let mut is_increasing = false;

    for (i, lvl) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if i == 1 {
            is_increasing = *lvl > prev_lvl;
        }

        let distance = (lvl - prev_lvl).abs();

        if is_increasing != (*lvl > prev_lvl) || !(1..=3).contains(&distance) {
            return false;
        }
        prev_lvl = *lvl;
    }

    true
}

// Part Two
fn verify_safety_fault_tolerant(report: &[i32]) -> bool {
    let mut prev_lvl = report[0];
    let mut is_increasing = false;
    let mut faults = 0;

    for (i, lvl) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if i == 1 {
            is_increasing = *lvl > prev_lvl;
        }

        let distance = (lvl - prev_lvl).abs();

        if is_increasing != (*lvl > prev_lvl) || !(1..=3).contains(&distance) {
            faults += 1;
            // return false;
        }

        if faults > 1 {
            return false;
        }

        prev_lvl = *lvl;
    }

    true
}
