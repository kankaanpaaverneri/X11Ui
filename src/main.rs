use x11ui::{
    Elm,
    WidgetContainer,
    Color,
    ContainerType
};
use std::error::Error;

enum UserMessage {
    Increment,
    Decrement,
    ChangeLabel(String)
}

struct Application {
    data: i32,
    label: String
}

impl Default for Application {
    fn default() -> Self {
        Self {
            data: 10,
            label: String::new()
        }
    }
}

const X: i16 = 100;
const Y: i16 = 100;

impl Elm for Application {
    type Message = UserMessage;
    fn view(&self) -> WidgetContainer<Self::Message> {
        let mut root_container = WidgetContainer::new(X, Y, 100, ContainerType::Vertical);
        root_container.create_button("+", 50, UserMessage::Increment);
        root_container.create_button("-", 50, UserMessage::Decrement);
        root_container.create_button(&self.data.to_string(), 50, UserMessage::ChangeLabel(String::from("Button clicked")));
        let child1 = root_container.create_container(200, ContainerType::Horizontal);
        child1.create_button(&self.label, 20, UserMessage::Increment);
        child1.create_button(&self.label, 20, UserMessage::Increment);

        root_container 
    }
    fn update(&mut self, message: &Self::Message) {
        match message {
            UserMessage::Increment => self.data += 1,
            UserMessage::Decrement => self.data -= 1,
            UserMessage::ChangeLabel(new_label) => self.label = new_label.to_string()
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    x11ui::init("X11 Ui", 800, 600, Color::Light, Application::default())?;
    Ok(())
}
