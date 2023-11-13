use windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONASTERISK;
use crate::lib::{Gui, TermState};

impl Gui for TermState {
    fn draw(&self) {
        //widgets
        self.ui(&self.device, |ui| {
            ui.time_widget();
            ui.label("Hello world".to_string());
            ui.win_messagebox("lptext", "lpcaption", MB_ICONASTERISK)
        });
        
    }
}
