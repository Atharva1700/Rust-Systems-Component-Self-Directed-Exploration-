use std::time::Instant;

// Rust: ownership model prevents use-after-free, double-free, data races at compile time
struct DataProcessor {
    buffer: Vec<i32>,
}

impl DataProcessor {
    fn new(capacity: usize) -> Self {
        DataProcessor {
            buffer: Vec::with_capacity(capacity),
        }
    }

    // Ownership: take value, process, return—no dangling pointers
    fn process(&mut self, data: Vec<i32>) -> ProcessResult {
        self.buffer.clear();
        self.buffer.extend(data);

        let sum: i64 = self.buffer.iter().map(|&x| x as i64).sum();
        let mean = if self.buffer.is_empty() {
            0.0
        } else {
            sum as f64 / self.buffer.len() as f64
        };

        ProcessResult { sum, mean }
    }

    fn cleanup(&mut self) {
        self.buffer.clear();
    }
}

#[derive(Debug)]
struct ProcessResult {
    sum: i64,
    mean: f64,
}

fn main() {
    let mut processor = DataProcessor::new(1_000_000);

    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let start = Instant::now();
    let result = processor.process(test_data);
    let duration = start.elapsed();

    println!("Rust Implementation:");
    println!("  Sum: {}", result.sum);
    println!("  Mean: {:.2}", result.mean);
    println!("  Time: {:?}", duration);
    println!("  Memory Safety: ✓ (compile-time ownership checks prevent use-after-free)");

    processor.cleanup();
}
