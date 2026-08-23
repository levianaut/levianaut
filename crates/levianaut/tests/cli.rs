use std::net::TcpListener;
use std::process::{Command, Output};

fn levianaut(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_levianaut"))
        .args(args)
        .output()
        .expect("levianaut should run")
}

fn output_text(output: &Output) -> &str {
    std::str::from_utf8(if output.stdout.is_empty() {
        &output.stderr
    } else {
        &output.stdout
    })
    .expect("output should be UTF-8")
}

#[test]
fn help_displays_cli_help() {
    let output = levianaut(&["--help"]);

    assert!(output.status.success());
    let stdout = output_text(&output);
    assert!(stdout.contains("Usage: levianaut <COMMAND>"));
    assert!(stdout.contains("server"));
}

#[test]
fn version_displays_current_version() {
    let output = levianaut(&["--version"]);

    assert!(output.status.success());
    assert_eq!(
        output_text(&output).trim(),
        format!("levianaut {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn server_help_displays_subcommand_help() {
    let output = levianaut(&["server", "--help"]);

    assert!(output.status.success());
    let stdout = output_text(&output);
    assert!(stdout.contains("Usage: levianaut server [OPTIONS]"));
    assert!(stdout.contains("--addr"));
}

#[test]
fn server_reports_unusable_address_with_its_cause() {
    // Occupy a port so that the server is guaranteed to fail to bind to it.
    let occupied = TcpListener::bind("127.0.0.1:0").expect("a free port should be available");
    let addr = occupied
        .local_addr()
        .expect("listener should have an address");

    let output = levianaut(&["server", "--addr", &addr.to_string()]);

    assert!(!output.status.success());
    let stderr = std::str::from_utf8(&output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains(&format!("levianaut: could not listen on {addr}")));
    assert!(stderr.contains("caused by:"));
}

#[test]
fn no_command_displays_help() {
    let output = levianaut(&[]);

    assert!(!output.status.success());
    let stdout = output_text(&output);
    assert!(stdout.contains("Usage: levianaut <COMMAND>"));
    assert!(stdout.contains("server"));
}
