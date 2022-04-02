#![windows_subsystem = "windows"]

mod trans;
mod test;

use trans::{Messages, Locale};
use windows::Win32::{
    UI::WindowsAndMessaging::{
        IDNO, IDCANCEL, MessageBoxA, MB_ICONINFORMATION, MB_YESNOCANCEL, MB_ICONWARNING
    }, 
    Foundation::HWND,
    Globalization::GetUserDefaultLocaleName
};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let mut lang = vec![0u16; 8];

    unsafe {
        GetUserDefaultLocaleName(lang.as_mut_slice()); 
    }

    let locale = Locale::from_string(&String::from_utf16(&lang)?);
    let (caption, info) = (
        Messages::InfoTitle.translate(&locale), 
        Messages::InfoText.translate(&locale)
    );

    unsafe {
        let result = MessageBoxA(HWND(0), info, caption, MB_ICONWARNING | MB_YESNOCANCEL);

        match result {
            IDNO | IDCANCEL => {
                let (caption, info) = (
                    Messages::ApologyTitle.translate(&locale),
                    Messages::ApologyText.translate(&locale)
                );

                MessageBoxA(HWND(0), info, caption, MB_ICONINFORMATION);
            }
            _ => {}
        }
    }

    Ok(())
}
