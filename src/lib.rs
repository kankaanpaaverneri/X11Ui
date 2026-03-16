use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _; 
use std::error::Error;

pub struct XWindow<C: Connection> {
    connection: C,
    screen_number: usize,
    window_id: Window,
}

impl<C: Connection> XWindow<C>
{
    fn new(
        connection: C,
        screen_number: usize,
        window_id: Window,
    ) -> Self {
        Self {
            connection,
            screen_number,
            window_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContainerType {
    Vertical,
    Horizontal,
}

pub struct Button<Message> {
    text: String,
    points: [Point; 5],
    message: Message
}

impl<Message> Button<Message> {
    pub fn is_button_interacted(&self, event_x: i16, event_y: i16) -> bool {
        if event_x >= self.points[0].x &&
            event_y <= self.points[0].y &&
            event_x <= self.points[2].x &&
            event_y >= self.points[2].y
        {
           return true; 
        }
        return false;
    }
}

pub struct Label {
    text: String,
    x: i16,
    y: i16,
}

impl Label {
    pub fn new(text: &str, x: i16, y: i16) -> Self {
        Self { text: String::from(text), x, y }
    }
}

pub struct WidgetContainer<Message> {
    buttons: Vec<Button<Message>>,
    labels: Vec<Label>,
    containers: Vec<WidgetContainer<Message>>,
    x: i16,
    y: i16,
    widget_spacing: i16,
    container_type: ContainerType
}

impl<Message> WidgetContainer<Message> {
    pub fn new(x: i16, y: i16, widget_spacing: u16, container_type: ContainerType) -> Self {
        Self {
            buttons: Vec::new(),
            labels: Vec::new(),
            containers: Vec::new(),
            x,
            y,
            widget_spacing: widget_spacing as i16,
            container_type,
        }
    }

    fn get_widget_count(&self) -> i16 {
        (self.buttons.len() + self.labels.len()) as i16
    }

    pub fn create_container(
        &mut self,
        widget_spacing: u16,
        container_type: ContainerType
    ) -> &mut WidgetContainer<Message> {
        let mut new_x = self.x;
        let mut new_y = self.y;
        if let Some(container) = self.containers.iter().last() {
            new_x = container.x;
            new_y = container.y;
            let mut widget_count = container.get_widget_count();
            if widget_count == 0 {
                widget_count = 1;
            }
            match self.container_type {
                ContainerType::Vertical => {
                    new_y += container.widget_spacing * widget_count;
                }
                ContainerType::Horizontal => {
                    new_x += container.widget_spacing * widget_count;
                }
            }
        } else {
            let mut widget_count = self.get_widget_count();
            if widget_count == 0 {
                widget_count = 1;
            }
            match self.container_type {
                ContainerType::Vertical => {
                    new_y += widget_count * self.widget_spacing;
                }
                ContainerType::Horizontal => {
                    new_x += widget_count * self.widget_spacing;
                }
            }
        }
        
        let new_container = WidgetContainer::new(new_x, new_y, widget_spacing, container_type);
        self.containers.push(new_container);
        self.containers.iter_mut().last().unwrap()
    }

    pub fn create_button(&mut self, text: &str, padding: u16, message: Message) {
        let padding: i16 = padding as i16;
        let width = 7 * text.len() as i16 + padding;
        let height = width / 2; 
        let mut x = self.x.clone();
        let mut y = self.y.clone();

        // Get previous button position on screen
        if let Some(previous_button) = self.buttons.iter().last() {
            let prev_button_x = previous_button.points[0].x;
            let prev_button_y = previous_button.points[0].y;
            x = prev_button_x;
            y = prev_button_y;

            match self.container_type {
                ContainerType::Vertical => {
                    y += self.widget_spacing;
                }
                ContainerType::Horizontal => {
                    x += self.widget_spacing;
                }
            }
        }

        // Get previous label position on screen
        if let Some(previous_label) = self.labels.iter().last() {
            let prev_label_x = previous_label.x;
            let prev_label_y = previous_label.y;
            x = prev_label_x;
            y = prev_label_y;

            match self.container_type {
                ContainerType::Vertical => {
                    y += self.widget_spacing;
                }
                ContainerType::Horizontal => {
                    x += self.widget_spacing;
                }
            }
        }

        let points = [
            Point {x: x, y},
            Point {x: x + width, y},
            Point {x: x + width, y: y - height},
            Point {x, y: y - height},
            Point {x, y}
        ];
        let new_button = Button {
           text: String::from(text),
           points, 
           message
        };
        self.buttons.push(new_button);
    }

    pub fn create_label(&mut self, text: &str) {
        // Get previous button position on screen
        let mut x = self.x;
        let mut y = self.y;
        // Get previous button position on screen
        if let Some(previous_button) = self.buttons.iter().last() {
            let prev_button_x = previous_button.points[0].x;
            let prev_button_y = previous_button.points[0].y;
            x = prev_button_x;
            y = prev_button_y;

            match self.container_type {
                ContainerType::Vertical => {
                    y += self.widget_spacing;
                }
                ContainerType::Horizontal => {
                    x += self.widget_spacing;
                }
            }
        }

        // Get previous label position on screen
        if let Some(previous_label) = self.labels.iter().last() {
            let prev_label_x = previous_label.x;
            let prev_label_y = previous_label.y;
            x = prev_label_x;
            y = prev_label_y;

            match self.container_type {
                ContainerType::Vertical => {
                    y += self.widget_spacing;
                }
                ContainerType::Horizontal => {
                    x += self.widget_spacing;
                }
            }
        }
        let new_label = Label::new(text, x, y);
        self.labels.push(new_label);
    }

    pub fn is_widget_interacted(&self, event_x: i16, event_y: i16) -> Option<&Message> {
        for container in &self.containers {
            if let Some(message) = container.is_widget_interacted(event_x, event_y) {
                return Some(message);
            }
        }
        for button in &self.buttons {
            if button.is_button_interacted(event_x, event_y) {
                return Some(&button.message);
            }
        }

        return None;
    }

}

pub trait Elm {
    type Message;
    fn view(&self) -> WidgetContainer<Self::Message>;
    fn update(&mut self, message: &Self::Message);
}

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
    }
}

pub enum Color {
    Dark,
    Light
}


pub fn init<Application: Elm>(
    title: &str,
    width: u16,
    height: u16,
    background_color: Color,
    mut application: Application 
) -> Result<(), Box<dyn Error>> {
    let (connection, screen_number) = x11rb::connect(None)?;
    let window_id = connection.generate_id()?;
    let pixel_color = match background_color {
        Color::Light => connection.setup().roots[screen_number].white_pixel,
        Color::Dark => connection.setup().roots[screen_number].black_pixel
    };
    let window_aux = CreateWindowAux::new()
        .event_mask(
            EventMask::EXPOSURE |
            EventMask::NO_EVENT |
            EventMask::BUTTON_PRESS |
            EventMask::KEY_PRESS |
            EventMask::POINTER_MOTION
        )
        .background_pixel(pixel_color);

    // Changing window title and enabling window close
    let atoms = Atoms::new(&connection)?.reply()?;
    connection.create_window(
        connection.setup().roots[screen_number].root_depth,
        window_id,
        connection.setup().roots[screen_number].root,
        0,
        0,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &window_aux
    )?;
    connection.change_property8(
        PropMode::REPLACE,
        window_id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes()
    )?;

    connection.change_property32(
        PropMode::REPLACE,
        window_id,
        atoms.WM_PROTOCOLS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW]
    )?;


    connection.map_window(window_id)?;
    connection.flush()?;
    let mut window = XWindow::new(
        connection,
        screen_number,
        window_id,
    );
    let mut container = application.view();
    // Main event loop
    loop {
        let event = window.connection.wait_for_event()?;
        match event {
            Event::Expose(_) => {
            }
            Event::KeyPress(_) => {
            }
            Event::ButtonPress(event) => {
                match event.detail {
                    1 => {
                        if let Some(message) = container.is_widget_interacted(event.event_x, event.event_y) {
                            application.update(message);
                        }
                        
                    }
                    3 => {
                        println!("Mouse right click");
                    }
                    4 => {
                        println!("Scroll up");
                    }
                    5 => {
                        println!("Scroll down");
                    }
                    _ => {}
                }
            }
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32 && event.window == window_id && data[0] == atoms.WM_DELETE_WINDOW {
                    return Ok(());
                }
            }
            _ => {}
        }
        // Do updated rendering here
        container = application.view();
        draw_widgets(&mut window, &container)?;
        window.connection.flush()?;
    }
}

fn draw_widgets<C: Connection, Message>(
    window: &mut XWindow<C>,
    parent_container: &WidgetContainer<Message>
) -> Result<(), Box<dyn Error>> {
    for container in &parent_container.containers {
        draw_widgets(window, container)?;
    }
    let buttons = &parent_container.buttons;
    
    for button in buttons {
        let gc = new_gc(window, window.screen_number, Color::Dark, Color::Dark)?;
        window.connection.poly_line(CoordMode::ORIGIN, window.window_id, gc, &button.points)?;
        window.connection.fill_poly(window.window_id, gc, PolyShape::CONVEX, CoordMode::ORIGIN, &button.points)?;
        let gc = new_gc(window, window.screen_number, Color::Light, Color::Dark)?;
        window.connection.image_text8(
            window.window_id,
            gc,
            button.points[0].x + (button.points[2].x - button.points[0].x) / 2 - button.text.len() as i16 * 3,
            button.points[0].y - (button.points[0].y - button.points[2].y) / 2 + 3,  
            button.text.as_bytes()
        )?;
    }
    let labels = &parent_container.labels;

    for label in labels {
        let gc = new_gc(window, window.screen_number, Color::Dark, Color::Light)?;
        window.connection.image_text8(window.window_id, gc, label.x, label.y, label.text.as_bytes())?;

    }
    Ok(())
}


pub fn new_gc<C: Connection>(
    window: &XWindow<C>,
    screen_number: usize,
    foreground: Color,
    background: Color
) -> Result<Gcontext, Box<dyn Error>> {
    let foreground_color = match foreground {
        Color::Light => window.connection.setup().roots[screen_number].white_pixel,
        Color::Dark => window.connection.setup().roots[screen_number].black_pixel
    };
    let background_color = match background {
        Color::Light => window.connection.setup().roots[screen_number].white_pixel,
        Color::Dark => window.connection.setup().roots[screen_number].black_pixel
    };

    let gc_id = window.connection.generate_id()?;
    let font_id = window.connection.generate_id()?;
    window.connection.open_font(font_id, b"fixed")?;
    let gc_values = CreateGCAux::new()
        .foreground(foreground_color)
        .background(background_color)
        .font(font_id);
    window.connection.create_gc(gc_id, window.window_id, &gc_values)?;
    window.connection.close_font(font_id)?;
    Ok(gc_id)
}
