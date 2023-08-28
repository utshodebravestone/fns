#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub starting_index: usize,
    pub ending_index: usize,
}

impl TextSpan {
    pub fn new(starting_index: usize, ending_index: usize) -> Self {
        Self {
            starting_index,
            ending_index,
        }
    }

    pub fn add(starting_span: Self, ending_span: Self) -> Self {
        Self {
            starting_index: starting_span.starting_index,
            ending_index: ending_span.ending_index,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub text_span: TextSpan,
}

impl Error {
    pub fn new(message: String, text_span: TextSpan) -> Self {
        Self { message, text_span }
    }

    pub(crate) fn report(&self, source_code: &str) {
        let mut line = 1;
        let mut column = 1;

        for (index, char) in source_code.chars().enumerate() {
            if index == self.text_span.starting_index {
                break;
            }

            if char == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        eprintln!("[error in line: {line}, column: {column}]");
        eprintln!("Error: {}", self.message);
    }
}
