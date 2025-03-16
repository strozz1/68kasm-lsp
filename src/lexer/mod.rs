

pub struct TkLine<'a>{
    label: Option<&'a str>,
    operation: Option<&'a str>,
    operand: Option<&'a str>,
    comment: Option<&'a str>,
    original: &'a str,
}
impl<'a> TkLine<'a>{
    fn empty()-> TkLine<'a>{
        TkLine{label:None,operand:None,operation:None,original:"",comment:None}
    }
}




pub fn tokenize_line(line: &str) ->Result<TkLine,String>{
    if line.len()==0{
        return Ok(TkLine::empty());
    }
    let mut len=0;
    let (label,l1)=search_label(line);
    len+=l1;
    let (op,l2)=search_op(&line[len..]);
    len+=l2;
    let (opds,l3)=search_op(&line[len..]);
    len+=l3;
    let (cmt,_)=search_comment(&line[len..]);
    let tk_line=TkLine{original: line,label,operation:op, operand:opds,comment:cmt};

    return Ok(tk_line);
}

/**given a String seach for a Label. The label must be the first word of the line and dont have
 white spaces before.
 Returns a slice containing the label or None if not found.
**/
fn search_label(line: &str)->(Option<&str>,usize){
    if let Some(idx)=line.char_indices().find(|&(_,c)| c == ' ' || c == '\t'){
        if idx.0 == 0 {return (None,0);}
        return (Some(&line[..idx.0]),idx.0);
    };
    return (None,0);
}

/**
  Search for the operation code.
  Returns a option of a pointer to a slice containing the op.
  The op MUST have spaces or tab before, even if no label present.
 */
fn search_op(line: &str)->(Option<&str>,usize){
    if let Some(index)=line.char_indices().find(|&(_,c)|c.is_alphabetic()){
        if index.0==0{
            return (None,0);
        }    
        let index2=line[index.0..].char_indices()
        .find(|&(_,c)| c == ' ' || c == '\t')
        .map(|(index, _)| index+1)  
        .unwrap_or_else(|| line.len());

        println!("op: {} {}\n",line,index2);
        return (Some(&line[index.0..index2]),index2);
    }    
    (None,0)
}
/*
 Search if comment line is present
*/
fn search_comment(line: &str)->(Option<&str>,usize){
    if let Some(index)=line.char_indices().find(|&(_,c)| c=='*'){
        return (Some(&line[index.0..]),index.0)
    }
    (None, 0)
}
#[cfg(test)]
mod test_tokenization{
    use super::*;
    #[test]
    fn correct_tokenize(){
        let line="TEST: MOVE.L a,b *comment";
        let a=TkLine{original:&line,label:Some(&line[..5]),operation:Some(&line[6..12]),
        operand:Some(&line[13..16]),comment:Some(&line[17..])};
        let b=tokenize_line(&line).expect("It must return a value");

        assert_eq!(a.label,b.label);
        assert_eq!(a.operation,b.operation);
        assert_eq!(a.operand,b.operand);
        assert_eq!(a.comment,b.comment);
        assert_eq!(a.original,b.original);

    }
     #[test]
    fn empty_line(){
        let e="";
        let a=TkLine{label:None,operand:None,operation:None,original:"",comment:None};
        let b=tokenize_line(e).expect("It must return a value");


        assert_eq!(a.label,b.label);
        assert_eq!(a.operation,b.operation);
        assert_eq!(a.operand,b.operand);
        assert_eq!(a.comment,b.comment);
        assert_eq!(a.original,b.original);

    }

    //TODO: when no label, the op if in the left is treated as a label

}




