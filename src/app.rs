use colored::Colorize;
use crate::lib::{Gui, TermState};

impl Gui for TermState {
    fn draw(&self) {
        println!("{}", "asd".red());
        //widgets
        self.ui(&self.device, |ui| {
            if ui.button("Test_button", true).take_action {
                println!("asd");
            };
            ui.time_label();
        });
        
    }
}
