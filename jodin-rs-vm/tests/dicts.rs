use jodin_common::assembly::instructions::{Asm, Assembly};
use jodin_common::assembly::location::AsmLocation;
use jodin_common::assembly::value::Value;
use jodin_common::core::function_names::RECEIVE_MESSAGE;
use jodin_common::init_logging;
use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::VMBuilder;
use log::{debug, LevelFilter};
use std::io::stdout;

#[test]
fn create_dict() {
    init_logging(LevelFilter::Trace);
    debug!("Creating dict");
    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut vm = VMBuilder::new()
            .memory(MinimumMemory::default())
            .alu(MinimumALU)
            .with_stdout(&mut buffer)
            .with_stderr(stdout())
            .build()
            .unwrap();

        let instructions = vec![
            Asm::label("__start"),
            Asm::push(Value::from([("ATTRIBUTE", "VALUE")])),
            Asm::SetVar(0),
            Asm::GetVar(0),
            Asm::get_attribute("ATTRIBUTE"),
            Asm::native_method("print", 1),
            Asm::push(vec!["ATTRIBUTE"]),
            Asm::push("get"),
            Asm::GetVar(0),
            Asm::SendMessage,
            Asm::native_method("print", 1),
            Asm::push(0u64),
            Asm::Return,
        ];
        vm.load(instructions);
        vm.run("__start").expect("VM should not fail");
    }
    let decoded = String::from_utf8(buffer).expect("Output should be in utf-8");
    assert_eq!(
        decoded, "VALUEVALUE",
        "VALUE should've been been printed twice, once statically and once dynamically"
    )
}

#[test]
fn dict_with_native_override() {
    init_logging(LevelFilter::Trace);
    debug!("Creating dict");
    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut vm = VMBuilder::new()
            .memory(MinimumMemory::default())
            .alu(MinimumALU)
            .with_stdout(&mut buffer)
            .with_stderr(stdout())
            .build()
            .unwrap();

        let receive_msg: Assembly = vec![
            Asm::label("__output"),
            Asm::push("OTHER VALUE"),
            Asm::Return,
        ];
        vm.load(receive_msg);

        let ignore_attribute = Value::from([
            (
                RECEIVE_MESSAGE,
                Value::Function(AsmLocation::Label("__output".to_string())),
            ),
            ("ATTRIBUTE", Value::from("VALUE")),
        ]);

        let instructions = vec![
            Asm::label("__start"),
            Asm::push(ignore_attribute),
            Asm::SetVar(0),
            Asm::push(vec!["ATTRIBUTE"]),
            Asm::push("get"),
            Asm::GetVar(0),
            Asm::SendMessage,
            Asm::native_method("print", 1),
            Asm::push(0u64),
            Asm::Return,
        ];
        vm.load(instructions);
        vm.run("__start").expect("VM should not fail");
    }
    let decoded = String::from_utf8(buffer).expect("Output should be in utf-8");
    assert_eq!(decoded, "OTHER VALUE");
}
