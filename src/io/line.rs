/// Holds a line of Jack code and the corresponding line number.
#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub number: usize,
}

impl Line {
    pub fn new(content: &str, number: usize) -> Line {
        Line {
            content: String::from(content),
            number,
        }
    }

    pub fn replace_content(mut self, from: &str, to: &str) -> Self {
        self.content = self.content.replace(from, to);
        self
    }
}
