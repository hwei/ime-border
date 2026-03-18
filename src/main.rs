use std::process::ExitCode;

fn main() -> ExitCode {
    match ime_border::cli::run(std::env::args()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::from(1)
        }
    }
}
