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
        // function is set to current symbol
        push T
        push 1
        create_array
        push current symbol to stack
        created_symbol = create_symbol()
        declare local input #1 
            with size get_size(T)
        collate local variables to locals
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
    }
and calling function<T>() is effectively the following pseduocode:

    get symbol function<T>
        do first:
            if no function<T> saved:
                function(T)
    function<T>()
        