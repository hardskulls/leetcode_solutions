
fn main() {}

fn solution(n : i32) -> i32
{
    let s = n.to_string();
    let s : String= s.chars().rev().collect();
    s.parse().unwrap_or(0)
}

fn fresh_solution(number : i32) -> i32
{
    if -10 < number && number < 10 { return number }
    
    let sign = number.signum();
    let number = if let Some(n) = number.checked_abs() { n } else { return 0 };
    
    let max_10_pow = infer_appropriate_divider(number);
    let max_10_pow = custom_log(max_10_pow);
    
    let mut reversed_number = 0_i32;
    for i in 1..=max_10_pow
    {
        let digit_s = number % 10_i32.pow(i);
        let div_by = 10_i32.pow(i - 1);
        let single_digit = digit_s / div_by;
        
        let add = if let Some(n) = single_digit.checked_mul(10_i32.pow(max_10_pow + 1 - i)) { n } else { return 0 };
        reversed_number = if let Some(n) = reversed_number.checked_add(add) { n } else { return 0 };
    }
    let last_digit = number / 10_i32.pow(max_10_pow);
    (reversed_number + last_digit).checked_mul(sign).unwrap_or(0)
}

fn infer_appropriate_divider(n : i32) -> i32
{
    let mut div = 1_000_000_000;
    loop
    {
        if div <= n
        { break div }
        else
        { div /= 10 }
    }
}

fn custom_log(number : i32) -> u32
{
    let mut power = 0;
    let mut ten = 10;
    while number / ten >= 1
    {
        power += 1;
        ten = ten.wrapping_mul(10);
    }
    power
}

#[cfg(test)]
mod tests
{
    use leetcode_solutions::benchmarking::bench_times;
    use crate::{custom_log, fresh_solution, solution};
    
    #[test]
    fn bench()
    {
        let times = 100;
        let number = i32::MAX / 10;
        let with_strings = bench_times(times, || { solution(number); } ).unwrap();
        let no_std = bench_times(times, || { fresh_solution(number); }).unwrap();
        assert_eq!(with_strings, no_std);
    }
    
    #[test]
    fn div_vs_rem_test()
    {
        assert_eq!(100 / 100, 1);
        assert_eq!(1 / 100, 0);
        assert_eq!(100 % 100, 0);
    }
    
    #[test]
    fn fresh_solution_test()
    {
        assert_eq!(fresh_solution(123), 321);
        assert_eq!(fresh_solution(-123), -321);
        let max = fresh_solution(i32::MAX);
        assert_eq!(max, 0);
        let min = fresh_solution(i32::MIN);
        assert_eq!(min, 0);
        let zero = fresh_solution(0);
        assert_eq!(zero, 0);
        let minus_1 = fresh_solution(-1);
        assert_eq!(minus_1, -1);
        let plus_1 = fresh_solution(1);
        assert_eq!(plus_1, 1);
        let plus_100 = fresh_solution(100);
        assert_eq!(plus_100, 1);
        let plus_1010 = fresh_solution(1010);
        assert_eq!(plus_1010, 101);
        assert_eq!(fresh_solution(-320105), -501023);
        assert_eq!(fresh_solution(320105), 501023);
        assert_eq!(fresh_solution(-2147483412), -2143847412);
        assert_eq!(fresh_solution(1563847412), 0);
    }
    
    #[test]
    fn test_custom_log()
    {
        assert_eq!(custom_log(0), 0);
        assert_eq!(custom_log(1), 0);
        assert_eq!(custom_log(10), 1);
        assert_eq!(custom_log(100), 2);
    }
}


