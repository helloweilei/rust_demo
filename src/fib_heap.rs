use std::{ cell::RefCell, rc::Rc };

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    key: T,
    // Maybe Rc<RefCell<Option<Node<T>>>> more better
    left: Option<NodeRef<T>>,
    right: Option<NodeRef<T>>,
    p: Option<NodeRef<T>>,
}

#[derive(Debug)]
pub struct FibHeap<T> {
    // 节点数量
    n: usize,
    // 最小节点
    min: Option<NodeRef<T>>,
}

impl<T: PartialOrd + Copy> FibHeap<T> {
    pub fn new() -> FibHeap<T> {
        FibHeap {
            n: 0,
            min: None,
        }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = Node {
            key,
            left: None,
            right: None,
            p: None,
        };
        let node_ref = Rc::new(RefCell::new(new_node));
        if self.n == 0 {
            node_ref.borrow_mut().left = Some(node_ref.clone());
            node_ref.borrow_mut().right = Some(node_ref.clone());
            self.min = Some(node_ref.clone());
        } else {
            let prev_node = self.min.clone().unwrap().borrow_mut().left.clone().unwrap();
            let next_node = self.min.clone().unwrap();

            prev_node.borrow_mut().right = Some(node_ref.clone());
            node_ref.borrow_mut().left = Some(prev_node);
            next_node.borrow_mut().left = Some(node_ref.clone());
            node_ref.borrow_mut().right = Some(next_node);

            if key < self.min.clone().unwrap().borrow().key {
                self.min = Some(node_ref.clone());
            }
            self.min = Some(node_ref.clone());
        }
        self.n = self.n + 1;
    }
    pub fn get_min(&self) -> Option<T> {
        return if let Some(node) = &self.min { Some(node.borrow().key) } else { None };
    }
    pub fn union(&mut self, other: FibHeap<T>) {
        let prev_node = self.min.clone().unwrap().borrow_mut().left.clone().unwrap();
        let next_node = self.min.clone().unwrap();

        let other_prev_node = other.min.clone().unwrap().borrow_mut().left.clone().unwrap();
        let other_next_node = other.min.clone().unwrap();

        prev_node.borrow_mut().right = Some(other_prev_node.clone());
        other_prev_node.borrow_mut().left = Some(prev_node);
        other_next_node.borrow_mut().right = Some(next_node.clone());
        next_node.borrow_mut().left = Some(other_next_node);

        if other.get_min() < self.get_min() {
            self.min = other.min;
        }
    }
}
