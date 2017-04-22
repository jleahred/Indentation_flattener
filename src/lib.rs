mod process_line;
mod parsing_lines;

use parsing_lines::ParsingLines;
use process_line::process_line;


#[cfg(test)]
mod tests;


// ------------------------------------------------------------------
// ------------------------------------------------------------------
//  API

//  CONSTANTS
const EOL: char = '\n';
const PUSH_INDENT: char = 0x02 as char;
const POP_INDENT: char = 0x03 as char;


//  NEW TYPES
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LineNum(u32);

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct SLine(String);

#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub struct SFlattedText(String);



//  ERROR TYPE
#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: LineNum,
    pub desc: String,
}


//  FUNCTION
pub fn flatter(input: &str) -> Result<SFlattedText, Error> {
    let mut parsing_lines = ParsingLines::new();

    for l in input.lines() {
        parsing_lines.add_opt_line(&process_line(&SLine::from(l)))?;
    }

    parsing_lines.close();
    Ok(parsing_lines.flat_text)
}


//  API
// ------------------------------------------------------------------
// ------------------------------------------------------------------



#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct NSpaces(usize);


impl SLine {
    pub fn new() -> Self {
        SLine(String::new())
    }
    pub fn from(s: &str) -> Self {
        SLine(String::from(s))
    }
}

impl SFlattedText {
    pub fn new() -> Self {
        SFlattedText(String::new())
    }
    pub fn from(s: &str) -> Self {
        SFlattedText(String::from(s))
    }
}
