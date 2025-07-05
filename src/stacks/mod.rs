//! Stack Problems Module
//! 
//! This module contains common stack problems and their solutions:
//! - Valid Parentheses
//! - Min Stack
//! - Evaluate Reverse Polish Notation
//! - Largest Rectangle in Histogram
//! - Next Greater Element
//! - Monotonic Stack problems

use crate::{measure_time, assert_result};

/// Problem: Valid Parentheses
/// Check if a string of parentheses is valid.
pub fn is_valid_parentheses(s: &str) -> bool {
    let mut stack = Vec::new();
    
    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => return false,
        }
    }
    
    stack.is_empty()
}

/// Min Stack implementation
/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }
    
    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        if let Some(val) = self.stack.pop() {
            if Some(&val) == self.min_stack.last() {
                self.min_stack.pop();
            }
            Some(val)
        } else {
            None
        }
    }
    
    pub fn top(&self) -> Option<&i32> {
        self.stack.last()
    }
    
    pub fn get_min(&self) -> Option<&i32> {
        self.min_stack.last()
    }
}

/// Problem: Evaluate Reverse Polish Notation
/// Evaluate the value of an arithmetic expression in Reverse Polish Notation.
pub fn eval_rpn(tokens: &[String]) -> i32 {
    let mut stack = Vec::new();
    
    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
    }
    
    stack.pop().unwrap()
}

/// Problem: Largest Rectangle in Histogram
/// Find the largest rectangle area in a histogram.
pub fn largest_rectangle_area(heights: &[i32]) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;
    let mut i = 0;
    
    while i < heights.len() {
        if stack.is_empty() || heights[*stack.last().unwrap()] <= heights[i] {
            stack.push(i);
            i += 1;
        } else {
            let top = stack.pop().unwrap();
            let area = if stack.is_empty() {
                heights[top] * i as i32
            } else {
                heights[top] * (i - stack.last().unwrap() - 1) as i32
            };
            max_area = max_area.max(area);
        }
    }
    
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let area = if stack.is_empty() {
            heights[top] * i as i32
        } else {
            heights[top] * (i - stack.last().unwrap() - 1) as i32
        };
        max_area = max_area.max(area);
    }
    
    max_area
}

/// Problem: Next Greater Element
/// Find the next greater element for each element in the array.
pub fn next_greater_elements(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];
    let mut stack = Vec::new();
    
    for i in 0..nums.len() {
        while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
            let index = stack.pop().unwrap();
            result[index] = nums[i];
        }
        stack.push(i);
    }
    
    result
}

/// Problem: Daily Temperatures
/// Find how many days you would have to wait until a warmer temperature.
pub fn daily_temperatures(temperatures: &[i32]) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack = Vec::new();
    
    for i in 0..temperatures.len() {
        while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
            let index = stack.pop().unwrap();
            result[index] = (i - index) as i32;
        }
        stack.push(i);
    }
    
    result
}

/// Problem: Simplify Path
/// Simplify a Unix-style absolute path.
pub fn simplify_path(path: &str) -> String {
    let mut stack = Vec::new();
    
    for component in path.split('/') {
        match component {
            "" | "." => continue,
            ".." => {
                stack.pop();
            }
            _ => stack.push(component),
        }
    }
    
    if stack.is_empty() {
        "/".to_string()
    } else {
        "/".to_string() + &stack.join("/")
    }
}

/// Problem: Decode String
/// Decode an encoded string.
pub fn decode_string(s: &str) -> String {
    let mut stack = Vec::new();
    let mut current_string = String::new();
    let mut current_number = 0;
    
    for ch in s.chars() {
        match ch {
            '[' => {
                stack.push((current_string.clone(), current_number));
                current_string.clear();
                current_number = 0;
            }
            ']' => {
                let (prev_string, repeat_count) = stack.pop().unwrap();
                current_string = prev_string + &current_string.repeat(repeat_count as usize);
            }
            '0'..='9' => {
                current_number = current_number * 10 + (ch as u8 - b'0') as i32;
            }
            _ => {
                current_string.push(ch);
            }
        }
    }
    
    current_string
}

/// Run all stack problem examples
pub fn run_examples() {
    println!("Running Stack Problem Examples...");
    
    // Valid Parentheses
    let valid = "()[]{}";
    let invalid = "([)]";
    let is_valid = measure_time("Valid Parentheses", || is_valid_parentheses(valid));
    let is_invalid = measure_time("Invalid Parentheses", || is_valid_parentheses(invalid));
    println!("Valid Parentheses: '{}' -> {}", valid, is_valid);
    println!("Invalid Parentheses: '{}' -> {}", invalid, is_invalid);
    assert_result(is_valid, true, "Valid Parentheses");
    assert_result(is_invalid, false, "Invalid Parentheses");
    
    // Min Stack
    let mut min_stack = MinStack::new();
    min_stack.push(3);
    min_stack.push(5);
    min_stack.push(2);
    min_stack.push(1);
    println!("Min Stack: push 3,5,2,1 -> min: {}", min_stack.get_min().unwrap());
    assert_result(*min_stack.get_min().unwrap(), 1, "Min Stack");
    
    // Evaluate RPN
    let tokens = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
    let result = measure_time("Evaluate RPN", || eval_rpn(&tokens));
    println!("Evaluate RPN: {:?} -> {}", tokens, result);
    assert_result(result, 9, "Evaluate RPN");
    
    // Largest Rectangle in Histogram
    let heights = vec![2, 1, 5, 6, 2, 3];
    let result = measure_time("Largest Rectangle", || largest_rectangle_area(&heights));
    println!("Largest Rectangle: {:?} -> {}", heights, result);
    assert_result(result, 10, "Largest Rectangle");
    
    // Next Greater Element
    let nums = vec![4, 1, 2];
    let result = measure_time("Next Greater Element", || next_greater_elements(&nums));
    println!("Next Greater Element: {:?} -> {:?}", nums, result);
    assert_result(result, vec![-1, 2, -1], "Next Greater Element");
    
    // Daily Temperatures
    let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let result = measure_time("Daily Temperatures", || daily_temperatures(&temps));
    println!("Daily Temperatures: {:?} -> {:?}", temps, result);
    assert_result(result, vec![1, 1, 4, 2, 1, 1, 0, 0], "Daily Temperatures");
    
    // Simplify Path
    let path = "/home//foo/";
    let result = measure_time("Simplify Path", || simplify_path(path));
    println!("Simplify Path: '{}' -> '{}'", path, result);
    assert_result(result, "/home/foo".to_string(), "Simplify Path");
    
    // Decode String
    let s = "3[a]2[bc]";
    let result = measure_time("Decode String", || decode_string(s));
    println!("Decode String: '{}' -> '{}'", s, result);
    assert_result(result, "aaabcbc".to_string(), "Decode String");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_parentheses() {
        assert_eq!(is_valid_parentheses("()"), true);
        assert_eq!(is_valid_parentheses("()[]{}"), true);
        assert_eq!(is_valid_parentheses("(]"), false);
        assert_eq!(is_valid_parentheses("([)]"), false);
    }
    
    #[test]
    fn test_min_stack() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(5);
        stack.push(2);
        stack.push(1);
        
        assert_eq!(*stack.get_min().unwrap(), 1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(*stack.get_min().unwrap(), 2);
    }
    
    #[test]
    fn test_eval_rpn() {
        let tokens = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
        assert_eq!(eval_rpn(&tokens), 9);
    }
    
    #[test]
    fn test_largest_rectangle_area() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(&heights), 10);
    }
    
    #[test]
    fn test_next_greater_elements() {
        let nums = vec![4, 1, 2];
        assert_eq!(next_greater_elements(&nums), vec![-1, 2, -1]);
    }
    
    #[test]
    fn test_daily_temperatures() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(daily_temperatures(&temps), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }
    
    #[test]
    fn test_simplify_path() {
        assert_eq!(simplify_path("/home//foo/"), "/home/foo");
        assert_eq!(simplify_path("/../"), "/");
    }
    
    #[test]
    fn test_decode_string() {
        assert_eq!(decode_string("3[a]2[bc]"), "aaabcbc");
        assert_eq!(decode_string("3[a2[c]]"), "accaccacc");
    }
} 