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
        let start_time = std::time::Instant::now();
        loop {
            //controls

            let controls: Vec<bool> = self.check_for_input(self.device.to_owned());
            let let_button_clone = self.let_button.clone();

            //left
            //right
            //up
            //down
            //enter

            if !self.let_button {
                if controls[2] {
                    self.current_index += 1;
                }
                if controls[3] {
                    self.current_index -= 1;
                }
                if controls[4] {}
                if controls[2] {}

                self.let_button = true;
            }

            if !controls.iter().all(|f| *f == false) || start_time.elapsed().as_secs() % 2 == 0 {
                if !let_button_clone {
                    self.draw();
                }
            } else {
                self.let_button = false;
            }
        }
    }
}
