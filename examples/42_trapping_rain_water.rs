
fn main() {}

fn my_inefficent_solution(heights : Vec<i32>) -> i32
{
    let mut idx = 0;
    let mut buf = 0;
    while idx < heights.len() - 1
    {
        let left = heights[idx];
        if left < 1
        {
            idx += 1;
            continue
        }
        let find_right = heights.iter().enumerate().skip(idx + 1).find(|(_, &x)| x > left);
        let (r_pos, right) =
            if let Some((i, r)) = find_right
            { (i, *r) }
            else
            {
                let max_height_left = heights.iter().enumerate().skip(idx + 1).max_by_key(|(_, &x)| x);
                if let Some((i, r)) = max_height_left { (i, *r) } else { break }
            };
        if right < 1 { return buf }
        
        let min = i32::min(left, right);
        let filled_with_water : i32 = heights[idx + 1 .. r_pos].iter().fold(0, |acc, x| acc + min - x);
        
        buf += filled_with_water;
        idx = r_pos;
    }
    buf
}

#[cfg(test)]
mod tests
{
    use leetcode_solutions::benchmarking::bench_times;
    use super::*;
    
    #[test]
    fn solution_test()
    {
        assert_eq!(vec![0,1,0,2,1,0,1,3,2,1,2,1][7], 3);
        assert_eq!(my_inefficent_solution(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(my_inefficent_solution(vec![4,2,0,3,2,5]), 9);
        assert_eq!(my_inefficent_solution(vec![0, 4, 2, 0, 3, 2, 5, 7, 0, 6, 0, 0, 5, 4, 0, 4, 0]), 29);
    }
}


