use jodin_common::assembly::instructions::Asm;
use jodin_common::assembly::value::Value;
use jodin_common::init_logging;
use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::VMBuilder;
use log::{debug, LevelFilter};

const HELLO_WORLD: &str = "Hello, world!";
const BYE_WORLD: &str = "Goodbye, world!";

#[test]
fn hello_world() {
    init_logging(LevelFilter::Info);
    debug!("Running hello world program");
    let mut buffer: Vec<u8> = Vec::new();
    let mut err_buffer: Vec<u8> = Vec::new();
    {
        let mut vm = VMBuilder::new()
            .memory(MinimumMemory::default())
            .alu(MinimumALU)
            .with_stdout(&mut buffer)
            .with_stderr(&mut err_buffer)
            .build()
            .unwrap();

        let instructions = vec![
            Asm::label("__start"),
            Asm::push(HELLO_WORLD),
            Asm::NativeMethod("print".to_string(), 1),
            Asm::push(Value::from(vec![Value::from(2u64), Value::from(BYE_WORLD)])),
            Asm::push(Value::from("write")),
            Asm::push(Value::Native),
            Asm::SendMessage,
            Asm::push(0u64),
            Asm::Return,
        ];
        vm.load(instructions);
        vm.run("__start").expect("VM should not fail");
    }
    let decoded = String::from_utf8(buffer).expect("Output should be in utf-8");
    assert_eq!(
        decoded, HELLO_WORLD,
        "Virtual machine should output {}",
        HELLO_WORLD
    );
    let decoded = String::from_utf8(err_buffer).expect("Output should be in utf-8");
    assert_eq!(
        decoded, BYE_WORLD,
        "Virtual machine should output {} to error output",
        BYE_WORLD
    );
}
