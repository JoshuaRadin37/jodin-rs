use jodin_asm::default_logging;
use jodin_asm::mvp::bytecode::Asm;
use jodin_asm::mvp::value::Value;
use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::VMBuilder;
use simplelog::info;
use std::io::Write;

const HELLO_WORLD: &str = "Hello, world!";

#[test]
fn hello_world() {
    default_logging();
    info!("Running hello world program");
    let mut buffer: Vec<u8> = Vec::new();
    let mut vm = VMBuilder::new()
        .memory(MinimumMemory::default())
        .alu(MinimumALU)
        .with_stdout(&mut buffer)
        .build();

    let instructions = vec![
        Asm::label("__start"),
        Asm::push(Value::from(vec![Value::from(HELLO_WORLD)])),
        Asm::push(Value::from("print")),
        Asm::push(Value::Native),
        Asm::SendMessage,
        Asm::push(0u64),
        Asm::Return,
    ];
    vm.load(instructions);
    vm.run("__start").expect("VM should not fail");
    drop(vm);
    let decoded = String::from_utf8(buffer).expect("Output should be in utf-8");
    assert_eq!(
        decoded, HELLO_WORLD,
        "Virtual machine should output {}",
        HELLO_WORLD
    )
}
