use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] shell_words::ParseError),
}

pub fn parse(command_string: impl AsRef<str>) -> Result<Command, Error> {
    let [program, args @ ..] = &shell_words::split(command_string.as_ref())?[..] else {
        return Ok(Command::new(""));
    };

    let mut command = Command::new(program);
    command.args(args);
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let command = parse(r#"p a b"#).unwrap();
        assert_eq!(command.get_program(), "p");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["a", "b"]);

        let command = parse(r#"p a "b c""#).unwrap();
        assert_eq!(command.get_program(), "p");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["a", "b c"]);

        let command = parse(r#"p "a\"b""#).unwrap();
        assert_eq!(command.get_program(), "p");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["a\"b"]);

        let command = parse(r#"p "a"#);
        assert!(command.is_err());
    }
}
