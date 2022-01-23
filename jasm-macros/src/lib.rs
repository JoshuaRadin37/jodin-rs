//! Contains the macros and structures needed to make good jasm

pub use jodin_common::{
    assembly::{asm_block::*, instructions::*},
    block,
};

#[macro_export]
macro_rules! jasm {
    ($($tt:tt)*) => {
        {
            let blk: $crate::AssemblyBlock = block![$($tt)*];
            blk.normalize()
        }
    };
}

#[macro_export]
macro_rules! var {
    ($var_id:literal = $value:expr) => {
        jasm![$value, $crate::Asm::SetVar($var_id)]
    };
    ($var_id:literal) => {
        $crate::Asm::GetVar($var_id)
    };
}

#[macro_export]
macro_rules! label {
    (pub $id:ident) => {
        $crate::Asm::pub_label(stringify!($id))
    };
    ($id:ident) => {
        $crate::Asm::label($crate::rel_label(stringify!($id)))
    };
}

#[macro_export]
macro_rules! goto {
    ($id:ident) => {
        $crate::Asm::goto($crate::rel_label(stringify!($id)))
    };
    (pub $id:ident) => {
        $crate::Asm::goto(stringify!($id))
    };
}

#[macro_export]
macro_rules! cond_goto {
    ($id:ident) => {
        $crate::Asm::cond_goto($crate::rel_label(stringify!($id)))
    };
    (pub $id:ident) => {
        $crate::Asm::cond_goto(stringify!($id))
    };
}

#[macro_export]
macro_rules! pop {
    () => {
        $crate::Asm::Pop
    };
}

#[macro_export]
macro_rules! return_ {
    () => {
        $crate::Asm::Return
    };
    ($expr:expr) => {
        $crate::block! {
            $expr,
            $crate::return_!()
        }
    };
}

#[macro_export]
macro_rules! push {
    ($($value:expr),*) => {
        $crate::block![
            $($crate::Asm::push($value)),*
        ]
    };
}

#[macro_export]
macro_rules! value {
    ($value:expr) => {
        $crate::push![$value]
    };
}

#[macro_export]
macro_rules! cond {
    (if ($cond:expr) {
        $($if_true:expr)?
    } else {
        $($if_false:expr)?
    }) => {
        $crate::block![
            if_block: $cond,
            // if not zero (aka falls through if condition is false)
            $crate::cond_goto!(if_true),
            $crate::label!(if_false),
            $($if_false,)?
            $crate::goto!(end_if),
            $crate::label!(if_true),
            $($if_true,)?
            $crate::goto!(end_if),
            $crate::label!(end_if)
        ]
    };
    (if ($cond:expr) { $($if_true:expr)? }) => {
        $crate::cond! (if ($cond) { $($if_true)? } else { })
    };
    (while ($cond:expr) { $($if_true:expr)?} ) => {
        $crate::block![while_block:
            $crate::label!(start_while),
            $cond,
            $crate::cond_goto!(start_while_block),
            $crate::goto!(end_while),
            $crate::label!(start_while_block),
            $($if_true,)?
            $crate::goto!(start_while),
            $crate::label!(end_while)
        ]
    };
    (for ($start:expr; $cond:expr; $delta:expr) { $($if_true:expr)? } ) => {
        $crate::block![for_block:
            $start,
            $crate::cond!{
                while ($cond) {
                    $crate::block![
                        $($if_true,)?
                        $delta
                    ]
                }
            }
        ]
    };
}

#[macro_export]
macro_rules! expr {
    (+, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::Add
        ]
    };
    (*, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::Multiply
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let blk = block! {factorial:
            label!(pub factorial);
            var!(0 = pop!());
            var!(1 = value!(1u64));
            cond!(
                for (var!(2 = var!(0)); value![true]; var!(2 = expr!(+, var!(2), value!(1u64)))) {
                    block![
                        expr!(*, var!(2), var!(1));
                        var!(1 = pop!());
                    ]
                }
            );
            return_!(var!(1))
        };
        println!("{blk:#?}")
    }
}
