use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..))]
    pub num_of_pass: u8,
    #[arg(short, long, default_value_t = 5, value_parser = clap::value_parser!(u8).range(1..))]
    pub word_count: u8,
}

pub fn process_command_line() -> Args {
    Args::parse()
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn verify_cli() {
    //     Args::command().debug_assert()
    // }

    #[test]
    fn verify_cli_display_help() {
        assert_eq!(
            Args::try_parse_from(["sppg", "--help",])
                .expect_err("this command is supposed to fail")
                .kind(),
            clap::error::ErrorKind::DisplayHelp,
            "the program displays a usage screen"
        );
    }

    #[test]
    fn verify_cli_arg_n_defaults_to_6() {
        let value = Args::try_parse_from(["sppg"])
            .expect("this command is supposed to work")
            .num_of_pass;

        assert_eq!(value, 6, "default -n value is 6");
    }

    #[test]
    fn verify_cli_arg_n_negative_is_error() {
        assert_eq!(
            Args::try_parse_from(["sppg", "-n", "-1"])
                .expect_err("this command is supposed to fail")
                .kind(),
            clap::error::ErrorKind::UnknownArgument,
            "if -n is negative it returns UnknownArgument error"
        );
    }

    #[test]
    fn verify_cli_arg_n_zero_is_error() {
        assert_eq!(
            Args::try_parse_from(["sppg", "-n", "0"])
                .expect_err("this command is supposed to fail")
                .kind(),
            clap::error::ErrorKind::ValueValidation,
            "if -n is zero the program returns a ValueValidation error"
        );
    }

    #[test]
    fn verify_cli_arg_w_zero_is_error() {
        assert_eq!(
            Args::try_parse_from(["sppg", "-w", "0"])
                .expect_err("this command is supposed to fail")
                .kind(),
            clap::error::ErrorKind::ValueValidation,
            "if -w is zero the program returns a ValueValidation error"
        );
    }

    #[test]
    fn verify_cli_arg_w_defaults_to_5() {
        let value = Args::try_parse_from(["sppg"])
            .expect("this command is supposed to work")
            .word_count;

        assert_eq!(value, 5, "default -w value is 5");
    }
}
