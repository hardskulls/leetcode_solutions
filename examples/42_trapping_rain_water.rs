use std::cmp::{max, min};

fn main() {}

fn my_inefficent_solution(heights: &[i32]) -> i32 {
    let mut idx = 0;
    let mut buf = 0;
    while idx < heights.len() - 1 {
        let left = heights[idx];
        if left < 1 {
            idx += 1;
            continue;
        }
        let find_right = heights
            .iter()
            .enumerate()
            .skip(idx + 1)
            .find(|(_, &x)| x > left);
        let (r_pos, right) = if let Some((i, r)) = find_right {
            (i, *r)
        } else {
            let max_height_left = heights
                .iter()
                .enumerate()
                .skip(idx + 1)
                .max_by_key(|(_, &x)| x);
            if let Some((i, r)) = max_height_left {
                (i, *r)
            } else {
                break;
            }
        };
        if right < 1 {
            return buf;
        }

        let min = i32::min(left, right);
        let filled_with_water: i32 = heights[idx + 1..r_pos]
            .iter()
            .fold(0, |acc, x| acc + min - x);

        buf += filled_with_water;
        idx = r_pos;
    }
    buf
}

fn maximize(state: &mut i32, h: &i32) -> Option<i32> {
    *state = max(*state, *h);
    Some(*state)
}

// TODO: test / fix
fn best_solution_from_leetcode(heights: &[i32]) -> i32 {
    let left_max = heights.iter().scan(0, maximize);
    let right_max = heights.iter().rev().scan(0, maximize);
    heights
        .iter()
        .zip(left_max)
        .zip(right_max)
        .map(|((&height, left_m), right_m)| max(min(height, left_m) - right_m, 0))
        .sum()
}

// TODO: test / fix
fn modified_leetcode_solution(heights: &[i32]) -> i32 {
    let left_max = heights.iter().scan(0, maximize);
    let right_max = heights
        .iter()
        .rev()
        .scan(0, maximize)
        .collect::<Vec<i32>>()
        .into_iter()
        .rev();
    heights
        .iter()
        .zip(left_max)
        .zip(right_max)
        .map(|((&height, left_m), right_m)| max(min(left_m, right_m) - height, 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_solutions::benchmarking::bench_times;

    #[test]
    fn solution_test() {
        assert_eq!(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1][7], 3);
        assert_eq!(
            my_inefficent_solution(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(my_inefficent_solution(&[4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(
            my_inefficent_solution(&[0, 4, 2, 0, 3, 2, 5, 7, 0, 6, 0, 0, 5, 4, 0, 4, 0]),
            29
        );
    }

    // TODO: test / fix
    #[test]
    fn leetcode_solution_test() {
        assert_eq!(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1][7], 3);
        assert_eq!(
            best_solution_from_leetcode(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(best_solution_from_leetcode(&[4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(
            best_solution_from_leetcode(&[0, 4, 2, 0, 3, 2, 5, 7, 0, 6, 0, 0, 5, 4, 0, 4, 0]),
            29
        );
    }
}
