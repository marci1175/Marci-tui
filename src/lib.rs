mod app;

pub mod lib {
    use std::fmt::{Debug, Display};

    use crossterm::style::Stylize;
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
        pub take_action: bool,
    }

    impl Response {
        pub fn listen_for_action(&self) -> Vec<bool> {
            self.check_for_input(DeviceState::new())
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
            println!("{}", text);
            if should_be_highlighted {
                println!("{}", text.clone().on_red());
            }
            Response {
                take_action: Response::device_query(DeviceState::new(), Keycode::Enter),
            }
        }

        pub fn label<T>(&self, text: T)
        where
            T: Display + Clone + Stylize, // Ensure T implements the required traits
            <T as Stylize>::Styled: Display,
        {
            println!("{}", text);
        }
    }

    #[derive(Clone, Debug)]
    pub struct TermState {
        pub let_button: bool,
        //navigation
        pub current_index: i64,

        pub device: DeviceState,
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

    impl TermState {
        //entrypoint
        pub fn init() {
            TermState::state(&mut TermState::default());
        }
        pub fn ui<R, F>(&self, device: &DeviceState, widgets: F)
        where
            F: FnOnce(&mut Widget) -> R,
        {
            let mut widget = Widget::new();
            widgets(&mut widget);
        }
    }
}
