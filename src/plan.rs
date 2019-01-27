
struct Plan{
    threads: usize,
    requests: usize,
}

impl Plan {
    fn new(threads: usize, requests: usize) -> Plan {
        Plan {
            threads: threads,
            requests: requests
        }
    }

    fn threads(&self) -> usize {
        self.threads
    }
    
    fn requests(&self) -> usize {
        self.requests
    }

    fn distribute(&self) -> Vec<usize> {
        let work = self.requests / self.threads;
        let extra_work = self.requests % self.threads;

        (0..self.threads)
        .map(|thread| {
            if thread < extra_work {
                work + 1 
            } else {
                work
            }
        })
        .filter(|thread| {*thread != 0})
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribution_of_workload() {
        // All workers utilized
        assert_eq!(Plan::new(3, 100).distribute(), [34, 33, 33]);
        assert_eq!(Plan::new(3, 1000).distribute(), [334, 333, 333]);
        assert_eq!(Plan::new(5, 10).distribute(), [2, 2, 2, 2, 2]);

        // Some workers unutilized
        assert_eq!(Plan::new(5, 3).distribute(), [1, 1, 1]);
        assert_eq!(Plan::new(10, 5).distribute(), [1, 1, 1, 1, 1]);
    }

}
