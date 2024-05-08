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
pub use std::borrow::Borrow;
pub use std::collections::HashMap;
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub mod messenger;
pub use messenger::*;




pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(i: usize) -> Self {
        Self {
            track_value: Rc::new(i),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string().replace("Warning: ", "")).borrow();
        self.all_messages.borrow_mut().push(msg.to_string()).borrow();
    }
    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string().replace("Info: ", "")).borrow();
        self.all_messages.borrow_mut().push(msg.to_string()).borrow();
    }
    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string().replace("Error: ", "")).borrow();
        self.all_messages.borrow_mut().push(msg.to_string()).borrow();
    }
}
