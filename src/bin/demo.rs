use broken_app::{
    algo, average_positive, concurrency, leak_buffer, normalize, sum_even, use_after_free,
};

fn main() {
    let nums = [1, 2, 3, 4];
    println!("sum_even: {}", sum_even(&nums));

    let data = [1_u8, 0, 2, 3];
    println!("non-zero bytes: {}", leak_buffer(&data));

    let text = " Hello World ";
    println!("normalize: {}", normalize(text));

    let nums_for_average = &[1, 2, 3, 4];
    println!("normalize: {}", average_positive(nums_for_average));

    println!("normalize: {}", unsafe { use_after_free() });

    let fib = algo::slow_fib(20);
    println!("fib(20): {}", fib);

    let uniq = algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]);
    println!("dedup: {:?}", uniq);

    let iterations = 1000;
    let threads = 10;
    let result = concurrency::race_increment(iterations, threads);
    println!("race_increment({iterations}, {threads}): {}", result);

    concurrency::reset_counter();
    println!("read_after_sleep: {}", concurrency::read_after_sleep());
}
