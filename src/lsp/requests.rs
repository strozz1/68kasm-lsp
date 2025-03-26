use super::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Option<u32>,
    pub method: String,
    pub params: Params,
}
