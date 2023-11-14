use windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONASTERISK;
use crate::lib::{Gui, TermState};

impl Gui for TermState {
    fn draw(&mut self) {
        //widgets        
        self.ui(&self.device, |ui| {
            ui.time_widget();
            ui.label("Hello world".to_string());
            if ui.button("FASZ", true).take_action {
                
            }
        });
        
    }
}
