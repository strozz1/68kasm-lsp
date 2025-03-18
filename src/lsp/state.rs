use std::collections::HashMap;



#[derive(Debug)]
pub struct State{
    pub files: HashMap<String,String> 
}

impl State{
    /**
     When a user opens a document, we have to keep track of its state, so we have to save it. We can do that
    With this, The first argument is the uri and the second is the content
    */
    pub fn open_document(self: &mut State,uri: String,content:String){
        self.files.insert(uri,content);
    }

    /**
     When a user edits a document, updates the current state of that file
    */
    pub fn edit_document(self: &mut State,uri: String,content:String){
        self.files.insert(uri,content);
    }

    pub fn hover(self: &mut State,position: super::Position)->Option<super::Hover>{
        //TODO
        let s= format!("Hover from {} at line {}",position.character,position.line);
        Some(super::Hover{contents:s})
    }
    pub fn definition(self: &mut State,doc:super::TextDocumentItem)->Option<super::Location>{
            let pos=super::Position{
                line: 0,
                character: 0,
            };
            let mut pos2 = pos.clone();
            pos2.character += 5;
            let range=super::Range::new(pos,pos2);
            let loc = super::Location::new(doc.uri, range);
            Some(loc)
    }

    pub fn new()->State{
        return State{
            files:HashMap::new(),
        }
    }
}
