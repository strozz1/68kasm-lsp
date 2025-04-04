use super::lsp::{Position, Range};
pub mod language;
pub fn tokenize<'a>(data: &str) -> Vec<TokenLine<'a>> {
    let mut lines: Vec<TokenLine> = Vec::new();
    for (idx, l) in data.lines().enumerate() {
        let line = tokenize_line(l.to_string(), idx as u32);
        lines.push(line);
    }
    lines
}

#[derive(Debug,Clone)]
pub enum TokenType {
    Label,
    Instruction,
    Operands,
    Comment,
}
#[derive(Debug,Clone)]
pub struct Token<'a> {
    token_type: TokenType,
    raw: &'a str,
    instr_specs: Option<language::InstructionSpecs>,
}
#[derive(Debug,Clone)]
pub struct TokenLine<'a> {
    pub label: Option<Range>,
    pub operation: Option<Range>,
    pub operand: Option<Range>,
    pub comment: Option<Range>,
    pub original: String,
    pub instruction: Option<Token<'a>>,
}
impl<'a> TokenLine<'a> {
    fn empty() -> TokenLine<'a> {
        TokenLine {
            label: None,
            operand: None,
            operation: None,
            original: "".to_string(),
            comment: None,
            instruction: None,
        }
    }
    pub fn is_empty(self: &TokenLine<'a>) -> bool {
        self.original.trim() == ""
    }
    pub fn get_op(self: &TokenLine<'a>) -> Option<&str> {
        let op = self.operation.clone()?;
        Some(&self.original[op.start.character as usize..op.end.character as usize])
    }
}

pub fn tokenize_line<'a>(line: String, lnum: u32) -> TokenLine<'a> {
    if line.is_empty() {
        return TokenLine::empty();
    }
    let cmt = search_comment(&line, lnum, 0);
    let trim_right = Range::first(&cmt);
    let mut len = 0;

    let label: Option<Range> = if len < trim_right {
        let label = search_label(&line, lnum, len);
        len = Range::last(&label);
        label
    } else {
        None
    };
    let op = if len < trim_right {
        //still chars
        let op = search_op(&line[len..trim_right], lnum, len);
        len = Range::last(&op);
        op
    } else {
        None
    };
    let opds = if len < trim_right {
        //still chars
        search_op(&line[len..trim_right], lnum, len)
    } else {
        None
    };

    TokenLine {
        original: line,
        label,
        operation: op,
        operand: opds,
        comment: cmt,
        instruction: None
    }

}

/**given a String seach for a Label. The label must be the first word of the line and dont have
 white spaces before.
**/
fn search_label(line: &str, lnum: u32, start: usize) -> Option<Range> {
    let pos1 = Position {
        line: lnum,
        character: 0,
    };
    if let Some(idx) = line.char_indices().find(|&(_, c)| c == ' ' || c == '\t') {
        if idx.0 == 0 {
            return None;
        }
        let pos2 = Position {
            line: lnum,
            character: (start + idx.0) as u32,
        };
        return Some(Range::new(pos1, pos2));
    };
    Some(Range::new(
        pos1,
        Position {
            line: lnum,
            character: line.len() as u32,
        },
    ))
}

/**
 Search for the operation code.
 The op MUST have spaces or tab before, even if no label present.
*/
fn search_op(line: &str, lnum: u32, start: usize) -> Option<Range> {
    if let Some(index) = line.char_indices().find(|&(_, c)| !c.is_ascii_whitespace()) {
        if index.0 == 0 {
            return None;
        }
        let mut index2 = line[index.0..]
            .char_indices()
            .find(|&(_, c)| c == ' ' || c == '\t' )
            .map(|(index, _)| index)
            .unwrap_or_else(|| line[index.0..].len());
        if index2 >= line.len() {
            index2 = line.len()
        }

        let pos1 = Position {
            line: lnum,
            character: (start + index.0) as u32,
        };
        let pos2 = Position {
            line: lnum,
            character: (start + index.0 + index2) as u32,
        };
        return Some(Range::new(pos1, pos2));
    }
    None
}
/*
 Search if comment line is present
*/
fn search_comment(line: &str, lnum: u32, start: usize) -> Option<Range> {
    if let Some(index) = line.char_indices().find(|&(_, c)| c == '*') {
        let pos1 = Position {
            line: lnum,
            character: (start + index.0) as u32,
        };
        let pos2 = Position {
            line: lnum,
            character: (start + line.len()) as u32,
        };
        return Some(Range::new(pos1, pos2));
    }
    Some(Range::new(
        Position {
            line: lnum,
            character: line.len() as u32,
        },
        Position {
            line: lnum,
            character: line.len() as u32,
        },
    ))
}
#[cfg(test)]
mod test_tokenization {
    use super::*;
    #[test]
    fn correct_tokenize() {
        let line = String::from("TEST: MOVE.L a,b *comment");
        let a = TokenLine {
            original: line.clone(),
            label: Some(Range::new(
                Position {
                    line: 0,
                    character: 0,
                },
                Position {
                    line: 0,
                    character: 5,
                },
            )),
            operation: Some(Range::new(
                Position {
                    line: 0,
                    character: 6,
                },
                Position {
                    line: 0,
                    character: 12,
                },
            )),
            operand: Some(Range::new(
                Position {
                    line: 0,
                    character: 13,
                },
                Position {
                    line: 0,
                    character: 16,
                },
            )),
            comment: Some(Range::new(
                Position {
                    line: 0,
                    character: 17,
                },
                Position {
                    line: 0,
                    character: line.len() as u32,
                },
            )),
            instruction:None
        };
        let b = tokenize_line(line, 0);

        assert_eq!(a.label, b.label);
        assert_eq!(a.operation, b.operation);
        assert_eq!(a.operand, b.operand);
        assert_eq!(a.comment, b.comment);
        assert_eq!(a.original, b.original);
    }
    #[test]
    fn empty_line() {
        let e = String::new();
        let a = TokenLine {
            label: None,
            operand: None,
            operation: None,
            original: String::new(),
            comment: None,
            instruction: None,
        };
        let b = tokenize_line(e, 0);

        assert_eq!(a.label, b.label);
        assert_eq!(a.operation, b.operation);
        assert_eq!(a.operand, b.operand);
        assert_eq!(a.comment, b.comment);
        assert_eq!(a.original, b.original);
    }

    //TODO: when no label, the op if in the left is treated as a label
}
