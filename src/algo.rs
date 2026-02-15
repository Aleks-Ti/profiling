/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            // лишняя копия, хотя можно было пушить значение напрямую
            out.push(*v);
            out.sort_unstable(); // сортировка каждую итерацию - ужас(вернул для теста)
        }
    }
    out
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
/// Старая версия просто выполнялась вечность и тест никогда не проходил.
pub fn slow_fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_slow_fib_correctness() {
        assert_eq!(slow_fib(0), 0);
        assert_eq!(slow_fib(1), 1);
        assert_eq!(slow_fib(2), 1);
        assert_eq!(slow_fib(3), 2);
        assert_eq!(slow_fib(4), 3);
        assert_eq!(slow_fib(5), 5);
        assert_eq!(slow_fib(10), 55);
        assert_eq!(slow_fib(20), 6765);
    }

    #[test]
    fn test_slow_fib_performance_regression() {
        let n = 50;

        let start = Instant::now();
        let result = slow_fib(n);
        let elapsed = start.elapsed();

        // Проверяем корректность
        assert_eq!(result, 12_586_269_025); // fib(50)

        // должно выполняться БЫСТРО
        assert!(
            elapsed < Duration::from_millis(10),
            "fib({}) took {:?} — likely exponential regression!",
            n,
            elapsed
        );
    }
}
