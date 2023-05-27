use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {}

fn solution(s : impl AsRef<str>) -> i32
{
    let s = s.as_ref();
    
    let mut count = 0;
    let mut seen = HashMap::new();
    
    let mut start = 0;
    
    for (i, c) in s.chars().enumerate()
    {
        let idx = seen.get(&c).copied();
        if let Some(idx) = idx
        { start = usize::max(start, idx + 1); }
        seen.insert(c, i);
        
        
        
        let current = (i + 1) - start;
        count = usize::max(count, current);
    }
    
    count as i32
}

fn length_of_longest_substring(s : impl AsRef<str>) -> i32
{
    let s = s.as_ref();
    let mut cpos = HashMap::<char, i32>::new();
    let mut origin = -1;
    let mut max = 0;
    
    for (i, c) in s.chars().enumerate()
    {
        match cpos.entry(c)
        {
            Entry::Occupied(mut oe) =>
                {
                    let orig = oe.insert(i as i32);
                    if orig > origin
                    { origin = orig }
                }
            Entry::Vacant(ve) => { ve.insert(i as i32); }
        };
        let m = i as i32 - origin;
        if m > max
        { max = m };
    }
    
    max
}


#[cfg(test)]
mod tests
{
    use leetcode_solutions::benchmarking::bench_times;
    use crate::{length_of_longest_substring, solution};
    
    #[test]
    fn solution_test()
    {
        assert_eq!(solution("abc"), 3);
        assert_eq!(solution(""), 0);
        assert_eq!(solution("a"), 1);
        assert_eq!(solution("babc"), 3);
        assert_eq!(solution("bbabc"), 3);
        assert_eq!(solution("bb"), 1);
        assert_eq!(solution("bbb"), 1);
        assert_eq!(solution("bbbacdacc"), 4);
        assert_eq!(solution("abcahjkl"), 7);
        assert_eq!(solution("abba"), 2);
        assert_eq!(solution("abcabcbb"), 3);
    }
    
    #[test]
    fn bench_solution()
    {
        let s = "l/b.khc.hkv.kjycfytfutdliyfkhgmjtdyhg.jgutgl/ighyuyf.uhijkujyfhtgcvnbmuiy;uigk.jb,nyfjhvyfjg,vhmfjhvjhvm";
        let count = 100_000;
        
        let my_solution = bench_times(count, || { solution(s); } ).unwrap();
        let leetcode_best = bench_times(count, || { length_of_longest_substring(s); } ).unwrap();
        
        assert_eq!(my_solution, leetcode_best);
    }
}


