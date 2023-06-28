pub mod benchmarking {
    use std::time::{Duration, Instant};

    /// Measures the execution time of a function once and returns the duration.
    #[inline]
    pub fn bench_once<F, T>(f: F) -> Duration
    where
        F: FnOnce() -> T,
    {
        let instant = Instant::now();
        f();
        instant.elapsed()
    }

    /// Measure a function's execution time.
    ///
    /// 'iterations' defines how many measurements there will be.
    /// Returns the lowest value, thereby minimizing OS influence on results.
    #[inline]
    pub fn bench_times<F, T>(iterations: u32, mut f: F) -> Option<Duration>
    where
        F: FnMut() -> T,
    {
        let cap = iterations.try_into().unwrap_or(0);
        let mut vec = Vec::with_capacity(cap);
        for _ in 0..iterations {
            let elapsed_time = bench_once(&mut f);
            vec.push(elapsed_time);
        }
        vec.into_iter().min()
    }

    /// Calculates the number of iterations of `bench_once` that can be executed within a specified time limit.
    #[inline]
    pub fn calc_iterations(one_measurment_takes: Duration, desired_time: Duration) -> u32 {
        let mut div = 1;
        while desired_time / div > one_measurment_takes {
            div *= 10
        }
        // For some reason, testing with an unmodified answer takes twice the expected time;
        // that's why there is a correction here.
        div / 2
    }
}

pub mod short_hands {
    pub fn at_idx(idx: usize) -> std::ops::RangeInclusive<usize> {
        idx..=idx
    }
}
