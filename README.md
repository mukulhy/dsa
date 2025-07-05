
# DSA Practice in Rust 🦀

A comprehensive Rust project for practicing Data Structures and Algorithms problems. This project is organized by topics and includes solutions, tests, and performance measurements.

## 🚀 Features

- **Organized by Topics**: Each DSA topic has its own module
- **Complete Solutions**: Working implementations with detailed comments
- **Comprehensive Tests**: Unit tests for all problems
- **Performance Measurement**: Built-in timing utilities
- **Notes System**: Space for your own notes and solutions
- **Modern Rust**: Uses Rust 2021 edition with best practices

## 📁 Project Structure

```
dsa_practice/
├── src/
│   ├── arrays/           # Array problems (Two Sum, Kadane's, etc.)
│   ├── linked_lists/     # Linked list problems (Reverse, Merge, etc.)
│   ├── stacks/           # Stack problems (Valid Parentheses, Min Stack, etc.)
│   ├── queues/           # Queue problems (Circular Queue, BFS, etc.)
│   ├── trees/            # Tree problems (BST, Traversal, etc.)
│   ├── graphs/           # Graph problems (DFS, BFS, Shortest Path, etc.)
│   ├── sorting/          # Sorting algorithms (QuickSort, MergeSort, etc.)
│   ├── searching/        # Searching algorithms (Binary Search, etc.)
│   ├── dynamic_programming/ # DP problems (Fibonacci, LCS, etc.)
│   ├── strings/          # String problems (Pattern Matching, etc.)
│   ├── math/             # Mathematical algorithms (Prime, GCD, etc.)
│   ├── notes/            # Your personal notes and solutions
│   └── main.rs           # Main entry point with examples
├── Cargo.toml            # Project dependencies
└── README.md             # This file
```

## 🛠️ Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone or navigate to the project directory:
```bash
cd dsa_practice
```

2. Build the project:
```bash
cargo build
```

3. Run the examples:
```bash
cargo run
```

4. Run tests:
```bash
cargo test
```

5. Run benchmarks:
```bash
cargo bench
```

## 📚 Topics Covered

### Arrays
- Two Sum
- Maximum Subarray Sum (Kadane's Algorithm)
- Move Zeroes
- Container With Most Water
- Trapping Rain Water
- Sliding Window problems

### Linked Lists
- Reverse Linked List
- Detect Cycle
- Merge Two Sorted Lists
- Remove Nth Node From End
- Add Two Numbers
- Palindrome Linked List

### Stacks
- Valid Parentheses
- Min Stack
- Evaluate Reverse Polish Notation
- Largest Rectangle in Histogram
- Next Greater Element
- Monotonic Stack problems

### Queues
- Implement Queue using Stacks
- Implement Stack using Queues
- Circular Queue
- Priority Queue
- Sliding Window Maximum
- BFS problems

### Trees (Coming Soon)
- Binary Tree Traversal
- Binary Search Tree operations
- AVL Tree
- B-Tree
- Tree problems

### Graphs (Coming Soon)
- DFS and BFS
- Shortest Path algorithms
- Topological Sort
- Minimum Spanning Tree
- Graph problems

### Sorting (Coming Soon)
- Bubble Sort
- Selection Sort
- Insertion Sort
- Merge Sort
- Quick Sort
- Heap Sort

### Searching (Coming Soon)
- Linear Search
- Binary Search
- Ternary Search
- Jump Search

### Dynamic Programming (Coming Soon)
- Fibonacci
- Longest Common Subsequence
- Knapsack Problem
- Edit Distance
- Matrix Chain Multiplication

### Strings (Coming Soon)
- Pattern Matching
- String Manipulation
- Regular Expressions
- String algorithms

### Math (Coming Soon)
- Prime Numbers
- GCD and LCM
- Number Theory
- Mathematical algorithms

## 🎯 How to Use This Project

### 1. Study the Problems
Each module contains well-documented problems with:
- Problem description
- Solution approach
- Time and space complexity
- Implementation

### 2. Run Examples
```bash
cargo run
```
This will run all the example problems and show their outputs.

### 3. Practice on Your Own
- Add your own solutions in the respective modules
- Create new problems in the appropriate directories
- Use the utility functions for timing and testing

### 4. Take Notes
Use the `notes/` directory to:
- Write your own solutions
- Document your learning
- Track your progress
- Add problem-solving strategies

### 5. Test Your Solutions
```bash
cargo test
```
All problems come with comprehensive tests.

## 🛠️ Utility Functions

The project includes several utility functions to help with practice:

### Timing
```rust
use crate::measure_time;

let result = measure_time("My Algorithm", || {
    // Your algorithm here
    my_algorithm(input)
});
```

### Array Printing
```rust
use crate::print_array;

let arr = vec![1, 2, 3, 4, 5];
print_array(&arr, "My Array");
```

### Result Assertion
```rust
use crate::assert_result;

let actual = my_function(input);
let expected = expected_result;
assert_result(actual, expected, "Test Name");
```

## 📝 Adding Your Own Problems

1. **Choose the right module**: Add your problem to the appropriate topic directory
2. **Follow the pattern**: Use the existing problem structure as a template
3. **Add tests**: Include comprehensive unit tests
4. **Document**: Add clear comments explaining your approach
5. **Update main.rs**: Add your problem to the examples if desired

Example structure:
```rust
/// Problem: Your Problem Name
/// Brief description of the problem
pub fn your_solution(input: &[i32]) -> i32 {
    // Your implementation here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_solution() {
        // Your tests here
    }
}
```

## 🎓 Learning Path

### Beginner Level
1. Start with **Arrays** - fundamental data structure
2. Move to **Linked Lists** - understand pointers and references
3. Practice **Stacks** and **Queues** - LIFO and FIFO concepts

### Intermediate Level
4. Study **Trees** - hierarchical data structures
5. Learn **Graphs** - complex relationships
6. Master **Sorting** and **Searching** algorithms

### Advanced Level
7. Practice **Dynamic Programming** - optimization problems
8. Solve **String** problems - text processing
9. Explore **Math** algorithms - number theory

## 🔧 Customization

### Adding Dependencies
Edit `Cargo.toml` to add new dependencies:
```toml
[dependencies]
your_dependency = "1.0"
```

### Creating New Modules
1. Create a new directory in `src/`
2. Add a `mod.rs` file
3. Update `main.rs` to include the module
4. Add your problems and tests

## 📊 Performance Tips

- Use `cargo bench` to measure performance
- Profile your code with `cargo flamegraph`
- Compare different approaches
- Document time and space complexity

## 🤝 Contributing

Feel free to:
- Add new problems
- Improve existing solutions
- Fix bugs
- Add better documentation
- Suggest new features

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [LeetCode](https://leetcode.com/)
- [HackerRank](https://www.hackerrank.com/)
- [GeeksforGeeks](https://www.geeksforgeeks.org/)

## 🎯 Goals

- Master fundamental data structures
- Understand algorithm design patterns
- Improve problem-solving skills
- Learn Rust programming
- Build a comprehensive DSA knowledge base

## 📈 Progress Tracking

Use the `notes/` directory to track your progress:
- Create a learning log
- Document solved problems
- Note important concepts
- Track time spent on each topic

---

**Happy Coding! 🚀**

Remember: Practice makes perfect. Start with simple problems and gradually work your way up to more complex ones. Don't forget to understand the underlying concepts, not just memorize solutions. 

