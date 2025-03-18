use serde::{Deserialize, Serialize};
pub mod state;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Option<u32>,
    pub method: String,
    pub params: Params,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub jsonrpc: String,
    pub id: Option<u32>,
}
impl Response{
    pub fn new(id: Option<u32>)->Response{
        Response{
            id,
            jsonrpc: String::from("2.0"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResponse{
    #[serde(flatten)]
    response: Response,
    result:InitializeResult
}
impl InitializeResponse {
    pub fn new(id: Option<u32>) -> InitializeResponse {
        let server_cap = ServerCapabilities {
            hover_provider: Some(true),
            text_document_sync: 1,
            document_highlight_provider: Some(true),
            definition_provider: Some(true),
        };
        let server_info: ServerInfo = ServerInfo {
            name: String::from("68kasm server"),
            version: Some(String::from("v-0.1")),
        };

        let init_res = InitializeResult {
            server_info,
            server_capabilities: server_cap,
        };
        return InitializeResponse {
            response:Response::new(id),
            result: init_res,
        };
    }
}


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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    #[serde(rename = "serverInfo")]
    pub server_info:ServerInfo,
    #[serde(rename = "capabilities")]
    pub server_capabilities: ServerCapabilities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub name: String,
    pub version: Option<String>,
}

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilities {}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentItem {
    pub version: Option<u32>,
    #[serde(rename = "languageId")]
    pub language_id: Option<String>,
    pub text: Option<String>,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentContentChange{
    pub text: String,
}


#[derive(Serialize,Clone, Deserialize, Debug)]
pub struct Position{
    pub line: u32,
    pub character: u32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Range{
    start: Position,
    end: Position,

}
impl Range{
    pub fn new(start:Position,end:Position)->Range{
        Range{
            start,
            end
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location{
    uri: String,
    range: Range,
}

impl Location{
    pub fn new(uri:String,range:Range)->Location{
        Location{
            uri,
            range
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Hover{
    contents: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverResponse{
    #[serde(flatten)]
    response: Response,
    result: Hover
}
impl HoverResponse{
    pub fn new(id:Option<u32>, hover: Hover)->HoverResponse{
        HoverResponse{
            response: Response::new(id),
            result:hover
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionResponse{
    #[serde(flatten)]
    response:Response,
    result:Location
}
impl DefinitionResponse{
    pub fn new(id:Option<u32>,result:Location)->DefinitionResponse{
        DefinitionResponse{
            response:Response::new(id),
            result
        }
    }
}
