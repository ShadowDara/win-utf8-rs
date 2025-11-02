// Imports are now only made on Windows!

#[cfg(windows)]
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use windows::Win32::System::Console::GetStdHandle;
#[cfg(windows)]
use windows::Win32::System::Console::STD_OUTPUT_HANDLE;
#[cfg(windows)]
use windows::Win32::System::Console::{
    CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, SetConsoleCP, SetConsoleMode,
    SetConsoleOutputCP,
};
#[cfg(windows)]
use windows::core::Result;

/// Enable UTF-8 for windows
/// Function does not nothing on other plattforms except windows
pub fn enable_utf8() -> Result<()> {
    // specifies Code for Windows
    #[cfg(windows)]
    {
        unsafe {
            // UTF-8 Codepage setzen
            SetConsoleCP(65001)?;
            SetConsoleOutputCP(65001)?;

            // Handle holen (Result<HANDLE, Error>)
            let h_out = GetStdHandle(STD_OUTPUT_HANDLE)?;
            if h_out == INVALID_HANDLE_VALUE {
                return Ok(());
            }

            // mode als CONSOLE_MODE initialisieren
            let mut mode = CONSOLE_MODE(0);
            GetConsoleMode(h_out, &mut mode)?;

            // Flag hinzufügen
            let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            // Set the Console Output to the new handle
            SetConsoleMode(h_out, new_mode)?;
        }
    }

    // Returning Ok on all plattforms
    Ok(())
}
