#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    input: VecDeque<(T, T)>,
    output: VecDeque<(T, T)>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            input : VecDeque::new(),
            output : VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        if self.input.is_empty() {
            self.input.push_back((val.clone(), val.clone()));
        } else {
            let new_min : T = std::cmp::min(val.clone(), self.input.back().unwrap().1.clone());
            self.input.push_back((val, new_min));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let ans = self.front().cloned();
        if !self.output.is_empty() {
            self.output.pop_back();
            return ans;
        } else {
            loop {
                if self.input.is_empty() {
                    break;
                }
                let pare = self.input.back().unwrap().clone();
                self.input.pop_back();
                if self.output.is_empty() {
                    self.output.push_back((pare.0.clone(), pare.0.clone()));
                } else {
                    self.output.push_back((pare.0.clone(), std::cmp::min(pare.0.clone(), self.output.back().unwrap().0.clone())));
                }
            }
            self.output.pop_back();
            return ans;
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.output.is_empty() {
            return self.input.front().map(|pair| &pair.0);
        }
        return self.output.back().map(|pair| &pair.0);
    }

    pub fn min(&self) -> Option<&T> {
        if self.input.is_empty() {
            return self.output.back().map(|pair| &pair.1);
        }
        if self.output.is_empty() {
            return self.input.back().map(|pair| &pair.1);
        }
        let min1 = self.input.back().map(|pair| &pair.1)?;
        let min2 = self.output.back().map(|pair| &pair.1)?;
        return Some(std::cmp::min(min1, min2));
    }

    pub fn len(&self) -> usize {
        return self.input.len() + self.output.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }
}
