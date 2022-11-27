#![allow(unused)]

use std::collections::LinkedList;
use std::fmt::Debug;

type S = usize;
type I = usize;
type H = usize;

const HEAD: I = 0;
const TAIL: I = 1;

#[derive(Debug)]
struct RelativeSkipList<T: Debug> {
    nodes: Vec<Node<T>>,
    height: H,
}

#[derive(Debug)]
enum Node<T: Debug> {
    Head(Head),
    Tail(Tail),
    ValueNode(ValueNode<T>),
}

#[derive(Debug)]
struct Tail {
    previous: I
}

impl Tail {
    pub fn new(previous: I) -> Self {
        Self { previous }
    }
}

#[derive(Debug)]
struct Head {
    next: Vec<(S, I)>
}

impl Head {
    pub fn new() -> Self {
        Self { next: Vec::new() }
    }
}

#[derive(Debug)]
struct ValueNode<T: Debug> {
    value: T,
    previous: I,
    next: Vec<(S, I)>
}

impl<T: Debug> ValueNode<T> {
    pub fn new(value: T, previous: I) -> Self {
        Self { value, previous, next: Vec::new() }
    }
}

impl<T: Debug> RelativeSkipList<T> {
    pub fn new(length: S) -> Self {
        let mut head = Head::new();
        let tail = Tail::new(HEAD);
        head.next.push((length, TAIL));
        let nodes = vec![Node::Head(head), Node::Tail(tail)];
        Self { nodes, height: 1 }
    }

    fn head(&self) -> &Head {
        if let Node::Head(head) = &self.nodes[HEAD] {
            head
        } else { unreachable!() }
    }

    fn head_mut(&mut self) -> &mut Head {
        if let Node::Head(head) = &mut self.nodes[HEAD] {
            head
        } else { unreachable!() }
    }

    fn tail(&self) -> &Tail {
        if let Node::Tail(tail) = &self.nodes[TAIL] {
            tail
        } else { unreachable!() }
    }

    fn tail_mut(&mut self) -> &mut Tail {
        if let Node::Tail(tail) = &mut self.nodes[TAIL] {
            tail
        } else { unreachable!() }
    }

    pub fn insert_after(&mut self, index: I, spacing: S, value: T) -> I {
        let mut height = generate_random_height();
        if height > self.height {
            height = self.height + 1;
            self.increase_height();
        }
        let index = self.nodes.len();
        index
    }

    fn increase_height(&mut self) {
        self.
    }
}

fn generate_random_height() -> H {
    let mut height = 1;
    while rand::random::<bool>() {
        height += 1;
    }
    height
}

#[cfg(test)]
mod tests {
    use crate::RelativeSkipList;

    #[test]
    fn it_works() {
        let mut list = RelativeSkipList::<char>::new();
        list.push(2, 'a');
        list.push(1, 'b');
        list.push(3, 'c');
        println!("{:#?}", list);
    }
}
