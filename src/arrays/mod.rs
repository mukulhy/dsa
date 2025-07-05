//! Array Problems Module
//! 
//! This module contains common array problems and their solutions:
//! - Two Sum
//! - Maximum Subarray Sum (Kadane's Algorithm)
//! - Move Zeroes
//! - Container With Most Water
//! - Trapping Rain Water
//! - Sliding Window problems

use crate::{measure_time, print_array, assert_result};

/// Problem: Two Sum
/// Given an array of integers nums and an integer target, 
/// return indices of the two numbers such that they add up to target.
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = std::collections::HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.insert(num, i);
    }
    None
}

/// Problem: Maximum Subarray Sum (Kadane's Algorithm)
/// Find the contiguous subarray with the largest sum.
pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    
    let mut max_so_far = nums[0];
    let mut max_ending_here = nums[0];
    
    for &num in nums.iter().skip(1) {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }
    
    max_so_far
}

/// Problem: Move Zeroes
/// Move all zeros to the end while maintaining the relative order of non-zero elements.
pub fn move_zeroes(nums: &mut [i32]) {
    let mut non_zero_index = 0;
    
    // Move all non-zero elements to the front
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[non_zero_index] = nums[i];
            non_zero_index += 1;
        }
    }
    
    // Fill the rest with zeros
    for i in non_zero_index..nums.len() {
        nums[i] = 0;
    }
}

/// Problem: Container With Most Water
/// Find two lines that together with the x-axis forms a container that would hold the most water.
pub fn max_area(height: &[i32]) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    
    while left < right {
        let width = (right - left) as i32;
        let h = height[left].min(height[right]);
        let area = width * h;
        max_area = max_area.max(area);
        
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    
    max_area
}

/// Problem: Trapping Rain Water
/// Calculate how much water can be trapped between bars.
pub fn trap_rain_water(height: &[i32]) -> i32 {
    if height.len() < 3 {
        return 0;
    }
    
    let mut left_max = vec![0; height.len()];
    let mut right_max = vec![0; height.len()];
    
    // Calculate left max heights
    left_max[0] = height[0];
    for i in 1..height.len() {
        left_max[i] = left_max[i-1].max(height[i]);
    }
    
    // Calculate right max heights
    right_max[height.len()-1] = height[height.len()-1];
    for i in (0..height.len()-1).rev() {
        right_max[i] = right_max[i+1].max(height[i]);
    }
    
    // Calculate trapped water
    let mut water = 0;
    for i in 0..height.len() {
        water += (left_max[i].min(right_max[i]) - height[i]).max(0);
    }
    
    water
}

/// Problem: Sliding Window Maximum
/// Find the maximum element in each sliding window of size k.
pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }
    
    let mut result = Vec::new();
    let mut deque = std::collections::VecDeque::new();
    
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

/// Run all array problem examples
pub fn run_examples() {
    println!("Running Array Problem Examples...");
    
    // Two Sum
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = measure_time("Two Sum", || two_sum(&nums, target));
    println!("Two Sum: {:?} -> {:?}", (nums, target), result);
    assert_result(result, Some((0, 1)), "Two Sum");
    
    // Maximum Subarray Sum
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = measure_time("Max Subarray Sum", || max_subarray_sum(&nums));
    println!("Max Subarray Sum: {:?} -> {}", nums, result);
    assert_result(result, 6, "Max Subarray Sum");
    
    // Move Zeroes
    let mut nums = vec![0, 1, 0, 3, 12];
    print_array(&nums, "Before Move Zeroes");
    measure_time("Move Zeroes", || move_zeroes(&mut nums));
    print_array(&nums, "After Move Zeroes");
    
    // Container With Most Water
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = measure_time("Max Area", || max_area(&height));
    println!("Max Area: {:?} -> {}", height, result);
    assert_result(result, 49, "Max Area");
    
    // Trapping Rain Water
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result = measure_time("Trap Rain Water", || trap_rain_water(&height));
    println!("Trap Rain Water: {:?} -> {}", height, result);
    assert_result(result, 6, "Trap Rain Water");
    
    // Sliding Window Maximum
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = measure_time("Max Sliding Window", || max_sliding_window(&nums, k));
    println!("Max Sliding Window: {:?}, k={} -> {:?}", nums, k, result);
    assert_result(result, vec![3, 3, 5, 5, 6, 7], "Max Sliding Window");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(&nums, 9), Some((0, 1)));
        assert_eq!(two_sum(&nums, 26), Some((2, 3)));
        assert_eq!(two_sum(&nums, 10), None);
    }
    
    #[test]
    fn test_max_subarray_sum() {
        assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_subarray_sum(&[1]), 1);
        assert_eq!(max_subarray_sum(&[5, 4, -1, 7, 8]), 23);
    }
    
    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
    
    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(&height), 49);
    }
    
    #[test]
    fn test_trap_rain_water() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_rain_water(&height), 6);
    }
    
    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(max_sliding_window(&nums, 3), vec![3, 3, 5, 5, 6, 7]);
    }
} 