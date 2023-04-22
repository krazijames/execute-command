use std::process::{Command, ExitStatus, Output};
use thiserror::Error;

pub trait ExecuteCommand {
    fn parse(command_string: impl AsRef<str>) -> Result<Command>;
    fn execute_status(&mut self) -> Result<ExitStatus>;
    fn execute_output(&mut self) -> Result<Output>;
    fn execute_string(&mut self) -> Result<String>;
}

impl ExecuteCommand for Command {
    fn parse(command_string: impl AsRef<str>) -> Result<Command> {
        parse(command_string)
    }

    fn execute_status(&mut self) -> Result<ExitStatus> {
        execute_status(self)
    }

    fn execute_output(&mut self) -> Result<Output> {
        execute_output(self)
    }

    fn execute_string(&mut self) -> Result<String> {
        execute_string(self)
    }
}

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

pub fn output(command_string: impl AsRef<str>) -> Result<Output> {
    execute_output(&mut parse(command_string)?)
}

pub fn string(command_string: impl AsRef<str>) -> Result<String> {
    execute_string(&mut parse(command_string)?)
}

fn execute_status(command: &mut Command) -> Result<ExitStatus> {
    let status = command.status()?;

    match status.success() {
        true => Ok(status),
        false => Err(Error::ExitStatus(status)),
    }
}

fn execute_output(command: &mut Command) -> Result<Output> {
    let output = command.output()?;

    match output.status.success() {
        true => Ok(output),
        false => Err(Error::Output(output)),
    }
}

fn execute_string(command: &mut Command) -> Result<String> {
    let output = execute_output(command)?;

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

    const BASE_COMMAND: &str = if cfg!(windows) { "cmd /C" } else { "sh -c" };

    #[test]
    fn test_parse() {
        let command = Command::parse(r#"p a b"#).unwrap();
        assert_eq!(command.get_program(), "p");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["a", "b"]);

        let command = Command::parse(r#"p a "b c""#).unwrap();
        assert_eq!(command.get_program(), "p");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["a", "b c"]);

        let command = Command::parse(r#"p "a\"b""#).unwrap();
        assert_eq!(command.get_program(), "p");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["a\"b"]);

        let command = Command::parse(r#"p "a"#);
        assert!(command.is_err());
    }

    #[test]
    fn test_execute_status() {
        assert!(Command::parse(format!("{BASE_COMMAND} 'exit 0'"))
            .unwrap()
            .execute_status()
            .is_ok());

        assert!(Command::parse(format!("{BASE_COMMAND} 'exit 1'"))
            .unwrap()
            .execute_status()
            .is_err());
    }

    #[test]
    fn test_execute_output() {
        assert!(Command::parse(format!("{BASE_COMMAND} 'exit 0'"))
            .unwrap()
            .execute_output()
            .is_ok());

        assert!(Command::parse(format!("{BASE_COMMAND} 'exit 1'"))
            .unwrap()
            .execute_output()
            .is_err());
    }

    #[test]
    fn test_execute_string() {
        assert_eq!(
            Command::parse(format!("{BASE_COMMAND} 'echo 1'"))
                .unwrap()
                .execute_string()
                .unwrap(),
            if cfg!(windows) { "1\r\n" } else { "1\n" }
        );

        assert!(Command::parse(format!("{BASE_COMMAND} 'exit 1'"))
            .unwrap()
            .execute_string()
            .is_err());
    }
}
