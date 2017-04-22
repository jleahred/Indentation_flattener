use std::char;


use {Error, LineNum, NSpaces, SLine, SFlattedText, EOL, PUSH_INDENT, POP_INDENT};
use process_line::LineInfo;



#[derive(Debug, PartialEq, Copy, Clone)]
struct IndentLevel(usize);


impl LineNum {
    fn inc(&mut self) -> &Self {
        self.0 += 1;
        self
    }
}

impl SFlattedText {
    fn push(&mut self, ch: char) -> &mut Self {
        self.0.push(ch);
        self
    }
    fn add_line(&mut self, l: &SLine) -> &mut Self {
        self.0.push_str(&l.0);
        self.0.push(EOL);
        self
    }
    fn add_push_indent(&mut self) -> &mut Self {
        self.push(PUSH_INDENT);
        self
    }
    fn add_pop_indents(&mut self, levels: IndentLevel) -> &mut Self {
        for _ in 0..levels.0 {
            self.push(POP_INDENT);
        }
        self
    }
}



#[derive(Debug)]
pub struct ParsingLines {
    line_counter: LineNum,
    prev_indent_spaces: Vec<NSpaces>,
    pub flat_text: SFlattedText,
}

impl ParsingLines {
    pub fn new() -> ParsingLines {
        ParsingLines {
            line_counter: LineNum(0),
            prev_indent_spaces: Vec::new(),
            flat_text: SFlattedText::from(""),
        }
    }

    pub fn add_opt_line(&mut self, line: &Option<LineInfo>) -> Result<&ParsingLines, Error> {
        self.line_counter.inc();
        match *line {
            None => Ok(self.add_eol()),
            Some(ref l) => self.add_line_info(l),
        }
    }

    pub fn close(&mut self) -> &mut Self {
        for _ in 1..self.prev_indent_spaces.len() {
            self.flat_text.push(POP_INDENT);
        }
        self
    }



    fn add_line_info(&mut self, l: &LineInfo) -> Result<&ParsingLines, Error> {
        match self.prev_indent_spaces.last().cloned() {
            None => Ok(self.add_first_line(l)),

            Some(last_prev) => {
                use std::cmp::Ordering::{Equal, Greater, Less};
                match l.indent.cmp(&last_prev) {
                    Equal => Ok(self.add_line(&l.content)),
                    Greater => Ok(self.add_subtoken_line(l)),
                    Less => self.add_backtoken_line(&l),
                }
            }
        }
    }

    //  ------------------------------
    fn add_eol(&mut self) -> &Self {
        self.flat_text.push(EOL);
        self
    }

    fn add_first_line(&mut self, line: &LineInfo) -> &Self {
        self.prev_indent_spaces.push(line.indent);
        self.add_line(&line.content)
    }

    fn add_line(&mut self, content: &SLine) -> &Self {
        //  no index modif
        self.flat_text.add_line(content);
        self
    }

    fn add_subtoken_line(&mut self, line: &LineInfo) -> &Self {
        self.prev_indent_spaces.push(line.indent);
        self.flat_text
            .add_push_indent()
            .add_line(&line.content);
        self
    }

    fn add_backtoken_line(&mut self, line: &LineInfo) -> Result<&Self, Error> {
        let back_levels = self.update_prevs(line.indent)?;
        self.flat_text
            .add_pop_indents(back_levels)
            .add_line(&line.content);
        Ok(self)
    }

    fn update_prevs(&mut self, spaces: NSpaces) -> Result<IndentLevel, Error> {
        fn _update_prevs(s: &mut ParsingLines, spaces: NSpaces) -> Result<(), Error> {
            fn get_error(line_counter: LineNum) -> Error {
                Error {
                    line: line_counter,
                    desc: "invalid indentation".to_owned(),
                }
            };

            use std::cmp::Ordering::{Equal, Greater, Less};
            let prev_spaces = s.prev_indent_spaces
                .last()
                .cloned()
                .ok_or(get_error(s.line_counter))?;

            match prev_spaces.cmp(&spaces) {
                Equal => Ok(()),
                Greater => {
                    s.prev_indent_spaces.pop();
                    _update_prevs(s, spaces)
                }
                Less => Err(get_error(s.line_counter)),
            }
        }
        let prev_indent_level = IndentLevel(self.prev_indent_spaces.len() - 1);
        _update_prevs(self, spaces)?;
        let after_indent_level = IndentLevel(self.prev_indent_spaces.len() - 1);
        Ok(IndentLevel(prev_indent_level.0 - after_indent_level.0))
    }
}
