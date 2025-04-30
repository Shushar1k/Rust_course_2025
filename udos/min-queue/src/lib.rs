#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    input: VecDeque<(i32, i32)>,
    output: VecDeque<(i32, i32)>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        input = VecDeque::new();
        output = VecDeque::new();
    }

    pub fn push(&mut self, val: T) {
        if input.is_empty() {
            input.push_back((val, val));
        } else {
            input.push_back((val, std::cmp::min(val, input.back().1)));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if !output.is_empty() {
            output.pop_back();
        } else {
            loop {
                if input.is_empty() {
                    break;
                }
                let pare = input.back();
                input.pop_back();
                if output.is_empty() {
                    output.push_back((pare.1, pare.1);
                } else {
                    output.push_back((pare.1, std::cmp::min(pare.1, output.back().1)));
                }
            }
            output.pop_back();
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return output.back();
        }
        if output.is_empty() {
            return input.front().1;
        }
        return output.back().1;
    }

    pub fn min(&self) -> Option<&T> {
        let min1 = input.back().2;
        let min2 = input.back().2;
        std::cmp::min(min1, min2);
    }

    pub fn len(&self) -> usize {
        input.len() + output.len();
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0;
    }
}
