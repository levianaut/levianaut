use std::error::Error;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match levianaut::run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            report(error.as_ref());
            ExitCode::FAILURE
        }
    }
}

/// Prints an error and each of its causes to stderr, one per line.
fn report(error: &dyn Error) {
    eprintln!("levianaut: {error}");

    let mut source = error.source();
    while let Some(cause) = source {
        eprintln!("  caused by: {cause}");
        source = cause.source();
    }
}
