use std::cmp;

pub struct StringScanner {
    index: usize,
    string: String,
}

impl StringScanner {
    pub fn new(string: &String) -> Self {
        return Self {
            index: 0,
            string: string.to_string()
        }
    }

    pub fn consume_exact(&mut self, n: usize) -> String {
        let start = self.index;
        let end = cmp::min(self.index + n, self.string.len());

        self.index = end;

        return String::from(&self.string[start..end]);
    }

    pub fn consume_until(&mut self, predicate: impl Fn (char) -> bool) -> String {
        let mut end = self.index;

        while end < self.string.len() {
            if predicate(self.string.chars().nth(end).unwrap()) {
                break;
            }

            end += 1;
        }

        let start = self.index;

        self.index = end;

        return String::from(&self.string[start..end]);
    }

    
    pub fn consume_until_char(&mut self, target_char: char) -> String {
        return self.consume_until(|char| char == target_char);
    }

    pub fn consume_rest(&mut self) -> String {
        return self.consume_until(|_char| false);
    }

    pub fn remaining(&self) -> usize {
        return self.string.len() - self.index;
    }

    pub fn finished(&self) -> bool {
        return self.remaining() <= 0;
    }
}