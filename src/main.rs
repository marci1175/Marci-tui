use std::time::Duration;

use colored::{ColoredString, Colorize};
use crossterm::terminal::{ClearType, Clear};
use device_query::{device_state, DeviceState, Keycode};
#[derive(Clone, Debug)]
struct Term_State {
    needs_redraw: bool,
    screen_lines: Vec<ColoredString>,
    let_button: bool,
}

impl Default for Term_State {
    fn default() -> Self {
        Self { needs_redraw: true, screen_lines: Vec::new(), let_button: false }
    }
}

pub fn device_query(device: DeviceState, keycode: Keycode) -> bool {
    return device.query_keymap().contains(&keycode);
}

pub fn check_for_input(device: &DeviceState) -> Vec<bool> {
    return
        vec![
        device_query(device.clone(), Keycode::Left),
        device_query(device.clone(), Keycode::Right),
        device_query(device.clone(), Keycode::Up),
        device_query(device.clone(), Keycode::Down),
        device_query(device.clone(), Keycode::Enter),
        ];
}

impl Term_State {
    pub fn draw(&self) {

        crossterm::execute!(std::io::stdout(), Clear(ClearType::Purge)).unwrap();
        
        //print screen
        for item in &self.screen_lines {
            println!("{item}");
        }

    }
    pub fn state(&mut self) {
        let device = DeviceState::new();
        loop {
            //control checks
        
            let controls: Vec<bool> = check_for_input(&device);
            
            //left
            //right
            //up
            //down
            //enter
            if !self.let_button {
                if controls[3] {
                    self.screen_lines.push("Pressed down".red());
                }
                if controls[4] {
                    self.screen_lines.push("asdasd down".red());
                }
                if controls[2] {
                    self.screen_lines.remove(0);
                }
            }
            

            if !controls.iter().all(|f| *f == false) {

                self.draw();

                self.let_button = true;
            }
            else {
                self.let_button = false;
            }

        }
    }
    pub fn init() {
        Term_State::state(&mut Term_State::default());
    }
}

fn main() {
    
   Term_State::init();

}
