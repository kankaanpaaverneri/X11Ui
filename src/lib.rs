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
    id: usize,
    text: String,
    points: [Point; 5],
    message: Message
}

impl<Message> Button<Message> {
    pub fn is_button_interacted(&self, event_x: i16, event_y: i16) -> bool {
        if event_x >= self.points[0].x &&
            event_y >= self.points[0].y &&
            event_x <= self.points[2].x &&
            event_y <= self.points[2].y
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
    pub widget_count: usize,
    buttons: Vec<Button<Message>>,
    labels: Vec<Label>,
    containers: Vec<WidgetContainer<Message>>,
    x: i16,
    y: i16,
    widget_spacing_x: i16,
    widget_spacing_y: i16,
    container_type: ContainerType
}

impl<Message> WidgetContainer<Message> {
    pub fn new(
        x: i16,
        y: i16,
        widget_spacing_x: u16,
        widget_spacing_y: u16,
        container_type: ContainerType,
        id: usize,
    ) -> Self {
        Self {
            widget_count: id,
            buttons: Vec::new(),
            labels: Vec::new(),
            containers: Vec::new(),
            x,
            y,
            widget_spacing_x: widget_spacing_x as i16,
            widget_spacing_y: widget_spacing_y as i16,
            container_type,
        }
    }

    fn move_point_horizontal(&self, container: &WidgetContainer<Message>) -> i16 {
        let mut x = self.x;
        if let Some(last_container) = container.containers.iter().last() {
            let new_x = container.move_point_horizontal(last_container);
            if new_x > x {
                x = new_x;
            }
        }
        if let Some(last_button) = container.buttons.iter().last() {
            if last_button.id == container.widget_count {
                let button_x = last_button.points[2].x + self.widget_spacing_x;
                if button_x > x {
                    x = button_x;
                }
            }
        }
        x 
    }

    fn move_point_vertical(&self, container: &WidgetContainer<Message>) -> i16 {
        let mut y = self.y;
        if let Some(last_container) = container.containers.iter().last() {
            let new_y = container.move_point_vertical(last_container);
            if new_y > y {
                y = new_y;
            }
        }
        if let Some(last_button) = container.buttons.iter().last() {
            if last_button.id == container.widget_count {
                let button_y = last_button.points[2].y + self.widget_spacing_y;
                if button_y > y {
                    y = button_y;
                }
            }
        }
        y
    }

    pub fn create_container(
        &mut self,
        widget_spacing_x: u16,
        widget_spacing_y: u16,
        new_container_type: ContainerType
    ) -> &mut WidgetContainer<Message> {
        let mut x = self.x;
        let mut y = self.y;
        if let Some(container) = self.containers.iter().last() {
            match self.container_type {
                ContainerType::Vertical => y = self.move_point_vertical(container),
                ContainerType::Horizontal => x = self.move_point_horizontal(container)
            }
        } 
        match self.container_type {
            ContainerType::Vertical => {
                if let Some(button) = self.buttons.iter().last() {
                    if button.id == self.widget_count {
                        y = button.points[2].y + self.widget_spacing_y;
                    }
                }
            }
            ContainerType::Horizontal => {
                if let Some(button) = self.buttons.iter().last() {
                    if button.id == self.widget_count {
                        x = button.points[2].x + self.widget_spacing_x;
                    }
                }
            }
        }
        self.widget_count += 1;
        let mut new_container = WidgetContainer::new(
            x,
            y,
            widget_spacing_x,
            widget_spacing_y,
            new_container_type,
            self.widget_count
        );

        self.containers.push(new_container); 
        self.containers.iter_mut().last().unwrap()
    }

    pub fn create_button(
        &mut self,
        text: &str,
        padding_x: u16,
        padding_y: u16,
        message: Message
    ) {
        let padding_x: i16 = padding_x as i16;
        let padding_y: i16 = padding_y as i16;
        let width = text.len() as i16 * 8 + padding_x;
        let height = padding_y * 2 + 16; 
        let mut x = self.x;
        let mut y = self.y;
        
        if let Some(previous_container) = self.containers.iter().last() {
            match self.container_type {
                ContainerType::Vertical => y = self.move_point_vertical(previous_container),
                ContainerType::Horizontal => x = self.move_point_horizontal(previous_container)
            }
        }
        if let Some(previous_button) = self.buttons.iter().last() {
            if self.widget_count == previous_button.id {
                match self.container_type {
                    ContainerType::Vertical => {
                        y = previous_button.points[2].y + self.widget_spacing_y;
                    }
                    ContainerType::Horizontal => {
                        x = previous_button.points[2].x + self.widget_spacing_x;
                    }
                }
            } 
        }

        let points: [Point; 5] = [
            Point {x, y},
            Point {x: x + width, y},
            Point {x: x + width, y: y + height},
            Point {x, y: y + height},
            Point {x, y},
        ];
        // Update the next button coordinates
        self.widget_count += 1;
        self.buttons.push(Button {
            id: self.widget_count,
            text: text.to_string(),
            points,
            message
        });
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
    
    fn get_widget_count(&self) -> i16 {
        (self.buttons.len() + self.labels.len()) as i16
    }

}

pub trait Elm {
    type Message;
    fn view(&self) -> WidgetContainer<Self::Message>;
    fn update(&mut self, message: &Self::Message) -> bool;
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

struct GraphicalContexts {
    dark_gc: Gcontext,
    foreground_light_gc: Gcontext,
    foreground_dark_gc: Gcontext
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
    let pixmap = window.connection.generate_id()?;
    let mut container = application.view();
    let font_id = window.connection.generate_id()?;
    window.connection.open_font(font_id, b"fixed")?;
    let gc_values = GraphicalContexts {
        dark_gc: new_gc(&window, window.screen_number, font_id, Color::Dark, Color::Dark)?,
        foreground_light_gc: new_gc(&window, window.screen_number, font_id, Color::Light, Color::Dark)?,
        foreground_dark_gc: new_gc(&window, window.screen_number, font_id, Color::Dark, Color::Light)?
    };
    // Main event loop

    loop {
        let event = window.connection.wait_for_event()?;
        let mut redraw = false;
        match event {
            Event::Expose(_) => {
                redraw = true;
            }
            Event::KeyPress(_) => {
            }
            Event::ButtonPress(event) => {
                
                match event.detail {
                    1 => {
                        redraw = true;
                        if let Some(message) = container.is_widget_interacted(event.event_x, event.event_y) {
                            if application.update(message) {
                                window.connection.clear_area(false, window.window_id, 0, 0, width, height)?;
                            }
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
        if redraw {
            container = application.view();
            draw_widgets(&mut window, &gc_values, &container)?;
            window.connection.flush()?;
        }
    }
    window.connection.free_gc(gc_values.dark_gc)?;
    window.connection.free_gc(gc_values.foreground_light_gc)?;
    window.connection.free_gc(gc_values.foreground_dark_gc)?;
    window.connection.close_font(font_id)?;
}


fn draw_widgets<C: Connection, Message>(
    window: &mut XWindow<C>,
    gc_values: &GraphicalContexts,
    parent_container: &WidgetContainer<Message>
) -> Result<(), Box<dyn Error>> {
    for container in &parent_container.containers {
        draw_widgets(window, gc_values, container)?;
    }
    let buttons = &parent_container.buttons;
    
    for button in buttons {
        window.connection.poly_line(CoordMode::ORIGIN, window.window_id, gc_values.dark_gc, &button.points)?;
        window.connection.fill_poly(
            window.window_id,
            gc_values.dark_gc,
            PolyShape::CONVEX,
            CoordMode::ORIGIN,
            &button.points
        )?;
        window.connection.image_text8(
            window.window_id,
            gc_values.foreground_light_gc,
            button.points[0].x + (button.points[2].x - button.points[0].x) / 2 - button.text.len() as i16 * 3,
            button.points[0].y - (button.points[0].y - button.points[2].y) / 2 + 3,  
            button.text.as_bytes()
        )?;
    }
    let labels = &parent_container.labels;

    for label in labels {
        window.connection.image_text8(
            window.window_id,
            gc_values.foreground_light_gc,
            label.x + parent_container.widget_spacing_x,
            label.y - parent_container.widget_spacing_y,
            label.text.as_bytes()
        )?;
    }
    Ok(())
}


pub fn new_gc<C: Connection>(
    window: &XWindow<C>,
    screen_number: usize,
    font_id: Font,
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
    let gc_values = CreateGCAux::new()
        .foreground(foreground_color)
        .background(background_color)
        .font(font_id);
    window.connection.create_gc(gc_id, window.window_id, &gc_values)?;
    Ok(gc_id)
}
