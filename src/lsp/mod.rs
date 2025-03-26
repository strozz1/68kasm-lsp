use capabilities::*;
use serde::{Deserialize, Serialize};
pub mod state;
pub mod requests;
pub mod response;
pub mod capabilities;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,
    #[serde(rename = "textDocument")]
    pub text_document: Option<TextDocumentItem>,
    #[serde(rename = "contentChanges")]
    pub text_document_change: Option<Vec<TextDocumentContentChange>>,

    //for TextDocumentPositionParams
    pub position: Option<Position>,
    // diagnosis
    pub identifier: Option<String>,
    #[serde(rename = "previousResultId")]
    pub previous_result_id: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
    #[serde(rename = "capabilities")]
    pub server_capabilities: ServerCapabilities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub name: String,
    pub version: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentItem {
    pub version: Option<u32>,
    #[serde(rename = "languageId")]
    pub language_id: Option<String>,
    pub text: Option<String>,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentContentChange {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}
#[derive(Serialize,Clone, Deserialize, Debug, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Range {
        Range { start, end }
    }
    pub fn last(opt: &Option<Range>) -> usize {
        match opt {
            Some(r) => r.end.character as usize,
            None => 0,
        }
    }
    pub fn first(opt: &Option<Range>) -> usize {
        match opt {
            Some(r) => r.start.character as usize,
            None => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    uri: String,
    range: Range,
}

impl Location {
    pub fn new(uri: String, range: Range) -> Location {
        Location { uri, range }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Hover {
    contents: String,
}


pub enum SemanticTokenTypes {
    Keyword,
    Variable,
    Number,
    Operator,
    Comment,
    String,
    Macro,
    Type,
    Method,
}
impl SemanticTokenTypes {
    pub fn str(&self) -> String {
        match self {
            SemanticTokenTypes::Keyword => String::from("keyword"),
            SemanticTokenTypes::Variable => String::from("variable"),
            SemanticTokenTypes::Number => String::from("number"),
            SemanticTokenTypes::Operator => String::from("operator"),
            SemanticTokenTypes::Comment => String::from("comment"),
            SemanticTokenTypes::String => String::from("string"),
            SemanticTokenTypes::Macro => String::from("macro"),
            SemanticTokenTypes::Type => String::from("type"),
            SemanticTokenTypes::Method => String::from("method"),
        }
    }
    pub fn list() -> Vec<String> {
        let types = [
            SemanticTokenTypes::Keyword,
            SemanticTokenTypes::Variable,
            SemanticTokenTypes::Number,
            SemanticTokenTypes::Operator,
            SemanticTokenTypes::Comment,
            SemanticTokenTypes::String,
            SemanticTokenTypes::Macro,
            SemanticTokenTypes::Type,
            SemanticTokenTypes::Method,
        ];

        types.iter().map(|token| token.str()).collect()
    }
}

pub enum SemanticTokenModifiers {}
impl SemanticTokenModifiers {
    pub fn list() -> Vec<String> {
        vec![]
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensLegend {
    #[serde(rename = "tokenTypes")]
    pub token_types: Vec<String>,
    #[serde(rename = "tokenModifiers")]
    pub token_modifiers: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    delta: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensOptions {
    pub legend: SemanticTokensLegend,
    pub range: Option<bool>,
    pub full: Option<Delta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticRegistrationOptions {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFilter {
    language: Option<String>,
    scheme: Option<String>,
    pattern: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TexDocumentRegistrationOptions {
    #[serde(rename = "documentSelector")]
    document_selector: Option<Vec<DocumentFilter>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensRegistrationOptions {
    #[serde(flatten)]
    pub semantic_tokens_options: SemanticTokensOptions,
    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
    #[serde(flatten)]
    pub text_document_registration_opts: TexDocumentRegistrationOptions,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokens {
    #[serde(rename = "resultId")]
    result_id: Option<String>,
    data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticOptions {
    identifier: Option<String>,
    #[serde(rename = "interFileDependencies")]
    inter_file_dependencies: bool,
    #[serde(rename = "workSpaceDiagnostics")]
    work_space_diagnostics: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticRegistrationOptions {
    #[serde(flatten)]
    opts: DiagnosticOptions,
    #[serde(rename = "documentSelector")]
    document_selector: Option<Vec<DocumentFilter>>,
    id: Option<String>,
}
/**
 Represents a diagnostics, such as a compoler error or warning.
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnostic {
    pub range: Range,
    severity: Option<usize>,
    pub code: Option<u32>,
    #[serde(rename = "codeDescription")]
    pub code_description: Option<String>,
    pub source: Option<String>,
    pub message: String,
    //pub tags: Option<Vec<DiagnosticTag>>,
    //data: LSPany
}
#[derive(Serialize, Deserialize, Debug)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}
impl DiagnosticSeverity {
    pub fn idx(self: &DiagnosticSeverity) -> usize {
        match self {
            DiagnosticSeverity::Error => 1,
            DiagnosticSeverity::Warning => 2,
            DiagnosticSeverity::Information => 3,
            DiagnosticSeverity::Hint => 4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDiagnosticReport {
    kind: String,
    #[serde(rename = "resultId")]
    pub result_id: Option<String>,
    pub items: Option<Vec<Diagnostic>>,
    #[serde(rename = "relatedDocuments")]
    pub related_documents: Option<Vec<String>>,
}

pub enum DiagnosticKind {
    Full,
    _Unchanged,
}
impl DiagnosticKind {
    pub fn to_string(self: &DiagnosticKind) -> String {
        match self {
            DiagnosticKind::Full => String::from("full"),
            DiagnosticKind::_Unchanged => String::from("unchanged"),
        }
    }
}
