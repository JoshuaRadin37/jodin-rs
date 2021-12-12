use std::io::Write;

#[test]
fn hello_world() {
    let mut buffer: Vec<u8> = Vec::new();
    let buf_box: Box<dyn Write> = Box::new(&mut buffer);

    buffer.push(0);
}
