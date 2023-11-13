use colored::Colorize;
use crossterm::terminal::{Clear, ClearType};

use crate::lib::{Gui, Keymap, TermState};

impl Gui for TermState {
    fn draw(&self) {
        //clear screen
        crossterm::execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
        println!("{}", "asd".red());
        //widgets
        self.ui(&self.device, |ui| {
            if ui.button("Test_button", true).take_action {
                println!("asd");
            };
        });
    }
    fn state(&mut self) {
        //draw first image
        self.draw();
        loop {
            //controls

            let controls = self.check_for_input(self.device.to_owned());
            let let_button_clone = self.let_button.clone();

            //left
            //right
            //up
            //down
            //enter

            if !self.let_button {
                if controls.up {
                    self.current_index += 1;
                }
                if controls.down {
                    self.current_index -= 1;
                }
                if controls.enter {}

                self.let_button = true;
            }

            if !controls.iter().all(|f| f == false){
                if !let_button_clone {
                    self.draw();
                }
            } else {
                self.let_button = false;
            }
        }
    }
}
