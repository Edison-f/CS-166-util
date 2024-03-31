///
/// # Arguments
///
/// * `frac`: Chance that y_i is in the dictionary
/// * `dict_size`: log_2(dictionary size)
/// * `password_count`: Number of passwords
///
/// returns: log_2(Expected work)
///
pub(crate) fn probability(frac: f64, dict_size: u32, password_count: u32) -> f64 {
    let mut result: f64 = 0f64;
    for i in 0..password_count {
        let add = f64::powf(1f64 - frac, i as f64) * frac * (i as f64 * f64::powf(2f64, dict_size as f64) + u64::pow(2, dict_size - 1) as f64);
        println!("{}", add);
        result += add;
    }
    f64::log2(result)
}