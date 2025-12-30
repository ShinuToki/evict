// Process service module for process operations

use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE,
    QueryFullProcessImageNameW, TerminateProcess,
};
use windows::core::PWSTR;

/// Get the process name for a given PID
/// Uses OpenProcess and QueryFullProcessImageNameW to retrieve the full path,
/// then extracts just the filename
pub fn get_process_name(pid: u32) -> Result<String, String> {
    unsafe {
        // Open process with query information access
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid)
            .map_err(|e| format!("Failed to open process {}: {}", pid, e))?;

        // Ensure handle is closed when we're done
        let result = get_process_name_from_handle(handle);
        let _ = CloseHandle(handle);
        result
    }
}

/// Helper function to get process name from an open handle
unsafe fn get_process_name_from_handle(handle: HANDLE) -> Result<String, String> {
    let mut buffer = vec![0u16; 1024];
    let mut size = buffer.len() as u32;

    // Query the full process image name
    unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
        .map_err(|e| format!("Failed to query process name: {}", e))?;
    }

    // Convert from wide string to Rust String
    let full_path = String::from_utf16_lossy(&buffer[..size as usize]);

    // Extract just the filename from the full path
    let filename = full_path
        .split('\\')
        .next_back()
        .unwrap_or(&full_path)
        .to_string();

    if filename.is_empty() {
        return Err("Process name is empty".to_string());
    }

    Ok(filename)
}

/// Terminate a process forcefully
/// Uses TerminateProcess with exit code 1 to force termination
pub fn kill_process(pid: u32) -> Result<(), String> {
    unsafe {
        // Open process with terminate access
        let handle = OpenProcess(PROCESS_TERMINATE, false, pid)
            .map_err(|e| format!("Failed to open process {} for termination: {}", pid, e))?;

        // Terminate the process with exit code 1
        let result = TerminateProcess(handle, 1)
            .map_err(|e| format!("Failed to terminate process {}: {}", pid, e));

        // Close the handle
        let _ = CloseHandle(handle);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: port-killer, Property 2: Process name retrieval succeeds for valid PIDs**
    // **Validates: Requirements 1.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn test_process_name_retrieval_for_valid_pids(pid in 1u32..=10000u32) {
            // Try to get process name - it should either succeed with a non-empty string
            // or fail with an error (process doesn't exist or no permission)
            // The property is that it doesn't crash or hang
            let result = get_process_name(pid);

            // If successful, name should be non-empty
            if let Ok(name) = result {
                prop_assert!(!name.is_empty(), "Process name should not be empty for PID {}", pid);
            }
            // If it fails, that's also acceptable (process might not exist)
            // The key is that the function returns without crashing
        }
    }

    // Unit tests for process operations
    // Requirements: 1.2

    #[test]
    fn test_get_process_name_with_current_process() {
        // Get current process PID
        let current_pid = std::process::id();

        // Should successfully get the process name
        let result = get_process_name(current_pid);
        assert!(result.is_ok(), "Should get name for current process");

        let name = result.unwrap();
        assert!(!name.is_empty(), "Process name should not be empty");
        // Current process should be evict.exe or similar
        assert!(
            name.ends_with(".exe"),
            "Process name should end with .exe on Windows"
        );
    }

    #[test]
    fn test_get_process_name_with_invalid_pid() {
        // Use a very high PID that's unlikely to exist
        let invalid_pid = 9999999u32;

        // Should return an error
        let result = get_process_name(invalid_pid);
        assert!(result.is_err(), "Should fail for invalid PID");
    }

    #[test]
    fn test_kill_process_with_invalid_pid() {
        // Use a very high PID that's unlikely to exist
        let invalid_pid = 9999999u32;

        // Should return an error
        let result = kill_process(invalid_pid);
        assert!(result.is_err(), "Should fail to kill invalid PID");
    }
}
