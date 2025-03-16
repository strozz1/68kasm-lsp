use std::io::{self, BufRead};
mod rpc;
mod lexer;
fn main() {
    println!("LSP initialiazed\n");

    let mut header = vec![0; 24];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        header.clear();
        let _red = handle.read_until('\n' as u8, &mut header).unwrap();

        if header.starts_with(b"Content-Length: ") {
            handle.read_until('\n' as u8, &mut header).unwrap();

            let index = header.iter().position(|&x| x == b'C').unwrap() + "Content-Length: ".len();
            let index2 = header.iter().position(|&x| x == b'\r').unwrap();
            let slice_num = &header[index..index2];
            let mut padded_bytes = [0u8; 4];
            padded_bytes[4 - slice_num.len()..].copy_from_slice(slice_num);
            let number: u32 = u32::from_be_bytes(padded_bytes);
            println!("\nNumber: {:?} at index {:?}", number, index2);
            return;
        }
    }
}
