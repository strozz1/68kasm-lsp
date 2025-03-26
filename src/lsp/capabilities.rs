use super::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCapabilities {
    #[serde(rename = "hoverProvider")]
    pub hover_provider: Option<bool>,
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: u32,
    #[serde(rename = "documentHighlightProvider")]
    pub document_highlight_provider: Option<bool>,
    #[serde(rename = "definitionProvider")]
    pub definition_provider: Option<bool>,
    #[serde(rename = "semanticTokensProvider")]
    pub semantic_tokens_provider: Option<SemanticTokensRegistrationOptions>,
    #[serde(rename = "diagnosticProvider")]
    pub diagnostics_provider: Option<DiagnosticRegistrationOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilities {}
