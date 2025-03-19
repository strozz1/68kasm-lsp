use super::lsp::{Position, Range};
use log::info;

pub fn tokenize(data: &str) -> Vec<TkLine> {
    let mut lines: Vec<TkLine> = Vec::new();
    for (idx, l) in data.lines().enumerate() {
        let line = tokenize_line(l.to_string(), idx as u32);
        lines.push(line);
    }
    return lines;
}
#[derive(Debug)]
pub struct TkLine {
    pub label: Option<Range>,
    pub operation: Option<Range>,
    pub operand: Option<Range>,
    pub comment: Option<Range>,
    pub original: String,
}
impl TkLine {
    fn empty() -> TkLine {
        TkLine {
            label: None,
            operand: None,
            operation: None,
            original: "".to_string(),
            comment: None,
        }
    }
}

pub fn tokenize_line(line: String, lnum: u32) -> TkLine {
    if line.len() == 0 {
        return TkLine::empty();
    }
    let mut len = 0;
    let label = search_label(&line, lnum, len);
    len = Range::last(&label);
    let op = if len < line.len() {
        //still chars
        let op = search_op(&line[len..], lnum, len);
        len = Range::last(&op);
        op
    } else {
        None
    };
    let opds = if len < line.len() {
        //still chars
        let opds = search_op(&line[len..], lnum, len);
        len = Range::last(&opds);
        opds
    } else {
        None
    };
    let cmt: Option<Range> = if len < line.len() {
        //still chars
        search_comment(&line[len..], lnum, len)
    } else {
        None
    };

    let tk_line = TkLine {
        original: line,
        label,
        operation: op,
        operand: opds,
        comment: cmt,
    };

    return tk_line;
}

/**given a String seach for a Label. The label must be the first word of the line and dont have
 white spaces before.
**/
fn search_label(line: &str, lnum: u32, start: usize) -> Option<Range> {
    if let Some(idx) = line.char_indices().find(|&(_, c)| c == ' ' || c == '\t') {
        if idx.0 == 0 {
            return None;
        }
        let pos1 = Position {
            line: lnum,
            character: 0,
        };
        let pos2 = Position {
            line: lnum,
            character: (start + idx.0) as u32,
        };
        return Some(Range::new(pos1, pos2));
    };
    return None;
}

/**
 Search for the operation code.
 The op MUST have spaces or tab before, even if no label present.
*/
fn search_op(line: &str, lnum: u32, start: usize) -> Option<Range> {
    if let Some(index) = line.char_indices().find(|&(_, c)| c.is_alphabetic()) {
        if index.0 == 0 {
            return None;
        }
        let mut index2 = line[index.0..]
            .char_indices()
            .find(|&(_, c)| c == ' ' || c == '\t')
            .map(|(index, _)| index )
            .unwrap_or_else(|| line.len());
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
    None
}
#[cfg(test)]
mod test_tokenization {
    use super::*;
    #[test]
    fn correct_tokenize() {
        let line = String::from("TEST: MOVE.L a,b *comment");
        let a = TkLine {
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
        let a = TkLine {
            label: None,
            operand: None,
            operation: None,
            original: String::new(),
            comment: None,
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
