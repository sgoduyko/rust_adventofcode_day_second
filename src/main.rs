
fn is_safe_report(report: &[i32; 5]) -> bool {
    // problems of the approach:
    // 1) overhard when report has a bit levels
    // we are going from 2 sides per iteration and have additional vars that make overhard when len of report is a bit.

    let mut first_index = 0;
    let mut last_index = report.len() - 1;

    let mut is_safe = true;
    let mut is_decreasing_start = true;
    let mut is_decreasing_end = true;
    let mut is_decreasing_global = true;


    while first_index < last_index {
        // from start. is decreasing
        if report[first_index] < report[first_index+1] {
            is_decreasing_start = false;
        } else { is_decreasing_start = true }

        // from end. is decreasing
        if report[last_index-1] < report[last_index] {
            is_decreasing_end = false;
        } else {is_decreasing_end = true}

        if is_decreasing_start != is_decreasing_end {
            is_safe = false;
            break;
        }
        if (first_index == 0) {
            is_decreasing_global = is_decreasing_start;
        };
        if first_index > 0 && is_decreasing_start != is_decreasing_global {
            is_safe = false;
            break;
        }
        
        // DIFF
        // from start. diff in (1, 3)
        let diff_start = (report[first_index] - report[first_index+1]).abs();
        if diff_start < 1 || diff_start > 3 {
            is_safe = false;
            break;
        }
        // from end. diff in (1, 3)
        let diff_end = (report[last_index-1] - report[last_index]).abs();
        if diff_end < 1 || diff_end > 3 {
            is_safe = false;
            break;
        }

        // up/down indexes;
        first_index += 1;
        last_index -= 1;
    };
    // RETURN
    is_safe
}

fn calc_safe_report_count(reports: &[[i32; 5]]) -> i32 {
    // Calc safe report count from slice of reports
    let mut safe_count = 0;
    for report in reports {
        if is_safe_report(report) {
            safe_count += 1;
        }
    }
    safe_count
}

fn main() {
    println!("Hello, world!");
}
