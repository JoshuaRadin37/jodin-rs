use jodin_common::init_logging;
use log::LevelFilter;

static ACKERMANN_FUNCTION: &str = r"
fn ackermann(m: int, n: int) -> int {
    
}
";

#[test]
fn ackermann_function() {
    init_logging(LevelFilter::Info);
}
