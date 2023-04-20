use std::process::{Command, ExitStatus, Output};
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
        false => Err(Error::ExitStatus(status)),
    }
}

pub fn string(command_string: impl AsRef<str>) -> Result<String> {
    execute_string(&mut parse(command_string)?)
}

fn execute_string(command: &mut Command) -> Result<String> {
    let output = command.output()?;

    if !output.status.success() {
        return Err(Error::Output(output));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("process exited unsuccessfully: {0}")]
    ExitStatus(ExitStatus),

    #[error("process exited unsuccessfully: {}", .0.status)]
    Output(Output),

    #[error(transparent)]
    ParseError(#[from] shell_words::ParseError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
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

    #[test]
    fn test_string() {
        assert_eq!(string("bash -c 'echo 1'").unwrap(), "1\n");
        assert!(string("bash -c 'exit 1'").is_err());
    }
}
