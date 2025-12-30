// Validation module for input validation

/// Validates a port string and returns a u16 port number if valid
///
/// Valid ports are in the range 1-65535
/// Returns an error for non-numeric input or out-of-range values
pub fn validate_port(port_str: &str) -> Result<u16, String> {
    // Try to parse as u16
    let port = port_str
        .parse::<u16>()
        .map_err(|_| format!("Port must be a valid number, got: {}", port_str))?;

    // Check range (1-65535)
    if port == 0 {
        return Err("Port must be between 1 and 65535, got: 0".to_string());
    }

    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: port-killer, Property 4: Invalid input validation rejects bad data**
    // **Validates: Requirements 3.1, 3.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn test_invalid_ports_rejected(port in prop::num::u32::ANY.prop_filter("out of range", |&p| p == 0 || p > 65535)) {
            let result = validate_port(&port.to_string());
            prop_assert!(result.is_err(), "Port {} should be rejected but was accepted", port);
        }

        #[test]
        fn test_valid_ports_accepted(port in 1u16..=65535u16) {
            let result = validate_port(&port.to_string());
            prop_assert!(result.is_ok(), "Port {} should be accepted but was rejected", port);
            prop_assert_eq!(result.unwrap(), port);
        }

        #[test]
        fn test_non_numeric_rejected(s in "[a-zA-Z]+") {
            let result = validate_port(&s);
            prop_assert!(result.is_err(), "Non-numeric string '{}' should be rejected", s);
        }
    }

    // Unit tests for edge cases
    // Requirements: 3.1, 3.2

    #[test]
    fn test_port_1_minimum_valid() {
        let result = validate_port("1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_port_65535_maximum_valid() {
        let result = validate_port("65535");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 65535);
    }

    #[test]
    fn test_port_0_invalid() {
        let result = validate_port("0");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be between 1 and 65535"));
    }

    #[test]
    fn test_port_65536_invalid() {
        let result = validate_port("65536");
        assert!(result.is_err());
    }

    #[test]
    fn test_non_numeric_string() {
        let result = validate_port("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a valid number"));
    }

    #[test]
    fn test_empty_string() {
        let result = validate_port("");
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_number() {
        let result = validate_port("-1");
        assert!(result.is_err());
    }
}
