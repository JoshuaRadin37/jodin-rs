# Generics

    for <T> fn function(input: T) -> T {
        T output = input;
        return output;
    }


How do we implement this? Getting generics to work is super important.

## Conversion

### Everything is a function

Let's say that by the default, every object gets an implementation of some
static method called construct() that creates a defaulted value of that type.

    for <T> fn function() -> T {
        let output: T = T::construct();
        return output;
    }
becomes the following pseudocode
    
    fn function(T: Symbol) {
        push T
        push 1usize
        create_array
        push [current symbol] to stack // syscall 0
        created_symbol = create_symbol()
        if not created_symbol is saved:
            declare local input #1 
                with size get_size(T)
            collate local variables to locals // syscall 1
            bytecode = {
                symbol.get (${T})::construct
                call
                local.set #1
                local.get #1
                return
            }
            ptr = perm_heap.allocate(bytecode)
            info = FunctionInfo {
                instruction_pointer: ptr,
                locals_offset_size: locals
            }
            save current_symbol with info
        return (get symbol created_symbol)
    }
and calling function<T>() is effectively the following pseduocode:

    function(T.symbol)()
        