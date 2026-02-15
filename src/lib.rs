pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        // утечка: не вызываем Box::from_raw(raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let mut sum = 0i64;
    let mut count = 0usize;

    for &value in values {
        if value > 0 {
            sum += value;
            count += 1;
        }
    }

    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    let val = *raw;
    drop(Box::from_raw(raw));
    val + *raw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_positive_regression() {
        // Основной случай: только положительные числа
        assert_eq!(average_positive(&[1, 2, 3]), 2.0);

        // Смешанные значения: отрицательные, ноль и положительные
        assert_eq!(average_positive(&[-5, -2, 0, 4, 6]), 5.0); // (4 + 6) / 2 = 5.0

        // Только неположительные — результат должен быть 0.0
        assert_eq!(average_positive(&[-1, -2, 0]), 0.0);

        // Пустой срез
        assert_eq!(average_positive(&[]), 0.0);

        // Один положительный элемент
        assert_eq!(average_positive(&[42]), 42.0);

        // Один отрицательный элемент
        assert_eq!(average_positive(&[-42]), 0.0);

        // Большие числа
        assert_eq!(average_positive(&[1_000_000, 2_000_000]), 1_500_000.0);

        // Убедимся, что НОЛЬ не учитывается как положительное число
        assert_eq!(average_positive(&[0, 0, 0]), 0.0);
        assert_eq!(average_positive(&[0, 1, 0]), 1.0); // только 1 учитывается
    }
}
