use super::super::lexer;

use std::collections::HashMap;

use super::*;

#[derive(Debug)]
pub struct State {
    pub files: HashMap<String, String>,
}

impl State {
    /**
     When a user opens a document, we have to keep track of its state, so we have to save it. We can do that
    With this, The first argument is the uri and the second is the content
    */
    pub fn open_document(self: &mut State, uri: String, content: String) {
        self.files.insert(uri, content);
    }

    /**
     When a user edits a document, updates the current state of that file
    */
    pub fn edit_document(self: &mut State, uri: String, content: String) {
        self.files.insert(uri, content);
    }

    pub fn hover(self: &mut State, position: Position) -> Option<Hover> {
        //TODO
        let s = format!(
            "Hover from {} at line {}",
            position.character, position.line
        );
        Some(Hover { contents: s })
    }
    pub fn definition(self: &mut State, doc: TextDocumentItem) -> Option<Location> {
        let pos = Position {
            line: 0,
            character: 0,
        };
        let mut pos2 = pos.clone();
        pos2.character += 5;
        let range = Range::new(pos, pos2);
        let loc = Location::new(doc.uri, range);
        Some(loc)
    }
    pub fn tokens_full(
        self: &mut State,
        doc: TextDocumentItem,
        id: Option<String>,
    ) -> Option<SemanticTokens> {
        let uri = &doc.uri;
        let txt = self.files.get(uri)?;
        let tk = lexer::tokenize(txt);

        let lines = parse_lines(tk);
        let st = SemanticTokens {
            result_id: id,
            data: lines,
        };

        Some(st)
    }
    pub fn diagnostics(
        self: &mut State,
        id: Option<u32>,
        doc: TextDocumentItem,
        _identifier: Option<String>,
        prev_res_id: Option<String>,
    ) -> Option<DocumentDiagnosticReportResponse> {
        let kind = DiagnosticKind::Full;
        let related_docs = None;
        let items = self.get_diagnostics(doc);
        let res = DocumentDiagnosticReportResponse::new(id, related_docs, kind, prev_res_id, items);
        Some(res)
    }

    pub fn new() -> State {
         State {
            files: HashMap::new(),
        }
    }
    fn get_diagnostics(self: &mut State, doc: TextDocumentItem) -> Option<Vec<Diagnostic>> {
        let uri = &doc.uri;
        let txt = self.files.get(uri)?;
        let lines = lexer::tokenize(txt);
        //xd
        let anal = lexical(&lines);
        Some(anal)
    }
}

fn parse_lines(tk: Vec<lexer::TkLine>) -> Vec<u32> {
    let mut lines: Vec<u32> = Vec::with_capacity(tk.len());
    for (i, l) in tk.iter().enumerate() {
        let mut line = parse_tk(l, i);
        lines.append(&mut line);
    }
    lines
}

fn parse_tk(tk: &lexer::TkLine, mut ln: usize) -> Vec<u32> {
    let mut line: Vec<u32> = Vec::with_capacity(5);
    let mut last_start = 0;
    if ln != 0 {
        ln = 1
    };

    if tk.is_empty() {
        line.push(1);
        line.push(0);
        line.push(0);
        line.push(0);
        line.push(0);
        return line;
    }
    if let Some(lb) = &tk.label {
        line.push(ln as u32); //line
        line.push(lb.start.character); //start relative
        line.push(lb.end.character - lb.start.character); //end
        line.push(8); //end
        line.push(0); //operdand
        last_start = lb.start.character; // for relative
        ln = 0;
    }
    if let Some(op) = &tk.operation {
        line.push(ln as u32);
        line.push(op.start.character - last_start); //start
        line.push(op.end.character - op.start.character); //end
        line.push(3); //end
        line.push(0); //operdand
        last_start = op.start.character; // for relative
        ln = 0;
    }
    if let Some(op) = &tk.operand {
        line.push(ln as u32);
        line.push(op.start.character - last_start); //start
        line.push(op.end.character - op.start.character); //end
        line.push(2); //operdand
        line.push(0); //operdand
        last_start = op.start.character; // for relative
        ln = 0;
    }
    if let Some(comment) = &tk.comment {
        line.push(ln as u32);
        line.push(comment.start.character - last_start); //start
        line.push(comment.end.character - comment.start.character); //end
        line.push(4); //operdand
        line.push(0); //operdand
    }

    line
}

pub fn lexical(lines: &Vec<lexer::TkLine>) -> Vec<Diagnostic> {
    let mut list: Vec<Diagnostic> = Vec::new();
    for l in lines {
        if list.len()> 10{
    break;
}
        list.append(&mut lexical_analisis(l));
    }

    //TODO check label not first char, ended in ;
    list
}

fn lexical_analisis(tk: &lexer::TkLine) -> Vec<Diagnostic> {
    let mut list = Vec::new();

    let label_diag = check_label(tk.label.clone());
    let op_diag = check_op(tk);

    if let Some(d) = op_diag {
        list.push(d)
    }
    if let Some(d) = label_diag {
        list.push(d)
    }
    list
}

fn check_op(tk: &lexer::TkLine) -> Option<Diagnostic> {
    let op = &tk.get_op();
    let operation = tk.operation.clone()?;
    match op {
        Some(str) => {
            if !lexer::language::is_instruction(str) {
                let range = Range::new(operation.start, operation.end);
                Some(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::Error.idx()),
                    code: None,
                    code_description: None,
                    source: None,
                    message: format!("Invalid intruction '{}'.",str),
                })
            } else {
                None
            }
        }
        None => None,
    }
}
fn check_label(op: Option<Range>) -> Option<Diagnostic> {
    if let Some(o) = op {
        //check length
        if o.end.character - o.start.character - 1 > 8 {
            return Some(Diagnostic {
                range: o,
                severity: Some(DiagnosticSeverity::Warning.idx()),
                code: None,
                message: String::from(
                    "Labels will only use the first 8 characters of the label. Consider removing the rest.",
                ),
                code_description: None,
                source: None,
            });
        }
    }
    None
}
