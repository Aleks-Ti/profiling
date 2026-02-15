use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, Ordering::SeqCst);
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    COUNTER.load(Ordering::SeqCst)
}

pub fn read_after_sleep() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}

pub fn reset_counter() {
    COUNTER.store(0, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_increment_regression() {
        // Простой случай: 1 поток, 1 итерация → ожидаем 1
        assert_eq!(race_increment(1, 1), 1);

        // Несколько потоков, несколько итераций
        assert_eq!(race_increment(100, 4), 400);

        // Много потоков, мало итераций
        assert_eq!(race_increment(1, 10), 10);

        // Большой объём — проверка отсутствия гонок
        let result = race_increment(10_000, 8);
        assert_eq!(result, 80_000);

        // Пустой запуск
        assert_eq!(race_increment(0, 5), 0);
        assert_eq!(race_increment(5, 0), 0); // нет потоков → 0
    }
}
