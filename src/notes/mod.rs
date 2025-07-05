//! Notes and Learning Progress Module
//! 
//! This module is for your personal notes, learning progress, and custom solutions.
//! Use this space to:
//! - Document your learning journey
//! - Write your own solutions
//! - Track your progress
//! - Add problem-solving strategies

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Learning Progress Tracker
pub struct LearningProgress {
    topics: HashMap<String, TopicProgress>,
    start_date: u64,
}

#[derive(Debug, Clone)]
pub struct TopicProgress {
    pub name: String,
    pub problems_solved: usize,
    pub total_problems: usize,
    pub last_practiced: u64,
    pub notes: Vec<String>,
    pub difficulty_level: DifficultyLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl LearningProgress {
    pub fn new() -> Self {
        LearningProgress {
            topics: HashMap::new(),
            start_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    pub fn add_topic(&mut self, name: &str, total_problems: usize, difficulty: DifficultyLevel) {
        self.topics.insert(name.to_string(), TopicProgress {
            name: name.to_string(),
            problems_solved: 0,
            total_problems,
            last_practiced: 0,
            notes: Vec::new(),
            difficulty_level: difficulty,
        });
    }
    
    pub fn mark_problem_solved(&mut self, topic: &str) {
        if let Some(progress) = self.topics.get_mut(topic) {
            progress.problems_solved += 1;
            progress.last_practiced = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
    
    pub fn add_note(&mut self, topic: &str, note: &str) {
        if let Some(progress) = self.topics.get_mut(topic) {
            progress.notes.push(note.to_string());
        }
    }
    
    pub fn get_progress(&self) -> &HashMap<String, TopicProgress> {
        &self.topics
    }
    
    pub fn get_overall_progress(&self) -> f64 {
        if self.topics.is_empty() {
            return 0.0;
        }
        
        let total_solved: usize = self.topics.values().map(|t| t.problems_solved).sum();
        let total_problems: usize = self.topics.values().map(|t| t.total_problems).sum();
        
        (total_solved as f64 / total_problems as f64) * 100.0
    }
    
    pub fn print_progress(&self) {
        println!("📊 Learning Progress Report");
        println!("==========================");
        println!("Overall Progress: {:.1}%", self.get_overall_progress());
        println!();
        
        for (topic, progress) in &self.topics {
            let percentage = (progress.problems_solved as f64 / progress.total_problems as f64) * 100.0;
            println!("📚 {}: {}/{} ({:.1}%) - {:?}", 
                     topic, 
                     progress.problems_solved, 
                     progress.total_problems, 
                     percentage,
                     progress.difficulty_level);
            
            if !progress.notes.is_empty() {
                println!("   📝 Notes: {}", progress.notes.last().unwrap());
            }
        }
    }
}

/// Personal Solution Template
pub struct PersonalSolution {
    pub problem_name: String,
    pub topic: String,
    pub solution: String,
    pub time_complexity: String,
    pub space_complexity: String,
    pub notes: String,
    pub date_solved: u64,
}

impl PersonalSolution {
    pub fn new(problem_name: &str, topic: &str) -> Self {
        PersonalSolution {
            problem_name: problem_name.to_string(),
            topic: topic.to_string(),
            solution: String::new(),
            time_complexity: String::new(),
            space_complexity: String::new(),
            notes: String::new(),
            date_solved: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    pub fn set_solution(&mut self, solution: &str) {
        self.solution = solution.to_string();
    }
    
    pub fn set_complexity(&mut self, time: &str, space: &str) {
        self.time_complexity = time.to_string();
        self.space_complexity = space.to_string();
    }
    
    pub fn add_notes(&mut self, notes: &str) {
        self.notes = notes.to_string();
    }
    
    pub fn print(&self) {
        println!("🔍 Problem: {}", self.problem_name);
        println!("📂 Topic: {}", self.topic);
        println!("⏱️  Time Complexity: {}", self.time_complexity);
        println!("💾 Space Complexity: {}", self.space_complexity);
        println!("📝 Notes: {}", self.notes);
        println!("💻 Solution:");
        println!("{}", self.solution);
    }
}

/// Study Plan Generator
pub struct StudyPlan {
    pub daily_goals: Vec<String>,
    pub weekly_targets: Vec<String>,
    pub recommended_order: Vec<String>,
}

impl StudyPlan {
    pub fn new() -> Self {
        StudyPlan {
            daily_goals: Vec::new(),
            weekly_targets: Vec::new(),
            recommended_order: vec![
                "arrays".to_string(),
                "linked_lists".to_string(),
                "stacks".to_string(),
                "queues".to_string(),
                "trees".to_string(),
                "graphs".to_string(),
                "sorting".to_string(),
                "searching".to_string(),
                "dynamic_programming".to_string(),
                "strings".to_string(),
                "math".to_string(),
            ],
        }
    }
    
    pub fn add_daily_goal(&mut self, goal: &str) {
        self.daily_goals.push(goal.to_string());
    }
    
    pub fn add_weekly_target(&mut self, target: &str) {
        self.weekly_targets.push(target.to_string());
    }
    
    pub fn get_next_topic(&self, current_progress: &LearningProgress) -> Option<String> {
        for topic in &self.recommended_order {
            if !current_progress.topics.contains_key(topic) {
                return Some(topic.clone());
            }
        }
        None
    }
    
    pub fn print_plan(&self) {
        println!("📋 Study Plan");
        println!("============");
        println!("🎯 Daily Goals:");
        for goal in &self.daily_goals {
            println!("   • {}", goal);
        }
        println!();
        println!("📅 Weekly Targets:");
        for target in &self.weekly_targets {
            println!("   • {}", target);
        }
        println!();
        println!("📚 Recommended Order:");
        for (i, topic) in self.recommended_order.iter().enumerate() {
            println!("   {}. {}", i + 1, topic);
        }
    }
}

/// Problem Solving Strategies
pub struct ProblemSolvingStrategies {
    strategies: HashMap<String, Vec<String>>,
}

impl ProblemSolvingStrategies {
    pub fn new() -> Self {
        let mut strategies = HashMap::new();
        
        // Array strategies
        strategies.insert("arrays".to_string(), vec![
            "Two Pointers".to_string(),
            "Sliding Window".to_string(),
            "Prefix Sum".to_string(),
            "Kadane's Algorithm".to_string(),
            "Binary Search".to_string(),
        ]);
        
        // Linked List strategies
        strategies.insert("linked_lists".to_string(), vec![
            "Two Pointers (Fast/Slow)".to_string(),
            "Reverse in Place".to_string(),
            "Merge Sort".to_string(),
            "Cycle Detection".to_string(),
        ]);
        
        // Stack strategies
        strategies.insert("stacks".to_string(), vec![
            "Monotonic Stack".to_string(),
            "Parentheses Matching".to_string(),
            "Expression Evaluation".to_string(),
            "Histogram Problems".to_string(),
        ]);
        
        // Queue strategies
        strategies.insert("queues".to_string(), vec![
            "BFS".to_string(),
            "Level Order Traversal".to_string(),
            "Sliding Window".to_string(),
            "Priority Queue".to_string(),
        ]);
        
        ProblemSolvingStrategies { strategies }
    }
    
    pub fn get_strategies(&self, topic: &str) -> Option<&Vec<String>> {
        self.strategies.get(topic)
    }
    
    pub fn add_strategy(&mut self, topic: &str, strategy: &str) {
        self.strategies
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(strategy.to_string());
    }
    
    pub fn print_strategies(&self, topic: &str) {
        if let Some(strategies) = self.get_strategies(topic) {
            println!("🎯 Problem Solving Strategies for {}:", topic);
            for (i, strategy) in strategies.iter().enumerate() {
                println!("   {}. {}", i + 1, strategy);
            }
        } else {
            println!("No strategies found for topic: {}", topic);
        }
    }
}

/// Initialize default learning progress
pub fn initialize_default_progress() -> LearningProgress {
    let mut progress = LearningProgress::new();
    
    progress.add_topic("arrays", 10, DifficultyLevel::Beginner);
    progress.add_topic("linked_lists", 8, DifficultyLevel::Beginner);
    progress.add_topic("stacks", 6, DifficultyLevel::Intermediate);
    progress.add_topic("queues", 6, DifficultyLevel::Intermediate);
    progress.add_topic("trees", 12, DifficultyLevel::Intermediate);
    progress.add_topic("graphs", 15, DifficultyLevel::Advanced);
    progress.add_topic("sorting", 8, DifficultyLevel::Intermediate);
    progress.add_topic("searching", 5, DifficultyLevel::Beginner);
    progress.add_topic("dynamic_programming", 20, DifficultyLevel::Advanced);
    progress.add_topic("strings", 10, DifficultyLevel::Intermediate);
    progress.add_topic("math", 8, DifficultyLevel::Advanced);
    
    progress
}

/// Example usage and demonstration
pub fn run_notes_examples() {
    println!("📝 Notes and Learning Progress Examples");
    println!("======================================");
    
    // Initialize progress
    let mut progress = initialize_default_progress();
    
    // Mark some problems as solved
    progress.mark_problem_solved("arrays");
    progress.mark_problem_solved("arrays");
    progress.mark_problem_solved("linked_lists");
    
    // Add some notes
    progress.add_note("arrays", "Two Sum is a great starting point for hash map problems");
    progress.add_note("linked_lists", "Remember to handle edge cases with null pointers");
    
    // Print progress
    progress.print_progress();
    println!();
    
    // Create a personal solution
    let mut solution = PersonalSolution::new("Two Sum", "arrays");
    solution.set_solution(r#"
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
"#);
    solution.set_complexity("O(n)", "O(n)");
    solution.add_notes("Use hash map to store complements. Time complexity is O(n) for single pass.");
    
    solution.print();
    println!();
    
    // Study plan
    let mut plan = StudyPlan::new();
    plan.add_daily_goal("Solve 2 array problems");
    plan.add_daily_goal("Practice linked list manipulation");
    plan.add_weekly_target("Complete arrays module");
    plan.add_weekly_target("Start linked lists module");
    
    plan.print_plan();
    println!();
    
    // Problem solving strategies
    let strategies = ProblemSolvingStrategies::new();
    strategies.print_strategies("arrays");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_learning_progress() {
        let mut progress = LearningProgress::new();
        progress.add_topic("arrays", 10, DifficultyLevel::Beginner);
        progress.mark_problem_solved("arrays");
        progress.mark_problem_solved("arrays");
        
        assert_eq!(progress.get_overall_progress(), 20.0);
    }
    
    #[test]
    fn test_personal_solution() {
        let mut solution = PersonalSolution::new("Test Problem", "arrays");
        solution.set_solution("fn test() {}");
        solution.set_complexity("O(1)", "O(1)");
        
        assert_eq!(solution.problem_name, "Test Problem");
        assert_eq!(solution.topic, "arrays");
        assert_eq!(solution.solution, "fn test() {}");
    }
    
    #[test]
    fn test_study_plan() {
        let plan = StudyPlan::new();
        assert_eq!(plan.recommended_order[0], "arrays");
        assert_eq!(plan.recommended_order[1], "linked_lists");
    }
    
    #[test]
    fn test_problem_solving_strategies() {
        let strategies = ProblemSolvingStrategies::new();
        let array_strategies = strategies.get_strategies("arrays").unwrap();
        assert!(array_strategies.contains(&"Two Pointers".to_string()));
    }
} 