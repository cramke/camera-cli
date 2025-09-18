#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_help_output() {
        let output = Command::new(env!("CARGO_BIN_EXE_camera-cli"))
            .arg("--help")
            .output()
            .expect("Failed to run camera-cli");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Camera CLI tool"));
    }
}
