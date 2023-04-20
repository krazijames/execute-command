use std::process::{Command, ExitStatus};
use thiserror::Error;

pub fn parse(command_string: impl AsRef<str>) -> Result<Command> {
    let [program, args @ ..] = &shell_words::split(command_string.as_ref())?[..] else {
        return Ok(Command::new(""));
    };

    let mut command = Command::new(program);
    command.args(args);
    Ok(command)
}

pub fn status(command_string: impl AsRef<str>) -> Result<ExitStatus> {
    execute_status(&mut parse(command_string)?)
}

fn execute_status(command: &mut Command) -> Result<ExitStatus> {
    let status = command.status()?;

    match status.success() {
        true => Ok(status),
        false => Err(Error::ExitStatusError(status)),
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] shell_words::ParseError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("process exited unsuccessfully: {0}")]
    ExitStatusError(ExitStatus),
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

    #[test]
    fn test_status() {
        assert!(status("bash -c 'exit 0'").is_ok());
        assert!(status("bash -c 'exit 1'").is_err());
    }
}
