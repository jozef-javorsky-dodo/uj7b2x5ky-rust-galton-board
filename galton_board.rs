use std::io::{stdin, stdout, Write};

fn main() {
    let rows = get_input("Enter the number of rows (default 9): ", 9);
    let balls = get_input("Enter the number of balls (default 256): ", 256);
    let distribution = simulate(rows, balls);
    let (mean, std_dev, variance) = statistics(&distribution);
    println!("Galton Board Simulation with {} balls and {} rows:", balls, rows);
    visualize(&distribution, mean, std_dev);
    println!("Mean: {:.2}", mean);
    println!("Standard Deviation: {:.2}", std_dev);
    println!("Variance: {:.2}", variance);
}

fn get_input(prompt: &str, default: usize) -> usize {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or_else(|_| default)
}

fn simulate(rows: usize, balls: usize) -> Vec<usize> {
    let mut distribution = vec![0; rows + 1];
    for _ in 0..balls {
        let mut bin = 0;
        for _ in 0..rows {
            if rand::random() { bin += 1; }
        }
        distribution[bin] += 1;
    }
    distribution
}

fn statistics(distribution: &[usize]) -> (f64, f64, f64) {
    let total = distribution.iter().sum::<usize>() as f64;
    if total == 0.0 { return (0.0, 0.0, 0.0); }
    let mean = distribution.iter().enumerate().map(|(i, &count)| (i as f64) * (count as f64)).sum::<f64>() / total;
    let variance = distribution.iter().enumerate().map(|(i, &count)| ((i as f64) - mean).powi(2) * (count as f64)).sum::<f64>() / total;
    let std_dev = variance.sqrt();
    (mean, std_dev, variance)
}

fn visualize(distribution: &[usize], mean: f64, std_dev: f64) {
    let max = distribution.iter().max().unwrap_or(&0);
    let width = 40; 
    for (i, &count) in distribution.iter().enumerate() {
        let bar_width = if *max > 0 { (count as f64 / *max as f64 * width as f64) as usize } else { 0 };
        let outlier = (i as f64) < (mean - 1.5 * std_dev) || (i as f64) > (mean + 1.5 * std_dev);
        let bar = (0..bar_width).map(|_| if outlier { '~' } else { '#' }).collect::<String>();
        println!("{:2}: {} ({})", i, bar, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let distribution = simulate(5, 100);
        assert_eq!(distribution.len(), 6);
        assert_eq!(distribution.iter().sum::<usize>(), 100);
    }

    #[test]
    fn test_statistics() {
        let distribution = vec![1, 4, 6, 4, 1];
        let (mean, std_dev, variance) = statistics(&distribution);
        assert!((mean - 2.0).abs() < 1e-10);
        assert!((std_dev - 1.0).abs() < 1e-10);
        assert!((variance - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_get_input_valid() {
        let mut input = "5\n".as_bytes();
        std::io::stdin().read_from(&mut input).unwrap();
        let result = get_input("Test p-inp: ", 10);
        assert_eq!(result, 5); 
    }
    
    #[test]
    fn test_get_input_invalid() {
        let mut input = "abc\n".as_bytes(); 
        std::io::stdin().read_from(&mut input).unwrap();
        let result = get_input("Test p-inp: ", 10);
        assert_eq!(result, 10); 
    }
}
