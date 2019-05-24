use std::time::{Duration, Instant};

pub fn time_it<F, R>(f: F) -> (R, Duration) where
    F: FnOnce() -> R, {
    let start = Instant::now();
    (f(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_time_it() {
        let (u, d) = time_it(|| 123);
        assert_eq!(u, 123);
        assert!(d > Duration::new(0,0));
    }

}
