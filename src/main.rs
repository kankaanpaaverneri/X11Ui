use x11ui::{
    Elm,
    WidgetContainer,
    ContainerType,
    Color
};
use std::error::Error;

enum UserMessage {
    Clear,
    Precent,
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    Divide,
    Multiply,
    Minus,
    Plus,
    Equals,
    Zero,
    Dot,
    ChangeLayout,
    ChangeText(String),
    AddK,
}

struct Application {
    data: i32,
    label: String,
    change: bool
}

impl Default for Application {
    fn default() -> Self {
        Self {
            data: 10,
            label: String::from(""),
            change: false
        }
    }
}

const X: i16 = 100;
const Y: i16 = 100;

fn example1() -> WidgetContainer<UserMessage> {
    let mut root_container = WidgetContainer::new(X, Y, 60, 10, ContainerType::Horizontal, Color::Cyan);
    root_container.create_button("Root", 10, 5, UserMessage::Precent);
    root_container.create_button("Root", 10, 5, UserMessage::Precent);
    root_container.create_button("Root", 10, 5, UserMessage::Precent);
    let child = root_container.create_container(60, 60, ContainerType::Vertical);
    child.create_button("Child 1", 5, 5, UserMessage::Seven);
    child.create_button("Child 1", 5, 5, UserMessage::Seven);
    child.create_label("Child label");
    child.create_button("Child 1", 5, 5, UserMessage::Seven);
    root_container.create_button("Root2", 10, 5, UserMessage::Precent);
    root_container.create_button("Root2", 10, 5, UserMessage::Precent);
    root_container.create_button("Root2", 10, 5, UserMessage::Precent);
    root_container.create_label("HelloLabel1");
    root_container.create_label("HelloLabel2");
    root_container.create_label("HelloLabel3");
    root_container 
}

fn example2() -> WidgetContainer<UserMessage> {
    let mut root_container = WidgetContainer::new(X, Y, 20, 60, ContainerType::Horizontal, Color::White);
    root_container.create_button("Root", 10, 20, UserMessage::Precent);
    root_container.create_button("Root", 10, 20, UserMessage::Precent);
    root_container.create_button("Root", 10, 20, UserMessage::Precent);
    root_container.create_button("Root", 10, 20, UserMessage::Precent);
        let child1 = root_container.create_container(20, 60, ContainerType::Vertical);
        child1.create_button("Child1", 10, 20, UserMessage::Six);
        child1.create_button("Child1", 10, 20, UserMessage::Six);
        child1.create_button("Child1", 10, 20, UserMessage::Six);
    let child2 = root_container.create_container(20, 60, ContainerType::Horizontal);
    child2.create_button("Child2", 10, 20, UserMessage::Six);
    child2.create_button("Child2", 10, 20, UserMessage::Six);
    child2.create_button("Child2", 10, 20, UserMessage::Six);
    root_container
}

fn calculator(app: &Application) -> WidgetContainer<UserMessage> {
    let mut root_container = WidgetContainer::new(X, Y, 10, 50, ContainerType::Vertical, Color::Orange);
    let row0 = root_container.create_container(15, 10, ContainerType::Horizontal);
    row0.create_label(&app.label);
    let buttons_container = root_container.create_container(15, 10, ContainerType::Vertical);
    let row1 = buttons_container.create_container(15, 10, ContainerType::Horizontal);
    row1.create_button("C", 323, 40, UserMessage::Clear)
        .set_background_color(Color::Cyan)
        .set_border(5)
        .set_border_color(Color::Orange)
        .hover();

    row1.create_button("%", 150, 40, UserMessage::Precent)
        .hover()
        .set_background_color(Color::Yellow)
        .set_foreground_color(Color::Green);
    row1.create_button("/", 150, 40, UserMessage::Divide).set_foreground_color(Color::Red).set_border(10).hover();
    let row2 = buttons_container.create_container(15, 10, ContainerType::Horizontal);
    row2.create_button("7", 150, 40, UserMessage::Seven).hover().set_background_color(Color::Purple);
    row2.create_button("8", 150, 40, UserMessage::Eight).hover().set_border(5).set_border_color(Color::Purple);
    row2.create_button("9", 150, 40, UserMessage::Nine).hover();
    row2.create_button("x", 150, 40, UserMessage::Multiply).hover();
    let row3 = buttons_container.create_container(15, 10, ContainerType::Horizontal);
    row3.create_button("4", 150, 40, UserMessage::Four).hover();
    row3.create_button("5", 150, 40, UserMessage::Five).hover();
    row3.create_button("6", 150, 40, UserMessage::Six).hover();
    row3.create_button("-", 150, 40, UserMessage::Minus).hover();
    let row4 = buttons_container.create_container(15, 10, ContainerType::Horizontal);
    row4.create_button("1", 150, 40, UserMessage::One).hover();
    row4.create_button("2", 150, 40, UserMessage::Two).hover();
    row4.create_button("3", 150, 40, UserMessage::Three).hover();
    row4.create_button("+", 150, 40, UserMessage::Plus).hover();
    let row5 = buttons_container.create_container(15, 10, ContainerType::Horizontal);
    row5.create_button("0", 323, 40, UserMessage::Zero).hover();
    row5.create_button(".", 150, 40, UserMessage::Dot).hover();
    row5.create_button("=", 150, 40, UserMessage::Equals)
        .hover()
        .set_background_color(Color::Blue)
        .set_foreground_color(Color::White);
    root_container
}

fn example3() -> WidgetContainer<UserMessage> {
    let mut root_container = WidgetContainer::new(X, Y, 10, 10, ContainerType::Vertical, Color::Green);
    root_container.create_button("Root1", 50, 1, UserMessage::One);
    root_container.create_button("Root2", 50, 1, UserMessage::One).hover();
    let child0 = root_container.create_container(2, 20, ContainerType::Vertical);
        let child1 = root_container.create_container(2, 20, ContainerType::Vertical);
        child1.create_button("Child1", 50, 4, UserMessage::Two)
            .hover()
            .set_foreground_color(Color::Red)
            .set_background_color(Color::Cyan);
        child1.create_button("Child1", 50, 4, UserMessage::Two);
            let child2 = child1.create_container(5, 10, ContainerType::Horizontal).padding(50);
            child2.create_button("Child2", 50, 4, UserMessage::Three);
            child2.create_button("Child2", 50, 4, UserMessage::Three);
                let child3 = child2.create_container(2, 10, ContainerType::Vertical);
                child3.create_button("Child3", 50, 4, UserMessage::Three);
                child3.create_button("Child3", 50, 4, UserMessage::Three);
                child3.create_button("Child3", 50, 4, UserMessage::Three);
                child3.create_label("Child3");
                child3.create_label("Child3");
                child3.create_label("Child3");
                    let child4 = child3.create_container(2, 10, ContainerType::Vertical);
                    child4.create_button("Child4 Is larger", 50, 4, UserMessage::Three);
                    child4.create_label("Child4");
                    child4.create_label("Child4 is this the largest label in child4 container");
                    child4.create_button("Child4", 50, 4, UserMessage::Three);
            child2.create_button("Yet another Child2", 5, 4, UserMessage::Three);
            child2.create_label("Yet another label");
            child2.create_button("And another Child2", 5, 4, UserMessage::Three);
    root_container.create_label("RootLabel");
    root_container.create_button("RootButton", 50, 4, UserMessage::One);
    root_container.create_label("RootLabel2");
    root_container.create_button("RootButton2", 50, 4, UserMessage::One);
    let root_child =  root_container.create_container(2, 20, ContainerType::Vertical);
    root_child.create_button("Root child1", 50, 4, UserMessage::One);
    root_child.create_button("Root child2", 50, 4, UserMessage::One);
    root_child.create_button("Root child3", 50, 4, UserMessage::One);
    root_container
}

fn example4(app: &Application) -> WidgetContainer<UserMessage> {
    let mut root_container = WidgetContainer::new(X, Y, 10, 10, ContainerType::Vertical, Color::White);
    root_container.create_button("Colored", 50, 5, UserMessage::ChangeLayout)
        //.set_background_color(Color::Green)
        //.set_foreground_color(Color::Orange)
        .hover();
    root_container.create_button("Change Layout", 50, 5, UserMessage::One)
        .set_border(5);
    let child1 = root_container.create_container(20, 10, ContainerType::Vertical).padding(20);
    if app.change {
        for i in 1..11 {
            let text = format!("Label {i}");
            child1.create_label(&text);
        }
    }
    root_container
}

impl Elm for Application {
    type Message = UserMessage;
    fn view(&self) -> WidgetContainer<Self::Message> {
        //example1()
        //example2()
        calculator(self)
        //example3()
        //example4(self)
    }
    fn update(&mut self, message: &Self::Message) -> bool {
        match message {
            UserMessage::Clear => {
                self.label.clear();
                return true;
            }
            UserMessage::Precent => self.label.push('%'),
            UserMessage::Seven => self.label.push('7'),
            UserMessage::Eight => self.label.push('8'),
            UserMessage::Nine => self.label.push('9'),
            UserMessage::Four => self.label.push('4'),
            UserMessage::Five => self.label.push('5'), 
            UserMessage::Six => self.label.push('6'), 
            UserMessage::One => self.label.push('1'), 
            UserMessage::Two => self.label.push('2'), 
            UserMessage::Three => self.label.push('3'), 
            UserMessage::Divide => self.label.push('/'), 
            UserMessage::Multiply => self.label.push('*'), 
            UserMessage::Minus => self.label.push('-'), 
            UserMessage::Plus => self.label.push('+'), 
            UserMessage::Equals => self.label.push('='), 
            UserMessage::Zero => self.label.push('0'), 
            UserMessage::Dot => self.label.push('.'), 
            UserMessage::ChangeLayout => {
                self.change = !self.change;
                return true;
            },
            UserMessage::ChangeText(new_text) => {
                self.label = new_text.to_string();
                return true;
            }
            UserMessage::AddK => {
                self.label.push('K');
                return true;
            }

        }
        return false;
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    x11ui::init("X11 Ui", 800, 600, Application::default())?;
    Ok(())
}
