pub struct LineParser {
    index: usize,
    lines: Vec<String>
}

impl LineParser {
    pub fn new(lines: Vec<String>) -> Self {
        return Self {
            index: 0,
            lines
        };
    }

    pub fn consume(&mut self) -> Option<String> {
        let result = self.lines.get(self.index);

        self.index += 1;

        return result.cloned();
    }

    pub fn get(&self) -> Option<String> {
        return self.lines.get(self.index).cloned();
    }
}