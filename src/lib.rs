use std::process::Command;

pub fn parse(command_string: impl AsRef<str>) -> Command {
    let [program, args @ ..] = &command_string.as_ref().split_whitespace().collect::<Vec<_>>()[..] else {
        return Command::new("");
    };

    let mut command = Command::new(program);
    command.args(args);
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let command = parse("program arg1 arg2");

        assert_eq!(command.get_program(), "program");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["arg1", "arg2"]);
    }
}
