import time

class DataProcessor:
    def __init__(self, capacity):
        self.buffer = []
        self.capacity = capacity

    def process(self, data):
        """Process data - in Python, no compile-time ownership checks."""
        self.buffer.clear()
        self.buffer.extend(data)

        total_sum = sum(self.buffer)
        mean = total_sum / len(self.buffer) if self.buffer else 0

        return {"sum": total_sum, "mean": mean}

    def cleanup(self):
        self.buffer.clear()


if __name__ == "__main__":
    processor = DataProcessor(1_000_000)
    test_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    start = time.perf_counter()
    result = processor.process(test_data)
    duration = time.perf_counter() - start

    print("Python Implementation:")
    print(f"  Sum: {result['sum']}")
    print(f"  Mean: {result['mean']:.2f}")
    print(f"  Time: {duration:.6f}s")
    print("  Memory Safety: ✗ (runtime checks, GC overhead, use-after-free possible)")

    processor.cleanup()
