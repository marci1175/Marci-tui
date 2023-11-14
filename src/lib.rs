mod win;
mod app;

pub mod lib {

    use std::fmt::Display;
    use crate::win::windows_bindings::win_messagebox;
    use chrono::{DateTime, Local};
    use crossterm::{style::Stylize, terminal::{Clear, ClearType}};
    use device_query::{DeviceState, Keycode};

    #[derive(Clone, Copy, Debug)]
    pub struct Keycodes {
        pub left: bool,
        pub right: bool,
        pub up: bool,
        pub down: bool,
        pub enter: bool
    }

    #[derive(Debug)]
    pub struct KeyIterator {
        keycodes: Keycodes,
        index: usize,
    }

    impl Keycodes {
        pub fn iter(&self) -> KeyIterator {
            KeyIterator {
                keycodes: self.clone(),
                index: 0,
            }
        }
    }

    impl Iterator for KeyIterator {
        type Item = bool;
    
        fn next(&mut self) -> Option<Self::Item> {
            let result = match self.index {
                0 => Some(self.keycodes.left),
                1 => Some(self.keycodes.right),
                2 => Some(self.keycodes.up),
                3 => Some(self.keycodes.down),
                4 => Some(self.keycodes.enter),
                _ => None,
            };
    
            self.index += 1;
            result
        }
    }

    pub trait Keymap {
        
        fn check_for_input(device: DeviceState) -> Keycodes {

            fn device_query(device: DeviceState, keycode: Keycode) -> bool {
                return device.query_keymap().contains(&keycode);
            }

            return Keycodes {
                left: device_query(device.clone(), Keycode::Left),
                right: device_query(device.clone(), Keycode::Right),
                up: device_query(device.clone(), Keycode::Up),
                down: device_query(device.clone(), Keycode::Down),
                enter: device_query(device.clone(), Keycode::Enter),
            };
        }
    }

    impl Keymap for TermState {}

    pub struct Response {
        pub take_action: bool,
    }

    impl Response {
        pub fn listen_for_action(&self) -> Keycodes {
            TermState::check_for_input(DeviceState::new())
        }
    }

    pub struct Widget;

    impl Widget {
        pub fn new() -> Self {
            Widget
        }

        pub fn button<T>(&self, text: T, should_be_highlighted: bool) -> Response
        where
            T: Display + Clone + Stylize, // Ensure T implements the required traits
            <T as Stylize>::Styled: Display,
        {
            //button behavior
            println!("{}", text);
            if should_be_highlighted {
                println!("{}", text.clone().on_red());
            }
            Response {
                take_action: TermState::check_for_input(DeviceState::new()).enter,
            }
        }

        pub fn time_widget(&self) {
            let now: DateTime<Local> = Local::now();
            println!("{:?}", now);
        }

        pub fn label<T>(&self, text: T)
        where
            T: ToString + Display + Stylize,
            <T as Stylize>::Styled: Display,
        {
            //label behavior
            println!("{}", text);
        }
    
        pub fn win_messagebox(&self, lptext: impl ToString, lpcaption : impl ToString, icon : u32) {
            win_messagebox(lptext, lpcaption, icon);
        }
    }

    #[derive(Clone, Debug)]
    pub struct TermSettings {
        pub window_name: String,
        pub constantly_update: bool,
    }  

    #[derive(Debug, Clone)]
    pub struct TermProps {
        pub window_size: String,
    }   

    //gui struct
    #[derive(Clone, Debug)]
    pub struct TermState {
        pub props: TermProps,
        pub settings: TermSettings,

        pub let_button: bool,
        //navigation
        pub current_index: i64,

        pub device: DeviceState,
    }

    impl Default for TermState {
        fn default() -> Self {
            Self {
                props: TermProps {
                    window_size: "window_size().unwrap(),".into(),
                },

                settings: TermSettings {
                    window_name: "marci_tui test".into(),
                    constantly_update: false,
                },

                let_button: false,
                current_index: 0,
                device: DeviceState::new(),
            }
        }
    }

    pub trait Gui {
        fn draw(&mut self) {}
    }
    pub trait Screen {
        fn state(&mut self) {}
    }
    impl Screen for TermState {
        fn state(&mut self) {
            //draw first image
            self.draw();
            loop {

                //win bindings



                //controls
                let controls = TermState::check_for_input(self.device.to_owned());
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
    
                if !controls.iter().all(|f| f == false) || self.settings.constantly_update {
                    if !let_button_clone {
                        //Redraw screen
                        crossterm::execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
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
        pub fn ui<R, F>(&self, _device: &DeviceState, widgets: F)
        where
            F: FnOnce(&mut Widget) -> R,
        {
            let mut widget = Widget::new();
            widgets(&mut widget);
        }
    }
}
