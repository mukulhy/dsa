//! DSA Practice - A comprehensive Rust project for practicing Data Structures and Algorithms
//! 
//! This project is organized by topics:
//! - Arrays: Basic array operations, two pointers, sliding window
//! - Linked Lists: Single, double, circular linked lists
//! - Stacks: Stack operations, monotonic stack problems
//! - Queues: Queue operations, priority queues
//! - Trees: Binary trees, BST, AVL, B-trees
//! - Graphs: DFS, BFS, shortest path algorithms
//! - Sorting: Various sorting algorithms
//! - Searching: Binary search, linear search
//! - Dynamic Programming: Memoization, tabulation
//! - Strings: String manipulation, pattern matching
//! - Math: Mathematical algorithms and number theory

mod arrays;
mod linked_lists;
mod stacks;
mod queues;
mod trees;
mod graphs;
mod sorting;
mod searching;
mod dynamic_programming;
mod strings;
mod math;
mod notes;

use std::collections::HashMap;

fn main() {
    println!("🚀 Welcome to DSA Practice in Rust!");
    println!("=====================================");
    
    // Example: Run some array problems
    println!("\n📊 Array Problems:");
    arrays::run_examples();
    
    // Example: Run some linked list problems
    println!("\n🔗 Linked List Problems:");
    linked_lists::run_examples();
    
    // Example: Run some stack problems
    println!("\n📚 Stack Problems:");
    stacks::run_examples();
    
    // Example: Run some queue problems
    println!("\n🎯 Queue Problems:");
    queues::run_examples();
    
    // Example: Run notes and learning progress
    println!("\n📝 Notes and Learning Progress:");
    notes::run_notes_examples();
    
    println!("\n✅ All examples completed! Check individual modules for more problems.");
    println!("📝 Notes and solutions are available in the notes/ directory.");
}

/// Utility function to measure execution time of algorithms
pub fn measure_time<F, T>(name: &str, f: F) -> T 
where 
    F: FnOnce() -> T 
{
    use std::time::Instant;
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("⏱️  {} took: {:?}", name, duration);
    result
}

/// Utility function to print arrays in a nice format
pub fn print_array<T: std::fmt::Display>(arr: &[T], name: &str) {
    println!("{}: [{:?}]", name, arr.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
}

/// Utility function to compare results
pub fn assert_result<T: PartialEq + std::fmt::Debug>(actual: T, expected: T, test_name: &str) {
    if actual == expected {
        println!("✅ {}: PASSED", test_name);
    } else {
        println!("❌ {}: FAILED - Expected {:?}, got {:?}", test_name, expected, actual);
    }
}
