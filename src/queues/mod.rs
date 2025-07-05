//! Queue Problems Module
//! 
//! This module contains common queue problems and their solutions:
//! - Implement Queue using Stacks
//! - Implement Stack using Queues
//! - Circular Queue
//! - Priority Queue
//! - Sliding Window Maximum
//! - BFS problems

use crate::{measure_time, assert_result};
use std::collections::{VecDeque, BinaryHeap, HashMap};

/// Problem: Implement Queue using Stacks
/// Implement a first in first out (FIFO) queue using only two stacks.
pub struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            input: Vec::new(),
            output: Vec::new(),
        }
    }
    
    pub fn push(&mut self, x: i32) {
        self.input.push(x);
    }
    
    pub fn pop(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(x) = self.input.pop() {
                self.output.push(x);
            }
        }
        self.output.pop().unwrap()
    }
    
    pub fn peek(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(x) = self.input.pop() {
                self.output.push(x);
            }
        }
        *self.output.last().unwrap()
    }
    
    pub fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}

/// Problem: Implement Stack using Queues
/// Implement a last in first out (LIFO) stack using only two queues.
pub struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }
    
    pub fn push(&mut self, x: i32) {
        self.queue2.push_back(x);
        while let Some(val) = self.queue1.pop_front() {
            self.queue2.push_back(val);
        }
        std::mem::swap(&mut self.queue1, &mut self.queue2);
    }
    
    pub fn pop(&mut self) -> i32 {
        self.queue1.pop_front().unwrap()
    }
    
    pub fn top(&self) -> i32 {
        *self.queue1.front().unwrap()
    }
    
    pub fn empty(&self) -> bool {
        self.queue1.is_empty()
    }
}

/// Problem: Circular Queue
/// Design your implementation of the circular queue.
pub struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        MyCircularQueue {
            data: vec![0; k as usize],
            head: 0,
            tail: 0,
            size: 0,
            capacity: k as usize,
        }
    }
    
    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        
        self.data[self.tail] = value;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }
    
    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        true
    }
    
    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head]
        }
    }
    
    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.tail + self.capacity - 1) % self.capacity]
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

/// Problem: Priority Queue Implementation
/// A simple priority queue using BinaryHeap
pub struct PriorityQueue<T> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }
    
    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }
    
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

/// Problem: Sliding Window Maximum (using Deque)
/// Find the maximum element in each sliding window of size k.
pub fn max_sliding_window_deque(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }
    
    let mut result = Vec::new();
    let mut deque = VecDeque::new();
    
    for i in 0..nums.len() {
        // Remove elements outside the window
        while let Some(&front) = deque.front() {
            if front <= i - k {
                deque.pop_front();
            } else {
                break;
            }
        }
        
        // Remove smaller elements from the back
        while let Some(&back) = deque.back() {
            if nums[back] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        
        deque.push_back(i);
        
        // Add maximum to result if window is complete
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    
    result
}

/// Problem: Number of Islands (BFS)
/// Count the number of islands in a 2D grid.
pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                count += 1;
                bfs_islands(grid, i, j);
            }
        }
    }
    
    count
}

fn bfs_islands(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut queue = VecDeque::new();
    queue.push_back((row, col));
    grid[row][col] = '0';
    
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in directions.iter() {
            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;
            
            if new_r >= 0 && new_r < rows as i32 && 
               new_c >= 0 && new_c < cols as i32 && 
               grid[new_r as usize][new_c as usize] == '1' {
                grid[new_r as usize][new_c as usize] = '0';
                queue.push_back((new_r as usize, new_c as usize));
            }
        }
    }
}

/// Problem: Open the Lock
/// Find the minimum number of turns required to open the lock.
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited = std::collections::HashSet::new();
    let deadends: std::collections::HashSet<String> = deadends.into_iter().collect();
    
    if deadends.contains("0000") {
        return -1;
    }
    
    let mut queue = VecDeque::new();
    queue.push_back(("0000".to_string(), 0));
    visited.insert("0000".to_string());
    
    while let Some((current, turns)) = queue.pop_front() {
        if current == target {
            return turns;
        }
        
        for i in 0..4 {
            for delta in [-1, 1] {
                let mut chars: Vec<char> = current.chars().collect();
                let digit = chars[i].to_digit(10).unwrap() as i32;
                let new_digit = (digit + delta + 10) % 10;
                chars[i] = std::char::from_digit(new_digit as u32, 10).unwrap();
                let next = chars.into_iter().collect::<String>();
                
                if !visited.contains(&next) && !deadends.contains(&next) {
                    visited.insert(next.clone());
                    queue.push_back((next, turns + 1));
                }
            }
        }
    }
    
    -1
}

/// Problem: Perfect Squares
/// Find the least number of perfect square numbers that sum to n.
pub fn num_squares(n: i32) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    
    queue.push_back((n, 0));
    visited.insert(n);
    
    while let Some((current, steps)) = queue.pop_front() {
        if current == 0 {
            return steps;
        }
        
        let sqrt = (current as f64).sqrt() as i32;
        for i in 1..=sqrt {
            let next = current - i * i;
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }
    
    0
}

/// Run all queue problem examples
pub fn run_examples() {
    println!("Running Queue Problem Examples...");
    
    // MyQueue
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("MyQueue: push 1,2,3 -> peek: {}", queue.peek());
    assert_result(queue.peek(), 1, "MyQueue Peek");
    
    // MyStack
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("MyStack: push 1,2,3 -> top: {}", stack.top());
    assert_result(stack.top(), 3, "MyStack Top");
    
    // Circular Queue
    let mut cq = MyCircularQueue::new(3);
    cq.en_queue(1);
    cq.en_queue(2);
    cq.en_queue(3);
    println!("Circular Queue: enqueue 1,2,3 -> rear: {}", cq.rear());
    assert_result(cq.rear(), 3, "Circular Queue Rear");
    
    // Priority Queue
    let mut pq = PriorityQueue::new();
    pq.push(3);
    pq.push(1);
    pq.push(4);
    pq.push(2);
    println!("Priority Queue: push 3,1,4,2 -> peek: {}", pq.peek().unwrap());
    assert_result(*pq.peek().unwrap(), 4, "Priority Queue Peek");
    
    // Sliding Window Maximum
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = measure_time("Max Sliding Window Deque", || max_sliding_window_deque(&nums, k));
    println!("Max Sliding Window Deque: {:?}, k={} -> {:?}", nums, k, result);
    assert_result(result, vec![3, 3, 5, 5, 6, 7], "Max Sliding Window Deque");
    
    // Number of Islands
    let mut grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let result = measure_time("Number of Islands", || num_islands(&mut grid));
    println!("Number of Islands: {}", result);
    assert_result(result, 3, "Number of Islands");
    
    // Open the Lock
    let deadends = vec!["0201".to_string(), "0101".to_string(), "0102".to_string(), "1212".to_string(), "2002".to_string()];
    let target = "0202".to_string();
    let result = measure_time("Open the Lock", || open_lock(deadends, target));
    println!("Open the Lock: target=0202 -> {}", result);
    assert_result(result, 6, "Open the Lock");
    
    // Perfect Squares
    let n = 12;
    let result = measure_time("Perfect Squares", || num_squares(n));
    println!("Perfect Squares: n={} -> {}", n, result);
    assert_result(result, 3, "Perfect Squares");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_queue() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.empty(), true);
    }
    
    #[test]
    fn test_my_stack() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.empty(), true);
    }
    
    #[test]
    fn test_circular_queue() {
        let mut cq = MyCircularQueue::new(3);
        assert_eq!(cq.en_queue(1), true);
        assert_eq!(cq.en_queue(2), true);
        assert_eq!(cq.en_queue(3), true);
        assert_eq!(cq.en_queue(4), false);
        assert_eq!(cq.rear(), 3);
        assert_eq!(cq.is_full(), true);
        assert_eq!(cq.de_queue(), true);
        assert_eq!(cq.en_queue(4), true);
        assert_eq!(cq.rear(), 4);
    }
    
    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        pq.push(3);
        pq.push(1);
        pq.push(4);
        assert_eq!(pq.peek(), Some(&4));
        assert_eq!(pq.pop(), Some(4));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), Some(1));
    }
    
    #[test]
    fn test_max_sliding_window_deque() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(max_sliding_window_deque(&nums, 3), vec![3, 3, 5, 5, 6, 7]);
    }
    
    #[test]
    fn test_num_islands() {
        let mut grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(num_islands(&mut grid), 3);
    }
    
    #[test]
    fn test_open_lock() {
        let deadends = vec!["0201".to_string(), "0101".to_string(), "0102".to_string(), "1212".to_string(), "2002".to_string()];
        let target = "0202".to_string();
        assert_eq!(open_lock(deadends, target), 6);
    }
    
    #[test]
    fn test_num_squares() {
        assert_eq!(num_squares(12), 3);
        assert_eq!(num_squares(13), 2);
    }
} 