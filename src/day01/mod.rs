mod input;

pub fn day01() {
    let lines = input::INPUT.split_whitespace();
    let window_size = 3;
    let mut increase_count = 0;

    let numbers = lines.map(|s: &str|s.parse().unwrap()).collect::<Vec<i32>>();
    for i in window_size..numbers.len() {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for j in 0..window_size {
            sum1 += numbers[i - j - 1];
            sum2 += numbers[i - j];
        }
        if sum1 < sum2 {
            increase_count += 1
        }
    }
    println!("DAY 01: Increase count: {}", increase_count);
}