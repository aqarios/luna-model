use std::fmt::{Display, Formatter};

const MAX_LINE_LENGTH: usize = 80;
const INDENTATION: usize = 2;

pub struct LineLengthRestrictor {
    current_string: String,
    current_line_length: usize,
    indent: usize,
}

impl LineLengthRestrictor {
    pub fn new(indent: usize) -> Self {
        Self {
            current_string: String::new(),
            current_line_length: 0,
            indent,
        }
    }

    pub fn write(&mut self, s: &str) -> &mut Self {
        if self.current_line_length + s.len() > MAX_LINE_LENGTH {
            self.new_line();
        }
        self.current_string += s;
        self.current_line_length += s.len();
        self
    }

    pub fn new_line(&mut self) -> &mut Self {
        self.current_string += &format!("\n{}", " ".repeat(self.indent));
        self.current_line_length = self.indent;
        self
    }

    pub fn space(&mut self) -> &mut Self {
        // todo: this function is buggy, as it adds trailing spaces to finished lines...
        // change to LP translator probably best in the long term.
        if self.current_line_length < MAX_LINE_LENGTH {
            self.current_string += " ";
            self.current_line_length += 1;
            self
        } else {
            self.new_line()
        }
    }

    pub fn with_new_line(&mut self, s: &str) -> &mut Self {
        self.new_line().write(s)
    }

    pub fn with_spaces(&mut self, s: &str) -> &mut Self {
        self.space().write(s).space()
    }

    pub fn increase_indent(&mut self) -> &mut Self {
        self.indent += INDENTATION;
        self
    }

    pub fn decrease_indent(&mut self) -> &mut Self {
        self.indent -= INDENTATION;
        self
    }
}

impl Display for LineLengthRestrictor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.current_string)
    }
}
