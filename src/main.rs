use crossterm::terminal::{Clear, ClearType};
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
impl Keymap for Widget {}

pub struct Response {
    take_action: bool,
}

impl Response {
    pub fn listen_for_action(&self) -> Vec<bool> {
        self.check_for_input(DeviceState::new())
    }
}

pub struct Widget {}

impl Widget {
    pub fn ui<R>(device : &DeviceState, _widgets: impl FnOnce(&mut Widget) -> R) {}
    fn button(&self, text: impl ToString + std::fmt::Display) {
        println!("{}", text);
    }
    
    fn label(&self, text: impl ToString + std::fmt::Display) {
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
    fn action(&mut self) {}
    fn draw(&self) {}
    fn state(&mut self) {}
}

impl Gui for TermState {
    fn action(&mut self) {}
    fn draw(&self) {
        //clear screen
        crossterm::execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();

        //print screen
        //widgets
        Widget::ui(&self.device, |ui| {
            ui.label("text");
        });
        println!("Ez hülye");
    }
    fn state(&mut self) {
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
                    self.action();
                }
                if controls[2] {}

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
}

impl TermState {
    pub fn init() {
        TermState::state(&mut TermState::default());
    }
}

fn main() {
    TermState::init();
}