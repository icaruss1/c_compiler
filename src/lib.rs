#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn newlines() -> Result<(), ()> {
        let bytes: Vec<u8> = fs::read("./c_files/newlines.c");
        assert_eq!(
            lexer::lex(bytes),
            Ok([
                10, 105, 110, 116, 32, 10, 109, 97, 105, 110, 10, 40, 32, 32, 32, 10, 41, 10, 123,
                10, 114, 101, 116, 117, 114, 110, 10, 48, 10, 59, 10, 125, 10
            ])
        );

        Ok(())
    }
}
