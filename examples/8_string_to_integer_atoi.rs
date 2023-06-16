

fn main()
{

}

const DEFAULT : i32 = 0;

#[inline]
fn solution(s : impl AsRef<str>) -> i32
{
    let default = 0;
    let cleaned_s = s.as_ref().trim_start();
    
    match cleaned_s.len()
    {
        0 => default,
        1 => one_letter(cleaned_s),
        2 => two_letter(cleaned_s),
        _ => three_or_more_letters(cleaned_s)
    }
}

#[inline]
fn one_letter(cleaned_s : &str) -> i32
{
    cleaned_s.chars()
        .take_while(|c| c.is_ascii_digit())
        .last()
        .map_or(DEFAULT, |c| c as i32 - 48)
}

#[inline]
fn two_letter(cleaned_s : &str) -> i32
{
    match (cleaned_s[0..1].chars().last(), cleaned_s[1..2].chars().last())
    {
        (Some('-'), Some(two @ '0'..='9')) => -(two as i32 - 48),
        (Some('+'), Some(two @ '0'..='9')) => two as i32 - 48,
        (Some(one @ '0'..='9'), Some(two @ '0'..='9')) => (one as i32 - 48) * 10 + (two as i32 - 48),
        (Some(one @ '0'..='9'), ..) => one as i32 - 48,
        _ => DEFAULT
    }
}

#[inline]
fn three_or_more_letters(cleaned_s : &str) -> i32
{
    if cleaned_s.len() < 3 { return DEFAULT }
    let (one, two) = (cleaned_s[0..1].chars().last().unwrap(), cleaned_s[1..2].chars().last().unwrap());
    let is_negative = one == '-';
    if !matches!((one, two), ('-' | '+', '0'..='9') | ('0'..='9', ..)) { return DEFAULT }
    
    let digits_start_idx = if one.is_ascii_digit() { 0 } else { 1 };
    
    let string_left = &cleaned_s[digits_start_idx..];
    let mut chars = string_left.char_indices();
    let end =
        chars.find(|(_idx, c)| !c.is_ascii_digit())
            .map(|(idx, _)| idx)
            .unwrap_or(string_left.len());
    let number_chars = &string_left[..end];
    number_chars.parse::<i32>()
        .map(|n| if is_negative { -n } else { n })
        .unwrap_or(if is_negative { i32::MIN } else { i32::MAX })
}

// Not mine (optimized).
#[inline]
pub fn my_atoi(s : impl AsRef<str>) -> i32
{
    let s = s.as_ref().trim_start();
    
    let first_char = s.get(0..1).and_then(|one| one.chars().last());
    if !matches!(first_char, Some('-') | Some('+') | Some('0'..='9'))
    { return DEFAULT }
    
    let (s, sign) =
        match s.strip_prefix('-')
        {
            Some(s) => (s, -1),
            None => (s.strip_prefix('+').unwrap_or(s), 1),
        };
    s.chars()
        .map_while(|c| c.to_digit(10))
        .fold(0, |acc, digit| acc.saturating_mul(10).saturating_add(sign * digit as i32))
}


#[cfg(test)]
mod tests
{
    use leetcode_solutions::benchmarking::bench_times;
    use crate::{my_atoi, solution};
    
    #[test]
    fn solution_test()
    {
        let (min, max) = (i32::MIN, i32::MAX);
        assert_eq!(solution(" -42"), -42);
        assert_eq!(solution("   +78957958465465_hiugougo"), max);
        assert_eq!(solution("   -78957958465465_hiugougo"), min);
        assert_eq!(solution("   jh + 78957958465465_hiugougo"), 0);
        assert_eq!(solution("   jh - 78957958465465_hiugougo"), 0);
        assert_eq!(solution("    + 78957958465465_hiugougo"), 0);
        
        assert_eq!(solution("    - 78957958465465_hiugougo"), 0);
        assert_eq!(solution("   -"), 0);
        assert_eq!(solution(" hoihl 789798"), 0);
        assert_eq!(solution("   .1"), 0);
        assert_eq!(solution("s"), 0);
        assert_eq!(solution("0"), 0);
        assert_eq!(solution("1"), 1);
        assert_eq!(solution("2"), 2);
        assert_eq!(solution("3"), 3);
        assert_eq!(solution("4"), 4);
        assert_eq!(solution("5"), 5);
        assert_eq!(solution("6"), 6);
        assert_eq!(solution("7"), 7);
        assert_eq!(solution("8"), 8);
        assert_eq!(solution("9"), 9);
        assert_eq!(solution("8l"), 8);
        assert_eq!(solution("           -869765436578966786467535424315325465487607698656321 lokcrtssrt"), min);
    }
    
    #[test]
    fn not_my_solution_test()
    {
        let (min, max) = (i32::MIN, i32::MAX);
        assert_eq!(my_atoi(" -42"), -42);
        assert_eq!(my_atoi("   +78957958465465_hiugougo"), max);
        assert_eq!(my_atoi("   -78957958465465_hiugougo"), min);
        assert_eq!(my_atoi("   jh + 78957958465465_hiugougo"), 0);
        assert_eq!(my_atoi("   jh - 78957958465465_hiugougo"), 0);
        assert_eq!(my_atoi("    + 78957958465465_hiugougo"), 0);
        
        assert_eq!(my_atoi("    - 78957958465465_hiugougo"), 0);
        assert_eq!(my_atoi("   -"), 0);
        assert_eq!(my_atoi(" hoihl 789798"), 0);
        assert_eq!(my_atoi("   .1"), 0);
        assert_eq!(my_atoi("s"), 0);
        assert_eq!(my_atoi("0"), 0);
        assert_eq!(my_atoi("1"), 1);
        assert_eq!(my_atoi("2"), 2);
        assert_eq!(my_atoi("3"), 3);
        assert_eq!(my_atoi("4"), 4);
        assert_eq!(my_atoi("5"), 5);
        assert_eq!(my_atoi("6"), 6);
        assert_eq!(my_atoi("7"), 7);
        assert_eq!(my_atoi("8"), 8);
        assert_eq!(my_atoi("9"), 9);
        assert_eq!(my_atoi("8l"), 8);
        assert_eq!(my_atoi("           -869765436578966786467535424315325465487607698656321 lokcrtssrt"), min);
    }
    
    #[test]
    fn bench()
    {
        let number_of_iterations = 10_000_000;
        let filler_value = "s";
        let fillers_number = 800;
        
        let mut s = String::from_iter(vec![filler_value; fillers_number].into_iter());
        s.insert(fillers_number / 2, '-');
        let my_long_sol = bench_times(number_of_iterations, || { solution(&s); });
        
        let mut s = String::from_iter(vec![filler_value; fillers_number].into_iter());
        s.insert(fillers_number / 2, '-');
        let short_func_sol = bench_times(number_of_iterations, || { my_atoi(&s); });
        
        assert_eq!(my_long_sol.unwrap(), short_func_sol.unwrap());
    }
}


