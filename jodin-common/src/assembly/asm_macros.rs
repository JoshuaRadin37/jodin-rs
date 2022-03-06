#[macro_export]
macro_rules! block {
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
    ($name:expr => $($asm:expr),* $(,)?) => {
        {
            use $crate::assembly::asm_block::{InsertAsm, AssemblyBlock};

            let mut output = AssemblyBlock::new(Some(&$name.to_string()));
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
    ($name:ident : $($asm:expr);* $(;)?) => {
        {
            use $crate::assembly::asm_block::{InsertAsm, AssemblyBlock};

            let mut output = AssemblyBlock::new(Some(&stringify!($name).to_string()));
            $(
                output.insert_asm($asm);
            )*
            output
        }
    };
    ($name:expr => $($asm:expr);* $(;)?) => {
        {
            use $crate::assembly::asm_block::{InsertAsm, AssemblyBlock};

            let mut output = AssemblyBlock::new(Some(&$name.to_string()));
            $(
                output.insert_asm($asm);
            )*
            output
        }
    };
    ($($asm:expr);* $(;)?) => {
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
