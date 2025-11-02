#[cfg(windows)]
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use windows::Win32::System::Console::{
    CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle,
    STD_OUTPUT_HANDLE, SetConsoleCP, SetConsoleMode, SetConsoleOutputCP,
};

/// Enable UTF-8 for Windows.
///
/// On non-Windows platforms, this does nothing and always returns `Ok(())`.
pub fn enable_utf8() -> std::io::Result<()> {
    #[cfg(windows)]
    {
        unsafe {
            // Set console code page to UTF-8
            SetConsoleCP(65001)?;
            SetConsoleOutputCP(65001)?;

            // Get standard output handle
            let h_out = GetStdHandle(STD_OUTPUT_HANDLE)?;
            if h_out == INVALID_HANDLE_VALUE {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid handle value",
                ));
            }

            // Read current mode
            let mut mode = CONSOLE_MODE(0);
            GetConsoleMode(h_out, &mut mode)?;

            // Add UTF-8 / VT processing flag
            let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            // Apply new mode
            SetConsoleMode(h_out, new_mode)?;
        }
    }

    // On all platforms, return success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_utf8_returns_ok() {
        let result = enable_utf8();
        assert!(
            result.is_ok(),
            "enable_utf8() should always return Ok on all platforms"
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_enable_utf8_on_windows() {
        // Call twice to ensure idempotency
        let result1 = enable_utf8();
        let result2 = enable_utf8();

        assert!(result1.is_ok(), "First call failed: {:?}", result1);
        assert!(result2.is_ok(), "Second call failed: {:?}", result2);
    }

    #[cfg(not(windows))]
    #[test]
    fn test_enable_utf8_on_non_windows() {
        // Should compile and run fine on Linux/Mac
        let result = enable_utf8();
        assert!(result.is_ok());
    }
}
