use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct InitializeRequest{

}

#[derive(Serialize,Deserialize,PartialEq, Eq,Debug)]
pub struct Message{
    method: String,
}

pub fn encode(content:impl Serialize)->String{

    let v=serde_json::to_string(&content).unwrap();
    let a =format!("Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", v.len(),v);
    a
}
pub fn _decode(msg: &str)->Message{
   let parts: Vec<&str> = msg.split(r"\r\n\r\n").collect();
   let _length:i32=parts.first().unwrap()[17..].parse().unwrap();
   let v: Message= serde_json::from_str(parts.get(1).unwrap()).unwrap();
    v
}

#[cfg(test)]
mod tests{
    use serde::Deserialize;

    use super::*;
    
    #[derive(Serialize,Deserialize,Debug,PartialEq, Eq)]
    struct EncodingExample{
        testing :bool,
    }
    #[test]
    fn test_encoding(){
        let expected=r#"Content-Length: 16\r\n\r\n{"testing":true}"#;
        let ex=EncodingExample{testing:true};
        let actual=encode(ex);
        assert_eq!(actual,expected);
    }

    #[test]
    fn test_decoding(){
        let expected=Message{method:"hi".to_string()};
        let ac= r#"Content-Length: 15\r\n\r\n{"method":"hi"}"#;
        let actual=_decode(ac);

        assert_eq!(actual,expected);
    }
}

