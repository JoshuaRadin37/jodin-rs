#[macro_export]
macro_rules! jasm {
     ($name:ident : $($asm:expr),* $(,)?) => {
        {
            use $crate::assembly::asm_block::{InsertAsm, AssemblyBlock};

            let mut output = AssemblyBlock::new(Some(&stringify!($name).to_string()));
            $(
                output.insert_asm($asm);
            )*
            output
        }
    };
    ($($asm:expr),* $(,)?) => {
        {
            use $crate::assembly::asm_block::{InsertAsm, AssemblyBlock};
            let mut output = AssemblyBlock::new(None);
            $(
                output.insert_asm($asm);
            )*
            output
        }
    };

}
