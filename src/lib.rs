
pub mod benchmarking
{
    use std::time::{Duration, Instant};
    
    #[inline]
    pub fn bench_once<F : FnOnce()>(f : F) -> Duration // where F: FnOnce
    {
        let instant = Instant::now();
        f();
        instant.elapsed()
    }
    
    #[inline]
    pub fn bench_times<F : FnMut()>(iterations : u32, mut f : F) -> Option<Duration> // where F: FnOnce
    {
        let cap = iterations.try_into().unwrap_or(0);
        let mut vec = Vec::with_capacity(cap);
        for _ in 0..iterations
        {
            let elapsed_time = bench_once(&mut f);
            vec.push(elapsed_time);
        }
        vec.into_iter().min()
    }
}


