use sppg::{cli::process_command_line, iterate, print_passphrases};

fn main() {
    let cli_args = process_command_line();
    let list = iterate(&cli_args);
    print_passphrases(&list);
}
