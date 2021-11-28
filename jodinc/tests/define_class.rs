use jodinc::parsing::parse_program;
use jodinc::{init_logging, process_jodin_node};
use log::LevelFilter;
use std::error::Error;

static JODIN_STRING: &str = r#"
public trait Shape {
    fn side_length(this, n: unsigned int) -> int;
    fn area(this) -> int;
}

public class Rectangle : Shape {
    width: int;
    height: int;
    
    public fn new(this, width: int, heigh: int) {
        this.width = width;
        this.height = height;
    }
    
    override public fn area(this) -> int {
        return this.width * this.length;
    }
    
    override(Shape) public fn side_length(this, n: unsigned int) -> int {
        if (n== 0 || n == 2) return this.width;
        if (n== 1 || n == 3) return this.length;
    }
}

public class Square : Rectangle {
    public fn new(this, len: int) {
        super(len, len);
    }
  
    override(Shape) public fn side_length(this, n: unsigned int) -> int {
        return this.width;
    }
}

"#;

fn _define_class() -> Result<(), Box<dyn Error>> {
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
