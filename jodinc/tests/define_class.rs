use jodin_asm::init_logging;
use jodin_common::error::JodinError;
use jodinc::parsing::parse_program;
use jodinc::process_jodin_node;
use log::LevelFilter;
use std::error::Error;

static JODIN_STRING: &str = r#"
public trait Shape {
    fn side_length(n: unsigned int) -> int;
    fn area() -> int;
    fn num_sides() -> int;
    fn perimeter() -> int {
        let sum = 0;
        let max = this.num_sides();
        for(unsigned int i = 0; i < max; ++i) {
            sum += this.side_length(i);
        }
        return sum;
    }
}

public class Rectangle : Shape {
    width: int;
    height: int;
    
    public fn new(width: int, heigh: int) {
        this.width = width;
        this.height = height;
    }
    
    @override(Shape::area) 
    public fn area() -> int {
        return this.width * this.length;
    }
    
    @override(Shape) 
    public fn side_length(n: unsigned int) -> int {
        if (n== 0 || n == 2) return this.width;
        if (n== 1 || n == 3) return this.length;
        return -1;
    }
}

public class Square<T> : Rectangle {
    public fn new(len: int) {
        super(len, len);
    }
  
    override public fn side_length(n: unsigned int) -> int {
        return this.width;
    }
}

"#;

fn _define_class() -> Result<(), JodinError> {
    init_logging(LevelFilter::Trace);
    let declaration = parse_program(JODIN_STRING)?;
    let (processed, env) = process_jodin_node(declaration)?;

    Ok(())
}

#[test]
fn define_class() {
    match _define_class() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    }
}
