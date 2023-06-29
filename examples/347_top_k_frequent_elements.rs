use std::collections::HashMap;

fn main() {}

fn solution(nums: Vec<i32>, k: i32) -> Vec<i32>
{
    let mut stor = HashMap::new();

    let update = |n| *stor.entry(n).or_insert(0) += 1;
    nums.into_iter().for_each(update);

    let mut stor: Vec<_> = stor.into_iter().collect();
    stor.sort_by_key(|(_k, v)| *v);

    stor.into_iter()
        .rev()
        .take(k as usize)
        .map(|(k, _v)| k)
        .collect()
}

#[cfg(test)]
mod tests
{
    use crate::solution;

    #[test]
    fn solution_test()
    {
        assert_eq!(solution(vec![0, 5, 4, 2, 1, 6, 6], 1), vec![6]);
    }
}
