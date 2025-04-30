#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    queue: VecDeque<T>,
    mins: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        MinQueue {
            queue: VecDeque::new(),
            mins: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.queue.push_back(val.clone());
        while let Some(old) = self.mins.back() {
            if old > &val {
                self.mins.pop_back();
            } else {
                break;
            }
        }
        self.mins.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(first) = self.queue.pop_front() {
            if let Some(min_first) = self.mins.front() {
                if &first == min_first {
                    self.mins.pop_front();
                }
            }
            return Some(first);
        }
        None
    }

    pub fn front(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.mins.front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }
}
