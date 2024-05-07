// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod messenger;

use std::collections::HashMap;
use std::cell::RefCell;

// Define the Worker structure
pub struct Worker<'a> {
    track_value: &'a mut messenger::Tracker,
    mapped_messages: HashMap<&'static str, String>,
    all_messages: RefCell<Vec<String>>,
}

impl<'a> Worker<'a> {
    // Associated function to create a new Worker instance
    pub fn new(track_value: &'a mut messenger::Tracker) -> Self {
        Worker {
            track_value,
            mapped_messages: HashMap::new(),
            all_messages: RefCell::new(Vec::new()),
        }
    }

    // Logger implementation for Worker
    pub fn logger(&mut self) -> &mut dyn messenger::Logger {
        let worker = self;
        struct WorkerLogger<'a> {
            worker: &'a mut Worker<'a>,
        }
        impl<'a> messenger::Logger for WorkerLogger<'a> {
            fn warning(&self, msg: &str) {
                self.worker.mapped_messages.insert("warning", msg.to_owned());
                self.worker.all_messages.borrow_mut().push(format!("Warning: {}", msg));
            }
            fn info(&self, msg: &str) {
                self.worker.mapped_messages.insert("info", msg.to_owned());
                self.worker.all_messages.borrow_mut().push(format!("Info: {}", msg));
            }
            fn error(&self, msg: &str) {
                self.worker.mapped_messages.insert("error", msg.to_owned());
                self.worker.all_messages.borrow_mut().push(format!("Error: {}", msg));
            }
        }
        WorkerLogger { worker }
    }

    // Getter for mapped messages
    pub fn get_mapped_messages(&self) -> &HashMap<&'static str, String> {
        &self.mapped_messages
    }

    // Getter for all messages
    pub fn get_all_messages(&self) -> Vec<String> {
        self.all_messages.borrow().clone()
    }
}
