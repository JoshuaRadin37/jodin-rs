use jasm_macros::{call, jasm, label, native, push, return_};
use jodin_common::{block, init_logging};
use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::MinimumALU;
use jodin_rs_vm::scoped_memory::VMMemory;
use jodin_rs_vm::vm::VMBuilder;
use jodin_vm_kernel::KernelPlugin;
use jodin_vm_plugins::core::debug::LoggingPlugin;
use jodin_vm_plugins::dynamic_plugin;
use log::LevelFilter;
use std::process::exit;

fn main() {
    init_logging(LevelFilter::Info);
    let mut vm_builder = VMBuilder::new()
        .memory(VMMemory::default())
        .alu(MinimumALU)
        .build()
        .unwrap();

    assert!(
        vm_builder.plugin::<KernelPlugin>(),
        "couldn't load kernel plugin"
    );
    assert!(
        vm_builder.plugin::<LoggingPlugin>(),
        "couldn't load logging plugin"
    );

    vm_builder.load(jasm![
        label!(pub start);
        call!(~ __start);
        return_!();
    ]);

    let exit_code = vm_builder.run("start").unwrap();
    exit(exit_code as i32);
}
