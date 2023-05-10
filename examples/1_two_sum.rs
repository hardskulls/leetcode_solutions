
use std::collections::HashMap;


fn main()
{

}


/// link: https://leetcode.com/problems/two-sum/
fn solution(nums: Vec<i32>, target: i32) -> Vec<i32>
{
    let iter =
        nums.iter()
            .enumerate()
            .map(|(idx, num)| (*num, idx));
    let table : HashMap<i32, usize> = HashMap::from_iter(iter);
    
    let sec_loop = 0..nums.len() - 1;
    for i in sec_loop
    {
        let complement = target - nums[i];
        if table.contains_key(&complement) && table[&complement] != i
        { return vec![i as i32, table[&complement] as i32] }
    }
    
    vec![0, 0]
}


#[cfg(test)]
mod tests
{
    use crate::solution;
    
    #[test]
    fn problem_1_two_sum()
    {
        
        let test_case_1 = (vec![1, 8, 7, 9], 15);
        assert_eq!(solution(test_case_1.0, test_case_1.1), vec![1, 2]);
    }
}


