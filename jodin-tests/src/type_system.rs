use jodinc::test_runner::ProjectBuilder;

#[test]
fn create_class() {
    let builder = ProjectBuilder::new("create_class").use_string(
        r#"



            fn print(value: int) {
                __NATIVE("print", value);
            }

            fn println(value: void) {
                print(value);
                print("\n");
            }

            fn main() -> unsigned int {

                return 0u;
            }

            "#,
    );

    let result = builder.compile_to_jasm().unwrap();
    println!("{:#?}", result);
}