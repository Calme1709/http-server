use std::cell::Cell;

trait Len {
    fn len(&self) -> usize;
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        return self.len();
    }
}

pub struct VecScanner<T> {
    index: Cell<usize>,
    input: Vec<T>,
}

impl<T> VecScanner<T> where T: Eq + ?Sized + Copy,
{
    pub fn new(input: Vec<T>) -> Self {
        return Self {
            index: Cell::new(0),
            input
        }
    }

    pub fn consume_exact(&self, n: usize) -> Vec<T> {
        let start = self.index.get();
        let end = core::cmp::min(self.index.get() + n, self.input.len());

        self.index.set(end);

        return self.input[start..end].to_vec();
    }

    pub fn consume_until(&self, predicate: impl Fn(T, usize) -> bool) -> Vec<T> {
        let mut end = self.index.get();

        while end < self.input.len() {
            if predicate(self.input[end], end) {
                break;
            }

            end += 1;
        }

        let start = self.index.get();

        self.index.set(end);

        return self.input[start..end].to_vec();
    }

    pub fn consume_until_value(&mut self, target_value: T) -> Vec<T> {
        return self.consume_until(|value, _index| value == target_value);
    }

    pub fn consume_until_pattern(&mut self, pattern: Vec<T>) -> Vec<T> {
        return self.consume_until(|_value, _index| self.peek(pattern.len()) == pattern);
    }

    pub fn consume_rest(&mut self) -> Vec<T> {
        return self.consume_until(|_value, _index| false);
    }

    pub fn peek(&self, n: usize) -> Vec<T> {
        let start = self.index.get();
        let end = core::cmp::min(self.index.get() + n, self.input.len());

        return self.input[start..end].to_vec();
    }

    pub fn finished(&self) -> bool {
        return self.index.get() >= self.input.len();
    }
}