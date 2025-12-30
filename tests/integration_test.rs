// Integration tests for the complete flow
// Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2

use std::net::TcpListener;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn test_complete_flow_with_free_port() {
    // Test with a high port number that's likely free
    let port = 54321;

    // Run the evict command
    let output = Command::new("cargo")
        .args(&["run", "--", &port.to_string()])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should indicate port is not in use
    assert!(
        stdout.contains("not in use") || stderr.contains("not in use"),
        "Expected 'not in use' message for free port. stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_no_arguments_shows_usage() {
    // Run the evict command without arguments
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show usage instructions
    assert!(
        stderr.contains("Usage:") || stderr.contains("usage"),
        "Expected usage message when no arguments provided. stderr: {}",
        stderr
    );

    // Should exit with error code
    assert!(
        !output.status.success(),
        "Should exit with error when no arguments provided"
    );
}

#[test]
fn test_invalid_port_shows_error() {
    // Test with invalid port (0)
    let output = Command::new("cargo")
        .args(&["run", "--", "0"])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show error message
    assert!(
        stderr.contains("Error:") || stderr.contains("error"),
        "Expected error message for invalid port. stderr: {}",
        stderr
    );

    // Should exit with error code
    assert!(
        !output.status.success(),
        "Should exit with error for invalid port"
    );
}

#[test]
fn test_non_numeric_port_shows_error() {
    // Test with non-numeric port
    let output = Command::new("cargo")
        .args(&["run", "--", "abc"])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show error message
    assert!(
        stderr.contains("Error:") || stderr.contains("error") || stderr.contains("Invalid"),
        "Expected error message for non-numeric port. stderr: {}",
        stderr
    );

    // Should exit with error code
    assert!(
        !output.status.success(),
        "Should exit with error for non-numeric port"
    );
}

#[test]
#[ignore] // This test requires creating a test process and may need admin privileges
fn test_complete_flow_with_test_process() {
    // This test would:
    // 1. Start a test server on a specific port
    // 2. Run evict on that port
    // 3. Verify the process was terminated
    // 4. Verify the port is now free

    // Create a test server on a specific port
    let port = 58888;

    // Spawn a thread that listens on the port
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind test server");

    // Give it a moment to bind
    thread::sleep(Duration::from_millis(100));

    // Run evict on that port
    let output = Command::new("cargo")
        .args(&["run", "--", &port.to_string()])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show process info and success message
    assert!(
        stdout.contains("PID:") || stdout.contains("process"),
        "Expected process information. stdout: {}",
        stdout
    );

    // Clean up
    drop(listener);
}
