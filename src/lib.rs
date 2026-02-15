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
    let slice_ptr = Box::into_raw(boxed); // тип: *mut [u8]

    let mut count = 0;
    unsafe {
        let raw = slice_ptr as *mut u8;
        let len = (&(*slice_ptr)).len();

        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        let _ = Box::from_raw(slice_ptr);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_whitespace() {
        result.push_str(word);
    }
    result.to_lowercase()
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
    let val = unsafe { *raw };
    let result = val + val;
    unsafe {
        drop(Box::from_raw(raw));
    }
    result
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

    #[test]
    fn test_normalize_regression() {
        // Базовый случай
        assert_eq!(normalize("Hello World"), "helloworld");

        // Множество пробелов → один
        assert_eq!(normalize("Hello     World"), "helloworld");

        // Табуляции и переносы
        assert_eq!(normalize("Hello\t\n\rWorld"), "helloworld");

        // Смешанные whitespace
        assert_eq!(normalize("  Hello \t \n World  "), "helloworld");

        // Только whitespace → пустая строка
        assert_eq!(normalize(" \t\n\r "), "");

        // Пустая строка
        assert_eq!(normalize(""), "");

        // Unicode whitespace (неразрывный пробел U+00A0)
        assert_eq!(normalize("Hello\u{00A0}World"), "helloworld");

        // Регистр
        assert_eq!(normalize("HELLO WoRLd"), "helloworld");
        assert_eq!(normalize("  Rust  "), "rust");
        assert_eq!(normalize("a\tb"), "ab");
        assert_eq!(normalize("a  \t  b"), "ab");
    }

    #[test]
    fn test_use_after_free_no_ub_and_correct_result() {
        assert_eq!(unsafe { use_after_free() }, 84);
    }

    #[test]
    fn test_leak_buffer_regression() {
        assert_eq!(leak_buffer(&[]), 0);
        assert_eq!(leak_buffer(&[0, 0, 0]), 0);
        assert_eq!(leak_buffer(&[1, 2, 3]), 3);
        assert_eq!(leak_buffer(&[0, 1, 0, 255, 0]), 2);

        // Границы
        assert_eq!(leak_buffer(&[0]), 0);
        assert_eq!(leak_buffer(&[1]), 1);

        // Большой буфер (проверка, что не падает)
        let large = vec![1u8; 10_000];
        assert_eq!(leak_buffer(&large), 10_000);
    }
}
