use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub eff: bool,
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..))]
    pub num_of_pass: u8,
    #[arg(short, long, default_value_t = 5, value_parser = clap::value_parser!(u8).range(1..))]
    pub word_count: u8,
    #[arg(short = 'c', long, default_value_if("quality", "true", Some("true")))]
    pub use_capital_char: bool,
    #[arg(short = 's', long, default_value_if("quality", "true", Some("true")))]
    pub use_special_char: bool,
    #[arg(short, long)]
    pub quality: bool,
    #[arg(short = 'S', long)]
    pub separator: Option<char>,
}

pub fn process_command_line() -> Args {
    Args::parse()
}

#[cfg(test)]
mod test {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert()
    }

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

    #[test]
    fn verify_cli_arg_e_defaults_to_false() {
        let value = Args::try_parse_from(["sppg"])
            .expect("this command is supposed to work")
            .eff;

        assert!(!value, "default -e value is false");
    }

    #[test]
    fn verify_cli_arg_e_is_true() {
        let value = Args::try_parse_from(["sppg", "--eff"])
            .expect("this command is supposed to work")
            .eff;

        assert!(value, "Arg -e is set to true");
    }

    #[test]
    fn verify_cli_arg_s_is_true01() {
        let value = Args::try_parse_from(["sppg", "--use-special-char"])
            .expect("this command is supposed to work")
            .use_special_char;

        assert!(value, "Arg --use-special-char is set to true");
    }

    #[test]
    fn verify_cli_arg_s_is_true02() {
        let value = Args::try_parse_from(["sppg", "-s"])
            .expect("this command is supposed to work")
            .use_special_char;

        assert!(value, "Arg -s is set to true");
    }

    #[test]
    fn verify_cli_arg_c_is_true01() {
        let value = Args::try_parse_from(["sppg", "--use-capital-char"])
            .expect("this command is supposed to work")
            .use_capital_char;

        assert!(value, "Arg --use-capital-char is set to true");
    }

    #[test]
    fn verify_cli_arg_c_is_true02() {
        let value = Args::try_parse_from(["sppg", "-c"])
            .expect("this command is supposed to work")
            .use_capital_char;

        assert!(value, "Arg -c is set to true");
    }

    #[test]
    fn verify_cli_arg_q_is_true01() {
        let value = Args::try_parse_from(["sppg", "--quality"])
            .expect("this command is supposed to work")
            .quality;

        assert!(value, "Arg --quality is set to true");
    }

    #[test]
    fn verify_cli_arg_q_is_true02() {
        let value = Args::try_parse_from(["sppg", "-q"])
            .expect("this command is supposed to work")
            .quality;

        assert!(value, "Arg -q is set to true");
    }

    #[test]
    fn verify_cli_arg_q_implies_c_s() {
        let value_cap = Args::try_parse_from(["sppg", "-q"])
            .expect("this command is supposed to work")
            .use_capital_char;
        let value_special = Args::try_parse_from(["sppg", "-q"])
            .expect("this command is supposed to work")
            .use_special_char;

        assert!(value_cap, "Arg -q implies -c");
        assert!(value_special, "Arg -q implies -s");
    }

    #[test]
    fn verify_cli_arg_separator_is_true02() {
        let value = Args::try_parse_from(["sppg", "-S", "-"])
            .expect("failed to parse -S argument")
            .separator;

        assert_eq!(value, Some('-'), "Arg -S is set to ' '");
    }

    #[test]
    fn verify_cli_arg_separator_is_true01() {
        let value = Args::try_parse_from(["sppg", "--separator", "-"])
            .expect("failed to parse --separater argument")
            .separator;

        assert_eq!(value, Some('-'), "Arg --separator is set to ' '");
    }
}
