//! Linked List Problems Module
//! 
//! This module contains common linked list problems and their solutions:
//! - Reverse Linked List
//! - Detect Cycle
//! - Merge Two Sorted Lists
//! - Remove Nth Node From End
//! - Add Two Numbers
//! - Palindrome Linked List

use crate::{measure_time, assert_result};

/// Definition for singly-linked list node
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

/// Helper function to create a linked list from a vector
pub fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    
    let mut head = Box::new(ListNode::new(values[0]));
    let mut current = &mut head;
    
    for &val in values.iter().skip(1) {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    Some(head)
}

/// Helper function to convert linked list to vector
pub fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    
    result
}

/// Problem: Reverse Linked List
/// Reverse a singly linked list.
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;
    
    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    
    prev
}

/// Problem: Detect Cycle in Linked List
/// Return true if there is a cycle in the linked list.
pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }
    
    let mut slow = &head;
    let mut fast = &head;
    
    while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
        slow = &slow_node.next;
        
        if let Some(fast_next) = &fast_node.next {
            fast = &fast_next.next;
        } else {
            return false;
        }
        
        if std::ptr::eq(slow.as_ref().unwrap(), fast.as_ref().unwrap()) {
            return true;
        }
    }
    
    false
}

/// Problem: Merge Two Sorted Lists
/// Merge two sorted linked lists and return it as a sorted list.
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(mut l1), Some(mut l2)) => {
            if l1.val <= l2.val {
                l1.next = merge_two_lists(l1.next, Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists(Some(l1), l2.next);
                Some(l2)
            }
        }
    }
}

/// Problem: Remove Nth Node From End of List
/// Remove the nth node from the end of the list and return its head.
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    
    let mut first = &mut dummy;
    let mut second = &mut dummy;
    
    // Move first pointer n+1 steps ahead
    for _ in 0..=n {
        first = first.next.as_mut().unwrap();
    }
    
    // Move both pointers until first reaches end
    while first.next.is_some() {
        first = first.next.as_mut().unwrap();
        second = second.next.as_mut().unwrap();
    }
    
    // Remove the nth node
    second.next = second.next.as_mut().unwrap().next.take();
    
    dummy.next
}

/// Problem: Add Two Numbers
/// Add two numbers represented by linked lists.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    let mut carry = 0;
    
    let mut p1 = &l1;
    let mut p2 = &l2;
    
    while p1.is_some() || p2.is_some() || carry > 0 {
        let val1 = p1.as_ref().map_or(0, |node| node.val);
        let val2 = p2.as_ref().map_or(0, |node| node.val);
        
        let sum = val1 + val2 + carry;
        carry = sum / 10;
        
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        
        p1 = p1.as_ref().and_then(|node| &node.next);
        p2 = p2.as_ref().and_then(|node| &node.next);
    }
    
    dummy.next
}

/// Problem: Palindrome Linked List
/// Check if a linked list is a palindrome.
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return true;
    }
    
    // Find the middle
    let mut slow = &head;
    let mut fast = &head;
    
    while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
        slow = &slow_node.next;
        if let Some(fast_next) = &fast_node.next {
            fast = &fast_next.next;
        } else {
            break;
        }
    }
    
    // Reverse the second half
    let mut second_half = reverse_list(slow.clone());
    let mut first_half = &head;
    
    // Compare the two halves
    while let (Some(first), Some(second)) = (first_half, &second_half) {
        if first.val != second.val {
            return false;
        }
        first_half = &first.next;
        second_half = second_half.as_ref().unwrap().next.clone();
    }
    
    true
}

/// Problem: Intersection of Two Linked Lists
/// Find the intersection point of two linked lists.
pub fn get_intersection_node(
    head_a: Option<Box<ListNode>>,
    head_b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if head_a.is_none() || head_b.is_none() {
        return None;
    }
    
    let mut p1 = &head_a;
    let mut p2 = &head_b;
    
    while !std::ptr::eq(p1.as_ref().unwrap(), p2.as_ref().unwrap()) {
        p1 = if p1.is_some() { &p1.as_ref().unwrap().next } else { &head_b };
        p2 = if p2.is_some() { &p2.as_ref().unwrap().next } else { &head_a };
    }
    
    p1.clone()
}

/// Run all linked list problem examples
pub fn run_examples() {
    println!("Running Linked List Problem Examples...");
    
    // Reverse Linked List
    let list = create_list(&[1, 2, 3, 4, 5]);
    let reversed = measure_time("Reverse List", || reverse_list(list.clone()));
    println!("Reverse List: {:?} -> {:?}", 
             list_to_vec(&list), 
             list_to_vec(&reversed));
    assert_result(list_to_vec(&reversed), vec![5, 4, 3, 2, 1], "Reverse List");
    
    // Merge Two Sorted Lists
    let l1 = create_list(&[1, 3, 5]);
    let l2 = create_list(&[2, 4, 6]);
    let merged = measure_time("Merge Lists", || merge_two_lists(l1.clone(), l2.clone()));
    println!("Merge Lists: {:?} + {:?} -> {:?}", 
             list_to_vec(&l1), 
             list_to_vec(&l2), 
             list_to_vec(&merged));
    assert_result(list_to_vec(&merged), vec![1, 2, 3, 4, 5, 6], "Merge Lists");
    
    // Remove Nth Node From End
    let list = create_list(&[1, 2, 3, 4, 5]);
    let result = measure_time("Remove Nth From End", || remove_nth_from_end(list.clone(), 2));
    println!("Remove Nth From End: {:?}, n=2 -> {:?}", 
             list_to_vec(&list), 
             list_to_vec(&result));
    assert_result(list_to_vec(&result), vec![1, 2, 3, 5], "Remove Nth From End");
    
    // Add Two Numbers
    let l1 = create_list(&[2, 4, 3]);
    let l2 = create_list(&[5, 6, 4]);
    let sum = measure_time("Add Two Numbers", || add_two_numbers(l1.clone(), l2.clone()));
    println!("Add Two Numbers: {:?} + {:?} -> {:?}", 
             list_to_vec(&l1), 
             list_to_vec(&l2), 
             list_to_vec(&sum));
    assert_result(list_to_vec(&sum), vec![7, 0, 8], "Add Two Numbers");
    
    // Palindrome Linked List
    let palindrome = create_list(&[1, 2, 2, 1]);
    let not_palindrome = create_list(&[1, 2, 3]);
    let is_pal = measure_time("Is Palindrome", || is_palindrome(palindrome.clone()));
    let is_not_pal = measure_time("Is Not Palindrome", || is_palindrome(not_palindrome.clone()));
    println!("Is Palindrome: {:?} -> {}", list_to_vec(&palindrome), is_pal);
    println!("Is Not Palindrome: {:?} -> {}", list_to_vec(&not_palindrome), is_not_pal);
    assert_result(is_pal, true, "Palindrome Check");
    assert_result(is_not_pal, false, "Not Palindrome Check");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reverse_list() {
        let list = create_list(&[1, 2, 3, 4, 5]);
        let reversed = reverse_list(list);
        assert_eq!(list_to_vec(&reversed), vec![5, 4, 3, 2, 1]);
    }
    
    #[test]
    fn test_merge_two_lists() {
        let l1 = create_list(&[1, 3, 5]);
        let l2 = create_list(&[2, 4, 6]);
        let merged = merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(&merged), vec![1, 2, 3, 4, 5, 6]);
    }
    
    #[test]
    fn test_remove_nth_from_end() {
        let list = create_list(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(list, 2);
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 5]);
    }
    
    #[test]
    fn test_add_two_numbers() {
        let l1 = create_list(&[2, 4, 3]);
        let l2 = create_list(&[5, 6, 4]);
        let sum = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(&sum), vec![7, 0, 8]);
    }
    
    #[test]
    fn test_is_palindrome() {
        let palindrome = create_list(&[1, 2, 2, 1]);
        let not_palindrome = create_list(&[1, 2, 3]);
        assert_eq!(is_palindrome(palindrome), true);
        assert_eq!(is_palindrome(not_palindrome), false);
    }
} 