extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "rust.pest"]
pub struct RustParser;

/// Loads a file from the file system into a string.
fn load_file(file: &std::path::Path) -> String {
    todo!()
}

/// Parses a string into an AST.
fn parse_string(file: String) -> String {
    todo!()
}

/// Inserts the value of an AST into an SQL database.
fn insert_ast() -> std::io::Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
