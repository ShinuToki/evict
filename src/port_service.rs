use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, MIB_TCPROW_OWNER_PID, MIB_TCPTABLE_OWNER_PID,
};
use windows::Win32::Networking::WinSock::AF_INET;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortBinding {
    pub pid: u32,
    pub port: u16,
}

/// Find the process ID that is using the specified port
pub fn find_process_by_port(port: u16) -> Result<Option<PortBinding>, String> {
    unsafe {
        // First call to get the required buffer size
        let mut size: u32 = 0;
        let result = GetExtendedTcpTable(
            None,
            &mut size,
            false,
            AF_INET.0 as u32,
            windows::Win32::NetworkManagement::IpHelper::TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result != windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER.0 {
            return Err(format!(
                "Failed to query TCP table size: error code {}",
                result
            ));
        }

        // Allocate buffer and make second call to get actual data
        let mut buffer: Vec<u8> = vec![0; size as usize];
        let result = GetExtendedTcpTable(
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
            false,
            AF_INET.0 as u32,
            windows::Win32::NetworkManagement::IpHelper::TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result != 0 {
            return Err(format!("Failed to get TCP table: error code {}", result));
        }

        // Parse the TCP table
        let table = buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID;
        let num_entries = (*table).dwNumEntries as usize;

        // Get pointer to the first entry
        let entries_ptr = &(*table).table as *const MIB_TCPROW_OWNER_PID;

        // Search for matching port
        for i in 0..num_entries {
            let entry = entries_ptr.add(i);
            let local_port = u16::from_be((*entry).dwLocalPort as u16);

            if local_port == port {
                let pid = (*entry).dwOwningPid;
                return Ok(Some(PortBinding { pid, port }));
            }
        }

        // Port not found
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_binding_struct() {
        let binding = PortBinding {
            pid: 1234,
            port: 8080,
        };
        assert_eq!(binding.pid, 1234);
        assert_eq!(binding.port, 8080);
    }

    #[test]
    fn test_find_process_by_port_returns_result() {
        // Test with a likely free port
        let result = find_process_by_port(54321);
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_free_port() {
        // Test with a high port number that's likely free
        let result = find_process_by_port(63999);
        assert!(result.is_ok());
        // Most likely this port is free
        if let Ok(binding_opt) = result {
            // If it's free, we should get None
            // If it's occupied, we should get Some with valid data
            if let Some(binding) = binding_opt {
                assert!(binding.pid > 0);
                assert_eq!(binding.port, 63999);
            }
        }
    }

    #[test]
    fn test_port_binding_equality() {
        let binding1 = PortBinding {
            pid: 100,
            port: 8080,
        };
        let binding2 = PortBinding {
            pid: 100,
            port: 8080,
        };
        let binding3 = PortBinding {
            pid: 200,
            port: 8080,
        };

        assert_eq!(binding1, binding2);
        assert_ne!(binding1, binding3);
    }

    // Property-based tests
    use proptest::prelude::*;

    proptest! {
        // **Feature: port-killer, Property 1: Port query returns valid result**
        #[test]
        fn prop_port_query_returns_valid_result(port in 1u16..=65535u16) {
            let result = find_process_by_port(port);
            // Should always return Ok with either Some(binding) or None
            prop_assert!(result.is_ok());

            // If we get a binding, verify it has the correct port
            if let Ok(Some(binding)) = result {
                prop_assert_eq!(binding.port, port);
                // PID should be non-zero
                prop_assert!(binding.pid > 0);
            }
        }
    }
}
