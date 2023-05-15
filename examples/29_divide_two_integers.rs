
use std::convert::TryInto;


fn main()
{
    dbg!(solution(18, -3));
}

fn solution(dividend : i32, divisor : i32) -> i32
{
    let mut answer : i64 = 0;
    let negative = dividend.signum() * divisor.signum();
    
    let (mut dividend, divisor) = ((dividend as i64).abs(), (divisor as i64).abs());
    
    for i in (0..=31).rev()
    {
        if divisor << i <= dividend
        {
            dividend -= divisor << i;
            answer += 1 << i
        }
    }
    
    answer *= negative as i64;
    
    answer.try_into()
        .unwrap_or(if negative == -1 { i32::MIN } else { i32::MAX })
}


#[cfg(test)]
mod tests
{
    use rand::Rng;
    use crate::solution;
    
    #[test]
    fn test_solution()
    {
        let (dividend, divisor) = (rand::thread_rng().gen::<i32>(), rand::thread_rng().gen::<i32>());
        let expect : i32 = (dividend as f32 / divisor as f32).trunc() as i32;
        assert_eq!(solution(dividend, divisor), expect);
        assert_eq!(solution(-1, -1), 1);
    }
}


