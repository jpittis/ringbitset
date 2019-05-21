use super::ringbitset::RingBitSet;

pub struct CircuitBreakerMetrics {
    bitset: RingBitSet,
}

#[derive(PartialEq, Debug)]
pub enum FailureReport {
    NotEnoughData,
    PercentFailed(f64),
}

impl CircuitBreakerMetrics {
    pub fn new(request_buffer_size: usize) -> CircuitBreakerMetrics {
        CircuitBreakerMetrics {
            bitset: RingBitSet::new(request_buffer_size),
        }
    }

    pub fn on_success(&mut self) -> FailureReport {
        let failures = self.bitset.set_next_bit(false);
        self.failure_report(failures)
    }

    pub fn on_failure(&mut self) -> FailureReport {
        let failures = self.bitset.set_next_bit(true);
        self.failure_report(failures)
    }

    fn failure_report(&self, failures: usize) -> FailureReport {
        if self.bitset.length() < self.bitset.capacity_in_bits() {
            FailureReport::NotEnoughData
        } else {
            FailureReport::PercentFailed(
                (failures as f64) * 100.0 / (self.bitset.capacity_in_bits() as f64),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_breaker_metrics() {
        let mut metrics = CircuitBreakerMetrics::new(5);
        assert_eq!(metrics.on_success(), FailureReport::NotEnoughData);
        assert_eq!(metrics.on_success(), FailureReport::NotEnoughData);
        assert_eq!(metrics.on_failure(), FailureReport::NotEnoughData);
        assert_eq!(metrics.on_failure(), FailureReport::NotEnoughData);
        assert_eq!(metrics.on_success(), FailureReport::PercentFailed(40.0));
        assert_eq!(metrics.on_failure(), FailureReport::PercentFailed(60.0));
        assert_eq!(metrics.on_failure(), FailureReport::PercentFailed(80.0));
        assert_eq!(metrics.on_failure(), FailureReport::PercentFailed(80.0));
        assert_eq!(metrics.on_failure(), FailureReport::PercentFailed(80.0));
        assert_eq!(metrics.on_failure(), FailureReport::PercentFailed(100.0));
        assert_eq!(metrics.on_success(), FailureReport::PercentFailed(80.0));
        assert_eq!(metrics.on_success(), FailureReport::PercentFailed(60.0));
        assert_eq!(metrics.on_success(), FailureReport::PercentFailed(40.0));
        assert_eq!(metrics.on_success(), FailureReport::PercentFailed(20.0));
        assert_eq!(metrics.on_success(), FailureReport::PercentFailed(0.0));
    }
}
