use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use super::{focusing_power::FocusingPower, lava_instruction::{LavaAction, LavaInstruction},lava_hash::LavaHash};

#[derive(Debug, Clone)]
struct LensBoxNode<'a> {
    focal_length: u8,
    previous: Option<Weak<RefCell<LensBoxNode<'a>>>>,
    next: Option<Rc<RefCell<LensBoxNode<'a>>>>
}

impl<'a> LensBoxNode<'a> {
    fn new(focal_length: u8) -> Self {
        LensBoxNode {
            focal_length,
            previous: None,
            next: None
        }
    }
}

impl<'a> FocusingPower for LensBoxNode<'a> {
    fn get_focusing_power(&self) -> usize {
        self.focal_length as usize
    }
}

#[derive(Debug, Clone)]
struct LensBox<'a> {
    head: Option<Rc<RefCell<LensBoxNode<'a>>>>,
    tail: Option<Weak<RefCell<LensBoxNode<'a>>>>,
    index_map: HashMap<&'a [u8], Rc<RefCell<LensBoxNode<'a>>>>
}

impl<'a> LensBox<'a> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            index_map: HashMap::new()
        }
    }

    fn nodes(&self) -> LensBoxNodeIterator<'a> {
        LensBoxNodeIterator(self.head.clone())
    }

    fn push(&mut self, label: &'a [u8], focal_length: u8) {
        if let Some(node) = self.index_map.get(label) {
            node.borrow_mut().focal_length = focal_length;
        } else {
            self.push_only(label, focal_length);
        }
    }

    fn remove(&mut self, label: &'a [u8]) {
        let node = self.index_map.remove(label);
        if let Some(node) = node {
            if let Some(previous_node) = &node.borrow().previous {
                if let Some(previous_node) = previous_node.upgrade() {
                    // Removed node was not first
                    if let Some(next_node) = &node.borrow().next {
                        // Removed node was not last
                        previous_node.borrow_mut().next = Some(Rc::clone(next_node));
                        next_node.borrow_mut().previous = Some(Rc::downgrade(&previous_node));
                    } else {
                        // Removed node was last
                        previous_node.borrow_mut().next = None;
                        self.tail = Some(Rc::downgrade(&previous_node));
                    }
                    return;
                }
            }

            // Removed node was first
            if let Some(next_node) = &node.borrow().next {
                // Removed node was not last
                next_node.borrow_mut().previous = None;
                self.head = Some(Rc::clone(next_node))
            } else {
                // Removed node was last
                self.head = None;
                self.tail = None;
            }
        }
    }

    fn push_only(&mut self, label: &'a [u8], focal_length: u8) {
        let node = Rc::new(RefCell::new(LensBoxNode::new(focal_length)));
        self.index_map.insert(label, Rc::clone(&node));

        if let Some(tail) = &self.tail {
            if let Some(tail) = tail.upgrade() {
                // Not empty
                tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().previous = Some(Rc::downgrade(&tail));
                self.tail = Some(Rc::downgrade(&node));
                return;
            }
        }
        // Empty
        self.tail = Some(Rc::downgrade(&node));
        self.head = Some(node);
    }
}

impl<'a> FocusingPower for LensBox<'a> {
    fn get_focusing_power(&self) -> usize {
        self.nodes()
            .enumerate()
            .map(|(i, node)| (i + 1) * node.borrow().get_focusing_power())
            .sum()
    }
}

struct LensBoxNodeIterator<'a>(Option<Rc<RefCell<LensBoxNode<'a>>>>);

impl<'a> Iterator for LensBoxNodeIterator<'a> {
    type Item = Rc<RefCell<LensBoxNode<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.clone();
        if let Some(next) = &current {
            self.0 = next.borrow().next.clone();
        }

        current
    }
}

#[derive(Debug, Clone)]
pub struct LensArray<'a>([LensBox<'a>; 256]);

impl<'a> LensArray<'a> {
    fn new() -> Self {
        LensArray([0usize; 256].map(|_| LensBox::new()))
    }

    fn process(&mut self, lava_instruction: LavaInstruction<'a>) {
        let label = lava_instruction.label;
        let lava_hash = label.get_lava_value();

        match lava_instruction.action {
            LavaAction::Set(focal_length) => self.0[lava_hash].push(label, focal_length),
            LavaAction::Decrement => self.0[lava_hash].remove(label),
        }
    }
}

impl<'a> FromIterator<LavaInstruction<'a>> for LensArray<'a> {
    fn from_iter<T: IntoIterator<Item=LavaInstruction<'a>>>(iter: T) -> Self {
        let mut lens_array = LensArray::new();
        for instruction in iter {
            lens_array.process(instruction);
        }

        lens_array
    }
}

impl<'a> FocusingPower for LensArray<'a> {
    fn get_focusing_power(&self) -> usize {
        self.0.iter()
            .enumerate()
            .map(|(i, lens_box)| (i + 1) * lens_box.get_focusing_power())
            .sum()
    }
}
