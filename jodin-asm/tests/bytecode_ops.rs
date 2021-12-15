use jodin_asm::mvp::bytecode::{Asm, Decode, Encode};

#[test]
fn can_serialize_and_deserialize() {
    let asm: Vec<Asm> = vec![Asm::push(10u64), Asm::push(10u64), Asm::Multiply];
    let debug_output = format!("{:?}", asm);
    let as_encoded = asm.encode();
    let encoded_size = as_encoded.len();
    println!(
        "{}",
        as_encoded
            .iter()
            .map(|b| format!("{:X}", b))
            .collect::<Vec<_>>()
            .join("")
    );
    let decoded = as_encoded.decode();
    println!("{:?}", decoded);
    let string = format!("{:?}", decoded);
    assert_eq!(
        string, debug_output,
        "If serialized/deserialized badly these values would be different"
    );
    assert!(
        string.bytes().len() > encoded_size,
        "The binary encoded size should be smaller than the debug output"
    );
}
