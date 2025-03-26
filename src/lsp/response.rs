use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub jsonrpc: String,
    pub id: Option<u32>,
}
impl Response {
    pub fn new(id: Option<u32>) -> Response {
        Response {
            id,
            jsonrpc: String::from("2.0"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResponse {
    #[serde(flatten)]
    response: Response,
    pub result: InitializeResult,
}
impl InitializeResponse {
    pub fn new(id: Option<u32>) -> InitializeResponse {
        let opts = SemanticTokensOptions {
            legend: SemanticTokensLegend {
                token_types: SemanticTokenTypes::list(),
                token_modifiers: SemanticTokenModifiers::list(),
            },
            full: Some(Delta { delta: true }),
            range: Some(false),
        };
        let sta = StaticRegistrationOptions { id: None };

        let tops = TexDocumentRegistrationOptions {
            document_selector: None,
        };
        let reg = SemanticTokensRegistrationOptions {
            semantic_tokens_options: opts,
            static_registration_options: sta,
            text_document_registration_opts: tops,
        };
        let diagnostic = DiagnosticOptions {
            identifier: None,
            inter_file_dependencies: false,
            work_space_diagnostics: false,
        };
        let diag_reg = DiagnosticRegistrationOptions {
            id: Some("sad1:".to_string()),
            opts: diagnostic,
            document_selector: None,
        };
        let server_cap = ServerCapabilities {
            hover_provider: Some(true),
            text_document_sync: 1,
            document_highlight_provider: Some(true),
            definition_provider: Some(true),
            semantic_tokens_provider: Some(reg),
            diagnostics_provider: Some(diag_reg),
        };
        let server_info: ServerInfo = ServerInfo {
            name: String::from("68kasm server"),
            version: Some(String::from("v-0.1")),
        };

        let init_res = InitializeResult {
            server_info,
            server_capabilities: server_cap,
        };
        InitializeResponse {
            response: Response::new(id),
            result: init_res,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct HoverResponse {
    #[serde(flatten)]
    response: Response,
    result: Hover,
}
impl HoverResponse {
    pub fn new(id: Option<u32>, hover: Hover) -> HoverResponse {
        HoverResponse {
            response: Response::new(id),
            result: hover,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionResponse {
    #[serde(flatten)]
    response: Response,
    result: Location,
}
impl DefinitionResponse {
    pub fn new(id: Option<u32>, result: Location) -> DefinitionResponse {
        DefinitionResponse {
            response: Response::new(id),
            result,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokenResponse {
    #[serde(flatten)]
    pub response: Response,
    pub result: SemanticTokens,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDiagnosticReportResponse {
    #[serde(flatten)]
    pub response: Response,
    pub result: DocumentDiagnosticReport,
}
impl DocumentDiagnosticReportResponse {
    pub fn new(
        id: Option<u32>,
        related_documents: Option<Vec<String>>,
        kind: DiagnosticKind,
        result_id: Option<String>,
        items: Option<Vec<Diagnostic>>,
    ) -> DocumentDiagnosticReportResponse {
        DocumentDiagnosticReportResponse {
            response: Response::new(id),
            result: DocumentDiagnosticReport {
                kind: kind.to_string(),
                related_documents,
                result_id,
                items,
            },
        }
    }
}
