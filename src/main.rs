use log::info;
use log4rs;
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Write};
use lsp::*;
mod lexer;
mod rpc;
mod lsp;
fn main() {
    let mut state=state::State::new();
    log4rs::init_file("/home/strozzi/projects/lsp/log4rs.yml", Default::default()).unwrap();
    info!("Server has started\n");
    let stdin = io::stdin();
    let stdout = io::stdout();
    let tmp=b"Content-Length: 3594\r\n\r\n{\"id\":1,\"method\":\"initialize\",\"params\":{\"workspaceFolders\":null,\"trace\":\"off\",\"processId\":12731,\"clientInfo\":{\"name\":\"Neovim\",\"version\":\"0.10.3+g9b5ee7df4e\"},\"workDoneToken\":\"1\",\"rootPath\":null,\"rootUri\":null,\"capabilities\":{\"workspace\":{\"didChangeConfiguration\":{\"dynamicRegistration\":false},\"workspaceFolders\":true,\"applyEdit\":true,\"symbol\":{\"dynamicRegistration\":false,\"symbolKind\":{\"valueSet\":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]}},\"didChangeWatchedFiles\":{\"dynamicRegistration\":false,\"relativePatternSupport\":true},\"semanticTokens\":{\"refreshSupport\":true},\"inlayHint\":{\"refreshSupport\":true},\"workspaceEdit\":{\"resourceOperations\":[\"rename\",\"create\",\"delete\"]},\"configuration\":true},\"general\":{\"positionEncodings\":[\"utf-16\"]},\"textDocument\":{\"hover\":{\"dynamicRegistration\":true,\"contentFormat\":[\"markdown\",\"plaintext\"]},\"inlayHint\":{\"dynamicRegistration\":true,\"resolveSupport\":{\"properties\":[\"textEdits\",\"tooltip\",\"location\",\"command\"]}},\"diagnostic\":{\"dynamicRegistration\":false},\"definition\":{\"linkSupport\":true,\"dynamicRegistration\":true},\"semanticTokens\":{\"dynamicRegistration\":false,\"tokenModifiers\":[\"declaration\",\"definition\",\"readonly\",\"static\",\"deprecated\",\"abstract\",\"async\",\"modification\",\"documentation\",\"defaultLibrary\"],\"serverCancelSupport\":false,\"augmentsSyntaxTokens\":true,\"tokenTypes\":[\"namespace\",\"type\",\"class\",\"enum\",\"interface\",\"struct\",\"typeParameter\",\"parameter\",\"variable\",\"property\",\"enumMember\",\"event\",\"function\",\"method\",\"macro\",\"keyword\",\"modifier\",\"comment\",\"string\",\"number\",\"regexp\",\"operator\",\"decorator\"],\"formats\":[\"relative\"],\"requests\":{\"full\":{\"delta\":true},\"range\":false},\"overlappingTokenSupport\":true,\"multilineTokenSupport\":false},\"references\":{\"dynamicRegistration\":false},\"implementation\":{\"linkSupport\":true},\"typeDefinition\":{\"linkSupport\":true},\"signatureHelp\":{\"dynamicRegistration\":false,\"signatureInformation\":{\"documentationFormat\":[\"markdown\",\"plaintext\"],\"activeParameterSupport\":true,\"parameterInformation\":{\"labelOffsetSupport\":true}}},\"synchronization\":{\"dynamicRegistration\":false,\"willSaveWaitUntil\":true,\"didSave\":true,\"willSave\":true},\"documentHighlight\":{\"dynamicRegistration\":false},\"codeAction\":{\"dynamicRegistration\":true,\"isPreferredSupport\":true,\"dataSupport\":true,\"resolveSupport\":{\"properties\":[\"edit\"]},\"codeActionLiteralSupport\":{\"codeActionKind\":{\"valueSet\":[\"\",\"quickfix\",\"refactor\",\"refactor.extract\",\"refactor.inline\",\"refactor.rewrite\",\"source\",\"source.organizeImports\"]}}},\"callHierarchy\":{\"dynamicRegistration\":false},\"rename\":{\"dynamicRegistration\":true,\"prepareSupport\":true},\"publishDiagnostics\":{\"tagSupport\":{\"valueSet\":[1,2]},\"dataSupport\":true,\"relatedInformation\":true},\"documentSymbol\":{\"dynamicRegistration\":false,\"hierarchicalDocumentSymbolSupport\":true,\"symbolKind\":{\"valueSet\":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]}},\"formatting\":{\"dynamicRegistration\":true},\"rangeFormatting\":{\"dynamicRegistration\":true},\"completion\":{\"dynamicRegistration\":false,\"contextSupport\":false,\"completionItemKind\":{\"valueSet\":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25]},\"completionList\":{\"itemDefaults\":[\"editRange\",\"insertTextFormat\",\"insertTextMode\",\"data\"]},\"completionItem\":{\"snippetSupport\":false,\"commitCharactersSupport\":false,\"preselectSupport\":false,\"deprecatedSupport\":false,\"documentationFormat\":[\"markdown\",\"plaintext\"]}},\"declaration\":{\"linkSupport\":true}},\"window\":{\"workDoneProgress\":true,\"showMessage\":{\"messageActionItem\":{\"additionalPropertiesSupport\":false}},\"showDocument\":{\"support\":true}}}},\"jsonrpc\":\"2.0\"}New request:: method: 'initialize'
InitializeParams { client_info: ClientInfo { name: \"Neovim\", version: \"0.10.3+g9b5ee7df4e\" } }";
    let mut header = vec![0; 24];
    let _cursor = Cursor::new(&tmp[..]);

    let mut handle = BufReader::new(stdin);
    let mut writer = BufWriter::new(stdout);
    loop {
        header.clear();
        let _red = handle.read_until(b'\n', &mut header).unwrap();

        if header.starts_with(b"Content-Length: ") {
            handle.read_until('\n' as u8, &mut header).unwrap();

            let index = header.iter().position(|&x| x == b'C').unwrap() + "Content-Length: ".len();
            let index2 = header.iter().position(|&x| x == b'\r').unwrap();
            let slice_num = &header[index..index2];
            let mut padded_bytes = [0u8; 4];
            padded_bytes[4 - slice_num.len()..].copy_from_slice(slice_num);
            let str_value = std::str::from_utf8(slice_num).unwrap();

            let length: i32 = str_value.parse().unwrap();
            //let length: u32 = u32::from_be_bytes(padded_bytes);
            //info!("Number: {:?} at index {:?}\n", length, index2);
            let mut data = vec![0u8; length as usize];
            let _ = handle.read_exact(&mut data);
            show_json(&data[..]);
            let req = parse_request(&data);
            let resp = manage_request(req,&mut state);
            if let Some(r) = resp {
                let _ = writer.write(&r.as_bytes()).unwrap();
                let _ = writer.flush();
                info!("Response sent\n");
            }
            data.clear();
            header.clear();
        }
    }
}
fn show_json(str: &[u8]) {
    info!("JSON:\n");
    info!("{:?}\n", String::from_utf8_lossy(str));
}

fn parse_request(data: &[u8]) -> Request {
    let req: Request = serde_json::from_slice(data).expect("Error deserializing");
    return req;
}


fn manage_request(req: Request, state: &mut state::State) -> Option<String> {
    info!("Method: '{}'\n", req.method);
    match &req.method[..] {
        "textDocument/didOpen" =>{
            let td=req.params.text_document?;
            let uri=td.uri;
            let text=td.text?;
            info!("File opened: {}\n",uri);
            state.open_document(uri,text);
            None
        },
        "initialize" => {
            let res = InitializeResponse::new(req.id);
            let r = rpc::encode(res);
            Some(r)
        },
        "initialized"=>{
            info!("Initialized\n");
            None
        },
        "textDocument/didChange" =>{
            let td=req.params.text_document?;
            let uri=td.uri;
            let text=req.params.text_document_change?;
            info!("File changed: {}\n",uri);
            state.edit_document(uri,text.as_slice()[0].text.clone());
            None
        },
        "textDocument/hover" =>{
            info!("Hover\n");
            let pos=req.params.position?;
            let hover=state.hover(pos)?;
            let hover_resp=HoverResponse::new(req.id,hover);
            let r=rpc::encode(hover_resp);
            Some(r)
        },
        _ => None,
    }
}
#[cfg(test)]
mod test_main {
    use super::*;
    #[test]
    fn parse_input_req() {
        let input = b"{\"jsonrpc\": \"2.0\",\"id\": 1,\"method\": \"textDocument/completion\"}";
        parse_request(input);
    }
}
