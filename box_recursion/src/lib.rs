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

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self {
            grade: None
        }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let add_w = Box::new(Worker {
            role,
            name,
            next: self.grade.take() 
            });
        self.grade = Some(add_w);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take()
            .map(|worker| {
                self.grade = worker.next;
                worker.name
            })


        // let last_worker = self.grade.clone();
        // match last_worker {
        //     None => None,
        //     Some(w) => {
        //         let last_worker_name = w.name;
        //         self.grade = w.next;
        //         Some(last_worker_name)
        //     }
        // }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade
            .as_ref()
            .map(|worker|{
                (worker.name.clone(), worker.role.clone())
            })
    }
}

// fn main() {
//     let mut list = WorkEnvironment::new();
//     list.add_worker(String::from("CEO"), String::from("Marie"));
//     list.add_worker(String::from("Manager"), String::from("Monica"));
//     list.add_worker(String::from("Normal Worker"), String::from("Ana"));
//     list.add_worker(String::from("Normal Worker"), String::from("Alice"));
//     println!("{:#?}", list);

//     println!("{:?}", list.last_worker());

//     list.remove_worker();
//     list.remove_worker();
//     list.remove_worker();
//     println!("{:?}", list);
//     list.remove_worker();
//     println!("{:?}", list);
// }

// $ cargo run
// WorkEnvironment {
//     grade: Some(
//         Worker {
//             role: "Normal Worker",
//             name: "Alice",
//             next: Some(
//                 Worker {
//                     role: "Normal Worker",
//                     name: "Ana",
//                     next: Some(
//                         Worker {
//                             role: "Manager",
//                             name: "Monica",
//                             next: Some(
//                                 Worker {
//                                     role: "CEO",
//                                     name: "Marie",
//                                     next: None,
//                                 },
//                             ),
//                         },
//                     ),
//                 },
//             ),
//         },
//     ),
// }
// Some{{"Alice", "Normal Worker"}}
// WorkEnvironment { grade: Some(Worker { role: "CEO", name: "Marie", next: None }) }
// WorkEnvironment { grade: None }
// $