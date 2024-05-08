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

use std::cell::{RefCell, Cell};
use std::rc::Rc;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        workers::default()
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let thread = Thread::new_thread(self.track_worker(), c, self);
        self.states.borrow_mut().push(false);
        (thread.pid, thread)
    }
    
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id];
    }

    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{:?} is already dropped", id)
        }

        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    // expected public fields
    pid: usize,
    cmd: String,
    parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Self{
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self)
    }
}
