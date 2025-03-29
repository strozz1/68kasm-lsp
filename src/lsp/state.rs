use super::super::lexer;
use log::info;

use std::collections::HashMap;

use super::*;

/**
 Represents an open file in the internal state of the server.
 It contains the raw contents, the version and if computed, the lexical tokens.
*/
#[derive(Debug, Clone)]
pub struct OpenedFile<'a> {
    data: String,
    tokens: Vec<lexer::TokenLine<'a>>,
    version: u32,
}
impl<'a> OpenedFile<'a> {
    pub fn new(data: String, version: u32) -> OpenedFile<'a> {
        OpenedFile {
            data,
            tokens: Vec::new(),
            version,
        }
    }
}
#[derive(Debug)]
pub struct State<'a> {
    pub files: HashMap<String, OpenedFile<'a>>,
}

impl<'a> State<'a> {
    /**
     When a user opens a document, we have to keep track of its state, so we have to save it. We can do that
    With this, The first argument is the uri and the second is the content
    */
    pub fn open_document(self: &mut State<'a>, uri: String, content: String) {
        self.files.insert(uri, OpenedFile::new(content, 0));
    }

    /**
     When a user edits a document, updates the current state of that file
    */
    pub fn edit_document(self: &mut State<'a>, uri: String, content: String) {
        self.files.insert(uri, OpenedFile::new(content, 0));
    }

    pub fn hover(self: &mut State<'a>, position: Position) -> Option<Hover> {
        //TODO
        let s = format!(
            "Hover from {} at line {}",
            position.character, position.line
        );
        Some(Hover { contents: s })
    }
    pub fn definition(self: &mut State<'a>, doc: TextDocumentItem) -> Option<Location> {
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

    /**
     Update the tokens from a TextDocumentItem. If the version of the file is the same, returns the unchanged content
    */
    fn update_tokens(self: &mut State<'a>, doc: TextDocumentItem) -> OpenedFile<'a> {
        let uri = &doc.uri;
        let (_,mut txt) = self
            .files
            .remove_entry(uri).expect("The file must exist in state");
        if doc.version == None || doc.version.unwrap() != txt.version {
            info!("New version of the file:'{}'. tokens updated\n",txt.version);
            let new = lexer::tokenize(&txt.data);
            txt.tokens = new;
            self.files.insert(uri.to_string(), txt.clone());
        }
        txt
    }
    pub fn tokens_full(
        self: &mut State<'a>,
        doc: TextDocumentItem,
        id: Option<String>,
    ) -> Option<SemanticTokens> {
        let file = self.update_tokens(doc);
        let num_lines = to_numeric_tk(&file.tokens);

        Some(SemanticTokens {
            result_id: id,
            data: num_lines,
        })
    }
    pub fn diagnostics(
        self: &mut State<'a>,
        id: Option<u32>,
        doc: TextDocumentItem,
        _identifier: Option<String>,
        prev_res_id: Option<String>,
    ) -> Option<response::DocumentDiagnosticReportResponse> {
        let kind = DiagnosticKind::Full;
        let related_docs = None;

        let tokens = self.update_tokens(doc);
        let diagnostics = self.diagnostics_from_tokens(tokens);
        let res = response::DocumentDiagnosticReportResponse::new(
            id,
            related_docs,
            kind,
            prev_res_id,
            diagnostics,
        );
        Some(res)
    }

    pub fn new() -> State<'a> {
        State {
            files: HashMap::new(),
        }
    }
    fn diagnostics_from_tokens(self: &mut State<'a>, file: OpenedFile) -> Option<Vec<Diagnostic>> {
        let lex = lexical(&file.tokens);
        Some(lex)
    }
}

fn to_numeric_tk(tk: &Vec<lexer::TokenLine>) -> Vec<u32> {
    let mut lines: Vec<u32> = Vec::with_capacity(tk.len());
    for (i, l) in tk.iter().enumerate() {
        let mut line = parse_tk(l, i);
        lines.append(&mut line);
    }
    lines
}

fn parse_tk(tk: &lexer::TokenLine, mut ln: usize) -> Vec<u32> {
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

pub fn lexical(lines: &Vec<lexer::TokenLine>) -> Vec<Diagnostic> {
    let mut list: Vec<Diagnostic> = Vec::new();
    for l in lines {
        if list.len() > 20 {
            break;
        }
        list.append(&mut lexical_analisis(l));
    }

    //TODO check label not first char, ended in ;
    list
}

fn lexical_analisis(tk: &lexer::TokenLine) -> Vec<Diagnostic> {
    let mut list = Vec::new();

    let label_diag = check_label(tk.label.clone());
    let op_diag = check_op(tk);

    if let Err(d) = op_diag {
        list.push(d)
    }
    if let Err(d) = label_diag {
        list.push(d)
    }
    list
}

fn check_op(tk: &lexer::TokenLine) -> Result<(), Diagnostic> {
    let op = &tk.get_op();
    match op {
        Some(str) => match lexer::language::is_instruction(str) {
            //todo use instr
            Ok(_) => Ok(()),
            Err(e) => {
                let operation = tk.operation.clone().unwrap();
                let range = Range::new(operation.start, operation.end);
                Err(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::Error.idx()),
                    code: None,
                    code_description: None,
                    source: None,
                    message: format!("{} '{}'.", e, str),
                })
            }
        },
        None => Ok(()),
    }
}
fn check_label(op: Option<Range>) -> Result<(), Diagnostic> {
    if let Some(o) = op {
        //check length
        if o.end.character - o.start.character - 1 > 8 {
            return Err(Diagnostic {
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
    Ok(())
}
