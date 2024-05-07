// File: messenger.rs

use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;

// Define the Logger trait
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

// Define the Tracker structure
pub struct Tracker {
    value: usize,
    max: usize,
}

impl Tracker {
    // Associated function to create a new Tracker instance
    pub fn new(max: usize) -> Self {
        Tracker {
            value: 0,
            max,
        }
    }

    // Set the value and check if it exceeds the maximum limit
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }

    // Peek at the current usage percentage
    pub fn peek(&self) -> usize {
        (self.value * 100) / self.max
    }
}

// Implement Logger for standard output
pub struct StdoutLogger {
    messages: RefCell<VecDeque<String>>,
}

impl StdoutLogger {
    pub fn new() -> Self {
        StdoutLogger {
            messages: RefCell::new(VecDeque::new()),
        }
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.borrow_mut().iter().cloned().collect()
    }
}

impl Logger for StdoutLogger {
    fn warning(&self, msg: &str) {
        self.messages.borrow_mut().push_back(format!("Warning: {}", msg));
    }

    fn info(&self, msg: &str) {
        self.messages.borrow_mut().push_back(format!("Info: {}", msg));
    }

    fn error(&self, msg: &str) {
        self.messages.borrow_mut().push_back(format!("Error: {}", msg));
    }
}
