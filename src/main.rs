use std::{fmt::{Debug, Display}, str::FromStr};

use crossterm::{terminal::{Clear, ClearType}, style::Stylize};
use device_query::{DeviceState, Keycode};

pub trait Keymap {
    fn device_query(device: DeviceState, keycode: Keycode) -> bool {
        return device.query_keymap().contains(&keycode);
    }
    fn check_for_input(&self, device: DeviceState) -> Vec<bool> {
        return vec![
            Self::device_query(device.clone(), Keycode::Left),
            Self::device_query(device.clone(), Keycode::Right),
            Self::device_query(device.clone(), Keycode::Up),
            Self::device_query(device.clone(), Keycode::Down),
            Self::device_query(device.clone(), Keycode::Enter),
        ];
    }
}

impl Keymap for TermState {}
impl Keymap for Response {}


pub struct Response {
    take_action: bool,
}

impl Response {
    pub fn listen_for_action(&self) -> Vec<bool> {
        self.check_for_input(DeviceState::new())
    }
}

pub struct Widget;

impl Widget {
    fn new() -> Self {
        Widget
    }

    fn button<T>(&self, text: T, should_be_highlighted : bool) -> Response 
    where
        T: Display + Clone + Stylize, // Ensure T implements the required traits
        <T as Stylize>::Styled: Display,
    {
        println!("{}", text);
        if should_be_highlighted {
            println!("{}", text.clone().on_red());
        }
        Response { take_action: Response::device_query(DeviceState::new(), Keycode::Enter) }
    }
    
    fn label<T>(&self, text: T)
    where
        T: Display + Clone + Stylize, // Ensure T implements the required traits
        <T as Stylize>::Styled: Display,
    {
        println!("{}", text);
    }


}

#[derive(Clone, Debug)]
struct TermState {
    let_button: bool,
    //navigation
    current_index: i64,

    device: DeviceState,
}

impl Default for TermState {
    fn default() -> Self {
        Self {
            let_button: false,
            current_index: 0,
            device: DeviceState::new(),
        }
    }
}

pub trait Gui {
    fn draw(&self) {}
    fn state(&mut self) {}
}

impl Gui for TermState {
    fn draw(&self) {
        //clear screen
        crossterm::execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
        println!("{}", "asd".red());
        //widgets
        self.ui(&self.device, |ui|{
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
                if controls[4] {

                }
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

impl TermState {
    //entrypoint
    pub fn init() {
        TermState::state(&mut TermState::default());
    }
    fn ui<R, F>(&self, device: &DeviceState, widgets: F)
    where
        F: FnOnce(&mut Widget) -> R,
    {
        let mut widget = Widget::new();
        widgets(&mut widget);
    }
}

fn main() {
    TermState::init();
}