//! Contains the macros and structures needed to make good jasm

pub use jodin_common::{
    assembly::{asm_block::*, instructions::*},
    block,
};

#[macro_export]
macro_rules! jasm {
    (ASM_STYLE $($tt:tt)*) => {
        $crate::asm_style_jodin_assembly!($($tt)*)
    };
    ($($tt:tt)*) => {
        {
            let blk: $crate::AssemblyBlock = $crate::block![$($tt)*];
            blk.normalize()
        }
    };
}

#[macro_export]
macro_rules! var {
    ($var_id:expr => $value:expr) => {
        $crate::jasm![$value, $crate::Asm::SetVar($var_id)]
    };
    ($var_id:expr) => {
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
            $crate::block![
                if_false:
                $($if_false,)?
                $crate::Asm::Nop
            ],
            $crate::goto!(end_if),
            $crate::label!(if_true),
            $crate::block![
                if_true:
                $($if_true,)?
                $crate::Asm::Nop
            ],
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
            $crate::block![
                if_true:
                $($if_true,)?
                $crate::Asm::Nop
            ],
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
macro_rules! if_ {
    (($cond:expr) $blk:block) => {
        $crate::cond!(if ($cond) {
            $blk
        })
    };
    (($cond:expr) $blk:block else $else_blk:block) => {
        $crate::cond!(if ($cond) { $blk } else { $else_blk })
    };
}

#[macro_export]
macro_rules! while_ {
    (($cond:expr) $blk:block) => {
        $crate::cond!(while ($cond) {
            $blk
        })
    };
}

#[macro_export]
macro_rules! for_ {
    (($init:expr;$cond:expr;$delta:expr) $blk:block) => {
        $crate::cond!(
            for ($init; $cond; $delta) $blk
        )
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
    (-, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::Subtract
        ]
    };
    (/, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::Divide
        ]
    };
    (%, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::Remainder
        ]
    };

    // boolean
    (!, $e:expr) => {
        $crate::block![
            $e,
            $crate::Asm::Not,
        ]
    };
    (>0, $e:expr) => {
        $crate::block![
            $e,
            $crate::Asm::GT0,
        ]
    };
    (&, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::And
        ]
    };
    (|, $l:expr, $r:expr) => {
        $crate::block![
            $r;
            $l;
            $crate::Asm::Or
        ]
    };
    (==, $l:expr, $r:expr) => {
        $crate::expr!(&,
            $crate::expr!(>0,$crate::expr!(-, $l.clone(), $r.clone())),
            $crate::expr!(>0,$crate::expr!(-, $r.clone(), $l.clone()))
        )
    };
    (!=, $l:expr, $r:expr) => {
        $crate::expr!(!,
            $crate::expr!(==, $l, $r)
        )
    };
    (>, $l:expr, $r:expr) => {
        $crate::expr!(>0, $crate::expr!(-, $l, $r))
    };
    (<, $l:expr, $r:expr) => {
        $crate::expr!(>, $r, $l)
    };
    (<=, $l:expr, $r:expr) => {
        $crate::expr!(!,
            $crate::expr!(>, $l, $r)
        )
    };
    (>=, $l:expr, $r:expr) => {
        $crate::expr!(!,
            $crate::expr!(<, $l, $r)
        )
    };
}

#[macro_export]
macro_rules! boolify {
    ($e:expr) => {
        $crate::block![$e, $crate::Asm::Boolify]
    };
}

#[macro_export]
macro_rules! scope {
    (push) => {
        $crate::Asm::native_method("@push_scope", None)
    };
    (pop) => {
        $crate::Asm::native_method("@pop_scope", None)
    };
    (save) => {
        $crate::Asm::native_method("@save_scope", 1)
    };
    (load) => {
        $crate::Asm::native_method("@load_scope", 1)
    };
    (global) => {
        $crate::Asm::native_method("@global_scope", None)
    };
    (back) => {
        $crate::Asm::native_method("@back_scope", None)
    };
}

#[macro_export]
macro_rules! asm_style_jodin_assembly {

    (@ @ $($tt:tt)*) => {
        $crate::jasm![$($tt)*]
    };
    (@ pub $id:ident ; $($kw:ident $arg:expr ;)* @ $($output:tt)*) => {
        $crate::asm_style_jodin_assembly!(@ $($kw $arg ;)* @ $($output)* $crate::label!(pub $id); )
    };
    (@ $id:ident ;  $($kw:ident $arg:expr ;)* @ $($output:tt)*) => {
        $crate::asm_style_jodin_assembly!(@ $($kw $arg ;)* @ $($output)* $crate::label!($id); )
    };
    (@ push $e:expr; $($kw:ident $arg:expr ;)* @ $($output:tt)*) => {
        $crate::asm_style_jodin_assembly!(@ $($kw $arg ;)* @ $($output)* $crate::push!($e); )
    };
    (@ return $e:expr; $($kw:ident $arg:expr ;)* @ $($output:tt)*) => {
        $crate::asm_style_jodin_assembly!(@ $($kw $arg ;)* @ $($output)* $crate::return_!($e); )
    };

    ($($tt:tt)*) => {
        $crate::asm_style_jodin_assembly!(@ $($tt)* @)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let blk = jasm! {factorial:
            label!(pub factorial);
            var!(0 => pop!());
            var!(1 => value!(1u64));
            cond!(
                for (var!(2 => var!(0)); expr!(>, var!(2), value!(0u64)); var!(2 => expr!(+, var!(2), value!(1u64)))) {
                    block![
                        expr!(*, var!(2), var!(1));
                        var!(1 => pop!());
                    ]
                }
            );

            return_!(var!(1));
        };
        println!("{blk:#?}")
    }
}
