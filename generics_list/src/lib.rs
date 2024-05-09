#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take().map(|node| Box::new(node)),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed_node| *boxed_node);
        }
    }

    pub fn len(&self) -> usize {
        // let mut len = 0;
        // let mut node = &self.head;
        // while let Some(curr_node) = node {
        //     len += 1;
        //     node = &curr_node.next.as_ref().map(|boxed_node| &**boxed_node); //idk wtf is that piece of sh*t
        // }
        // len
        match &self.head {
            Some(v) => {
                let mut l = 0;
                let mut curr = &v.next;
                loop{
                    match  curr{
                        Some(node) => {
                            curr = &node.next;
                            l+=1;
                        },
                        None => { break; },
                    }
                }
                l+1
            },
            None => 0
        }
    }
}
