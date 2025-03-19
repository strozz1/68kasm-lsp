use log::info;
use log4rs;
use lsp::*;
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Write};
mod lexer;
mod lsp;
mod rpc;
fn main() {
    let mut state = state::State::new();
    log4rs::init_file("/home/strozzi/projects/lsp/log4rs.yml", Default::default()).unwrap();
    info!("Server has started\n");
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut header = vec![0; 24];

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
            let req = parse_request(&data);
            let resp = manage_request(req, &mut state);
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
        "textDocument/didOpen" => {
            let td = req.params.text_document?;
            let uri = td.uri;
            let text = td.text?;
            info!("File opened: {}\n", uri);
            state.open_document(uri, text);
            None
        }
        "initialize" => {
            let res = InitializeResponse::new(req.id);
            let r = rpc::encode(res);
            Some(r)
        }
        "initialized" => {
            info!("Initialized\n");
            None
        }
        "textDocument/didChange" => {
            let td = req.params.text_document?;
            let uri = td.uri;
            let text = req.params.text_document_change?;
            info!("File changed: {}\n", uri);
            state.edit_document(uri, text.as_slice()[0].text.clone());
            None
        }
        "textDocument/hover" => {
            info!("Hover\n");
            let pos = req.params.position?;
            let hover = state.hover(pos)?;
            let hover_resp = HoverResponse::new(req.id, hover);
            let r = rpc::encode(hover_resp);
            Some(r)
        }
        "textDocument/definition" => {
            info!("definition request\n");


            let doc = req.params.text_document?;
            let loc=state.definition(doc)?;
            let res = DefinitionResponse::new(req.id, loc);
            let r = rpc::encode(res);
            Some(r)
        }

        "textDocument/semanticTokens"=>{
            info!("SemanticTokens request\n");
            let doc=req.params.text_document?;
            let tk=state.tokens_full(doc,None)?;
            let res=SemanticTokenResponse{
                response:Response::new(req.id),
                result:tk
            };
            let r = rpc::encode(res);

            Some(r)
        }
        "textDocument/semanticTokens/full"=>{
            info!("Full SemanticTokens request\n");
            let doc=req.params.text_document?;
            let tk=state.tokens_full(doc,None)?;
            let res=SemanticTokenResponse{
                response:Response::new(req.id),
                result:tk
            };
            let r = rpc::encode(res);
            Some(r)
        }
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
