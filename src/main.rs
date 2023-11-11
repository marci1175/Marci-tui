use colored::{ColoredString, Colorize};
use crossterm::terminal::{Clear, ClearType};
use device_query::{DeviceState, Keycode};

impl Widgets {

    pub fn button(text : impl ToString + std::fmt::Display) {
        println!("{text}");
    }

    pub fn label(text : impl ToString + std::fmt::Display) {
        println!("{text}");
    }
}

#[derive(Clone, Debug)]
struct Widgets {}

#[derive(Clone, Debug)]
struct TermState {
    screen_lines: Vec<ColoredString>,
    let_button: bool,
    //navigation
    current_index: usize,
}

impl Default for TermState {
    fn default() -> Self {
        Self {
            screen_lines: Vec::new(),
            let_button: false,
            current_index: 0,
        }
    }
}

pub fn device_query(device: DeviceState, keycode: Keycode) -> bool {
    return device.query_keymap().contains(&keycode);
}

pub fn check_for_input(device: &DeviceState) -> Vec<bool> {
    return vec![
        device_query(device.clone(), Keycode::Left),
        device_query(device.clone(), Keycode::Right),
        device_query(device.clone(), Keycode::Up),
        device_query(device.clone(), Keycode::Down),
        device_query(device.clone(), Keycode::Enter),
    ];
}

impl TermState {
    pub fn action(&mut self) {
        
    }
    pub fn draw(&self) {

        //clear screen
        crossterm::execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();

        //print screen
        //widgets
        Widgets::label("Test".blue());
        
    }
    pub fn state(mut self) {
        let device = DeviceState::new();

        loop {
            //control checks

            let controls: Vec<bool> = check_for_input(&device);

            //left
            //right
            //up
            //down
            //enter
            let let_button_clone = self.let_button.clone();

            if !self.let_button {
                if controls[2] {
                    self.current_index += 1;
                }
                if controls[3] {
                    self.current_index -= 1;
                }
                if controls[4] {
                    self.action();
                }
                if controls[2] {

                }

                self.let_button = true;
            }

            if !controls.iter().all(|f| *f == false) {
                if !let_button_clone {
                    self.draw();
                }
            } else {
                self.let_button = false;
            }
        }
    }
    pub fn init() {
        TermState::state(TermState::default());
    }
}

fn main() {
    TermState::init();
}
