use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _; 
use std::error::Error;

pub struct XWindow<C: Connection> {
    connection: C,
    screen_number: usize,
    window_id: Window,
    font_id: Font,
    background_color: Color,
    foreground_color: Color 
}

impl<C: Connection> XWindow<C>
{
    fn new(
        connection: C,
        screen_number: usize,
        window_id: Window,
        background_color: Color, 
        foreground_color: Color 
    ) -> Self {
        Self {
            connection,
            screen_number,
            window_id,
            font_id: 0,
            background_color,
            foreground_color
        }
    }
}



#[derive(Debug, Clone)]
pub enum ContainerType {
    Vertical,
    Horizontal,
}


pub struct Button<Message> {
    id: u128,
    text: String,
    points: [Point; 5],
    message: Message,
    hover: bool,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    border: u32,
    border_color: Option<Color> 
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

    pub fn hover(&mut self) -> &mut Button<Message> {
        self.hover = true;
        self
    }
    
    pub fn set_background_color(&mut self, color: Color) -> &mut Button<Message> {
        self.background_color = Some(color);
        self
    }

    pub fn set_foreground_color(&mut self, color: Color) -> &mut Button<Message> {
        self.foreground_color = Some(color);
        self
    }
    
    pub fn set_border(&mut self, border_size: u32) -> &mut Button<Message> {
        self.border = border_size;
        self
    }

    pub fn set_border_color(&mut self, border_color: Color) -> &mut Button<Message> {
        self.border_color = Some(border_color);
        self
    }


    fn draw_button<C: Connection>(
        &self,
        window: &XWindow<C>,
        button_background_gc: Gcontext,
        button_foreground_gc: Gcontext,
        border_gc: Option<Gcontext>,
        border: u32
    ) -> Result<(), Box<dyn Error>> {
        if border > 0 {
            if let Some(border_gc) = border_gc {
                window.connection.poly_line(
                    CoordMode::ORIGIN,
                    window.window_id,
                    border_gc,
                    &self.points
                )?;
            }
        } else {
            window.connection.poly_line(
                CoordMode::ORIGIN,
                window.window_id,
                button_background_gc,
                &self.points
            )?;
        }
        window.connection.fill_poly(
            window.window_id,
            button_background_gc,
            PolyShape::CONVEX,
            CoordMode::ORIGIN,
            &self.points
        )?;
        window.connection.image_text8(
            window.window_id,
            button_foreground_gc,
            self.points[0].x + (self.points[2].x - self.points[0].x) / 2 - self.text.len() as i16 * 3,
            self.points[0].y - (self.points[0].y - self.points[2].y) / 2 + 3,  
            self.text.as_bytes()
        )?;
        Ok(())
    }

    fn is_background_color_match(&self, background_color: &Color) -> bool {
        if let Some(button_background_color) = &self.background_color {
            if *background_color == *button_background_color {
                return true;
            }
        }
        false
    }

    fn is_foreground_color_match(&self, foreground_color: &Color) -> bool {
        if let Some(button_foreground_color) = &self.foreground_color {
            if *foreground_color == *button_foreground_color {
                return true;
            }
        }
        false
    }

}

pub struct Label {
    id: u128,
    text: String,
    begin_x: i16,
    begin_y: i16,
    end_x: i16,
    end_y: i16,
    text_color: u32,
}

impl Label {
    pub fn new(id: u128, text: &str, x: i16, y: i16, text_color: u32) -> Self {
        let length = text.len() as i16 * 6;
        let height = 15;
        
        Self {
            id, 
            text: String::from(text),
            begin_x: x,
            begin_y: y,
            end_x: x + length,
            end_y: y + height,
            text_color
        }
    }
}

pub struct WidgetContainer<Message> {
    id: u128,
    widget_count: usize,
    buttons: Vec<Button<Message>>,
    labels: Vec<Label>,
    containers: Vec<WidgetContainer<Message>>,
    x: i16,
    y: i16,
    widget_spacing_x: i16,
    widget_spacing_y: i16,
    container_type: ContainerType,
    background_color: Color
}

impl<Message> WidgetContainer<Message> {
    pub fn new(
        x: i16,
        y: i16,
        widget_spacing_x: u16,
        widget_spacing_y: u16,
        container_type: ContainerType,
        background_color: Color
    ) -> Self {
        Self {
            id: 0,
            widget_count: 0,
            buttons: Vec::new(),
            labels: Vec::new(),
            containers: Vec::new(),
            x,
            y,
            widget_spacing_x: widget_spacing_x as i16,
            widget_spacing_y: widget_spacing_y as i16,
            container_type,
            background_color
        }
    }


    pub fn create_container(
        &mut self,
        widget_spacing_x: u16,
        widget_spacing_y: u16,
        new_container_type: ContainerType
    ) -> &mut WidgetContainer<Message> {
        let mut x = self.x;
        let mut y = self.y;
        for container in &self.containers {
            match self.container_type {
                ContainerType::Vertical => y = self.move_point_vertical(container),
                ContainerType::Horizontal => x = self.move_point_horizontal(container)
            }
        } 
        (x, y) = self.get_button_endpoint(x, y);
        (x, y) = self.get_label_endpoint(x, y);
        let mut new_container = WidgetContainer::new(
            x,
            y,
            widget_spacing_x,
            widget_spacing_y,
            new_container_type,
            self.background_color.clone()
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
    ) -> &mut Button<Message> {
        let padding_x: i16 = padding_x as i16;
        let padding_y: i16 = padding_y as i16;
        let width = text.len() as i16 * 8 + padding_x;
        let height = padding_y * 2 + 16; 
        let mut x = self.x;
        let mut y = self.y;
        
        for container in &self.containers {
            match self.container_type {
                ContainerType::Vertical => y = self.move_point_vertical(container),
                ContainerType::Horizontal => x = self.move_point_horizontal(container)
            }
        }
        (x, y) = self.get_button_endpoint(x, y);
        (x, y) = self.get_label_endpoint(x, y);

        let points: [Point; 5] = [
            Point {x, y},
            Point {x: x + width, y},
            Point {x: x + width, y: y + height},
            Point {x, y: y + height},
            Point {x, y},
        ];
        self.buttons.push(Button {
            id: 0,
            text: text.to_string(),
            points,
            hover: false,
            message,
            foreground_color: None, 
            background_color: None,
            border: 5,
            border_color: None
        });
        self.buttons.iter_mut().last().unwrap()
    }

    pub fn create_label(&mut self, text: &str) {
        let mut x = self.x;
        let mut y = self.y;

        for container in &self.containers {
            match self.container_type {
                ContainerType::Vertical => y = self.move_point_vertical(container),
                ContainerType::Horizontal => x = self.move_point_horizontal(container)
            }
        }
        (x, y) = self.get_button_endpoint(x, y);
        (x, y) = self.get_label_endpoint(x, y);

        self.labels.push(Label::new(0, text, x, y, 0));
    }

    pub fn is_widget_interacted(&self, event_x: i16, event_y: i16) -> Option<&Button<Message>> {
        for container in &self.containers {
            if let Some(button) = container.is_widget_interacted(event_x, event_y) {
                return Some(button);
            }
        }
        for button in &self.buttons {
            if button.is_button_interacted(event_x, event_y) {
                return Some(&button);
            }
        }

        return None;
    }

    fn move_point_horizontal(&self, container: &WidgetContainer<Message>) -> i16 {
        let mut x = self.x;
        for child_container in &container.containers {
            let new_x = container.move_point_horizontal(child_container);
            if new_x > x {
                x = new_x;
            }
        }
        for button in &container.buttons {
            let button_x = button.points[2].x + self.widget_spacing_x;
            if button_x > x {
                x = button_x;
            }
        }
        for label in &container.labels {
            let label_x = label.end_x + self.widget_spacing_x;
            if label_x > x {
                x = label_x;
            }
        }
        x 
    }

    fn move_point_vertical(&self, container: &WidgetContainer<Message>) -> i16 {
        let mut y = self.y;
        for child_container in &container.containers {
            let new_y = container.move_point_vertical(child_container);
            if new_y > y {
                y = new_y;
            }
        }
        for button in &container.buttons {
            let button_y = button.points[2].y + self.widget_spacing_y;
            if button_y > y {
                y = button_y;
            }
        }
        for label in &container.labels {
            let label_y = label.end_y + self.widget_spacing_y;
            if label_y > y {
                y = label_y;
            }
        }
        y
    }

    fn get_button_endpoint(&self, mut x: i16, mut y: i16) -> (i16, i16) {
        for button in &self.buttons {
            match self.container_type {
                ContainerType::Vertical => {
                    let new_y = button.points[2].y + self.widget_spacing_y;
                    if new_y > y {
                        y = new_y;
                    }
                    
                }
                ContainerType::Horizontal => {
                    let new_x = button.points[2].x + self.widget_spacing_x;
                    if new_x > x {
                        x = new_x;
                    }
                }
            }
        }
        (x, y)
    }

    fn get_label_endpoint(&self, mut x: i16, mut y: i16) -> (i16, i16) {
        for label in &self.labels {
            match self.container_type {
                ContainerType::Vertical => {
                    let new_y = label.end_y + self.widget_spacing_y;
                    if new_y > y {
                        y = new_y;
                    }
                }
                ContainerType::Horizontal => {
                    let new_x = label.end_x + self.widget_spacing_x;
                    if new_x > x {
                        x = new_x;
                    }
                }
            }
        }
        (x, y)
    }

    pub fn padding(&mut self, padding: u16) -> &mut WidgetContainer<Message> {
        let padding: i16 = padding as i16;
        self.x += padding;
        self.y += padding;
        self
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

struct GraphicalContextMatrix {
    gc_ids: [[Gcontext; 10]; 10]
}

impl GraphicalContextMatrix {
    fn new<C: Connection>(window: &XWindow<C>) -> Result<Self, Box<dyn Error>> {
        let mut gc_ids = [[0; 10]; 10];
        for bg in 0..COLORS {
            for fg in 0..COLORS {
                let background_color = Color::get(bg);
                let foreground_color = Color::get(fg);
                let gc_id = new_gc(window, &background_color, &foreground_color, 0)?;
                gc_ids[bg][fg] = gc_id;
            }
        }
        Ok(Self {
            gc_ids
        })
    }

    fn get_only_button_background_contexts<C: Connection>(&self, window: &XWindow<C>) -> (Gcontext, Gcontext) {
        if let Color::Black = window.background_color {
            let background_gc = self.gc_ids[1][1];
            let foreground_gc = self.gc_ids[1][0];
        }
        for bg in 0..COLORS {
            for fg in 0..COLORS {
                let background_color = Color::get(bg);
                let foreground_color = Color::get(fg);
                if window.background_color == background_color {
                    let background_gc = self.gc_ids[fg][fg];
                    let foreground_gc = self.gc_ids[fg][0];
                }
            }
        }
        let background_gc = self.gc_ids[1][1];
        let foreground_gc = self.gc_ids[1][0];
        return (background_gc, foreground_gc);
    }

    fn get_default_button_contexts<C: Connection>(&self, window: &XWindow<C>) -> (Gcontext, Gcontext) {
        if let Color::Black = window.background_color {
            let background_gc = self.gc_ids[1][1];
            let foreground_gc = self.gc_ids[1][0];
            return (background_gc, foreground_gc);
        }
        for bg in 0..COLORS {
            for fg in 0..COLORS {
                let background_color = Color::get(bg);
                let foreground_color = Color::get(fg);
                if window.background_color == background_color {
                    let background_gc = self.gc_ids[fg][fg];
                    let foreground_gc = self.gc_ids[fg][1];
                    return (background_gc, foreground_gc);
                }

            }
        }
        let background_gc = self.gc_ids[1][1];
        let foreground_gc = self.gc_ids[1][0];
        return (background_gc, foreground_gc);
    }

    fn get_default_label_context<C: Connection>(&self, window: &XWindow<C>) -> Gcontext {
        if let Color::Black = window.background_color {
            return self.gc_ids[0][1];
        }
        for bg in 0..COLORS {
            for fg in 0..COLORS {
                let background_color = Color::get(bg);
                if background_color == window.background_color {
                    return self.gc_ids[bg][0];
                }
            }
        }
        self.gc_ids[0][1]
    }
}

fn new_gc<C: Connection>(
    window: &XWindow<C>,
    background_color: &Color,
    foreground_color: &Color,
    border_size: u32,
) -> Result<Gcontext, Box<dyn Error>> {
    let gc_id = window.connection.generate_id()?;
    let gc_values = CreateGCAux::new()
        .foreground(foreground_color.to_hex())
        .background(background_color.to_hex())
        .line_width(border_size)
        .font(window.font_id);
    window.connection.create_gc(gc_id, window.window_id, &gc_values)?;
    Ok(gc_id)
}

const COLORS: usize = 9; 
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Cyan,
    Yellow,
    Orange,
    Purple,
}

impl Color {
    fn get(index: usize) -> Color {
        match index {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Red,
            3 => Color::Green,
            4 => Color::Blue,
            5 => Color::Cyan,
            6 => Color::Yellow,
            7 => Color::Orange,
            8 => Color::Purple,
            _ => Color::Black
        }
    }
    fn to_hex(&self) -> u32 {
        match self {
            Color::Black => 0x0,
            Color::White => 0xffffff,
            Color::Red => 0xff0000,
            Color::Green => 0x00ff00,
            Color::Blue => 0x0000ff,
            Color::Cyan => 0x00ffff,
            Color::Yellow => 0xffff00,
            Color::Orange => 0xffa500,
            Color::Purple => 0x800080,
        }
    }
}
fn init_window_colors<C: Connection>(connection: &C, screen_number: usize, background_color: Color) -> (Color, Color) {
    let foreground_color = match background_color {
        Color::White => Color::Black,
        Color::Black => Color::White,
        _ => Color::Black
    };
    (background_color, foreground_color)
}


pub fn init<Application: Elm>(
    title: &str,
    width: u16,
    height: u16,
    mut application: Application 
) -> Result<(), Box<dyn Error>> {
    let mut container = application.view();
    generate_ids_for_widgets(&mut container);
    let (connection, screen_number) = x11rb::connect(None)?;
    let (background_color, foreground_color) = init_window_colors(
        &connection,
        screen_number,
        container.background_color.clone()
    );
    let window_id = connection.generate_id()?;
    let window_aux = CreateWindowAux::new()
        .event_mask(
            EventMask::EXPOSURE |
            EventMask::NO_EVENT |
            EventMask::BUTTON_PRESS |
            EventMask::KEY_PRESS |
            EventMask::POINTER_MOTION
        )
        .background_pixel(container.background_color.to_hex());

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
        background_color,
        foreground_color
    );

    // Get default font
    let font_id = window.connection.generate_id()?;
    window.connection.open_font(font_id, b"fixed")?;
    window.font_id = font_id;

    let default_gc_values = GraphicalContextMatrix::new(&window)?; 

    let mut button_hovered_id = 0;
    // Main event loop
    loop {
        let event = window.connection.wait_for_event()?;
        let mut redraw = false;
        let mut update = false;
        match event {
            Event::Expose(_) => {
                redraw = true;
            }
            Event::KeyPress(event) => {
            }
            Event::MotionNotify(event) => {
                if let Some(button) = container.is_widget_interacted(event.event_x, event.event_y) {
                    if button.id != button_hovered_id {
                        button_hovered_id = button.id;
                        redraw = true;
                    }
                } else {
                    if button_hovered_id != 0 {
                        button_hovered_id = 0;
                        redraw = true;
                    }
                }
            }
            Event::ButtonPress(event) => {
                match event.detail {
                    1 => {
                        if let Some(button) = container.is_widget_interacted(event.event_x, event.event_y) {
                            if application.update(&button.message) {
                                window.connection.clear_area(false, window.window_id, 0, 0, width, height)?;
                            }
                            update = true;
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
        if update {
            container = application.view();
            generate_ids_for_widgets(&mut container);
            draw_widgets(&mut window, &default_gc_values, &container, button_hovered_id)?;
            window.connection.flush()?;
            update = false;
        }
        if redraw {
            draw_widgets(&mut window, &default_gc_values, &container, button_hovered_id)?;
            window.connection.flush()?;
        }
    }
    window.connection.close_font(window.font_id)?;
}


fn draw_widgets<C: Connection, Message>(
    window: &mut XWindow<C>,
    default_gc_values: &GraphicalContextMatrix,
    parent_container: &WidgetContainer<Message>,
    button_hovered_id: u128,
) -> Result<(), Box<dyn Error>> {
    for container in &parent_container.containers {
        draw_widgets(window, default_gc_values, container, button_hovered_id)?;
    }
    let buttons = &parent_container.buttons;
    
    for button in buttons {
        if button.id == button_hovered_id && button.hover {
            // Button is hovered
            
        }
        // Button has no background nor foreground color set
        if button.background_color.is_none() && button.foreground_color.is_none() {
            let (default_background_gc, default_foreground_gc) = default_gc_values.get_default_button_contexts(window);
            button.draw_button(
                window,
                default_background_gc,
                default_foreground_gc,
                None,
                button.border
            )?;
            continue;
        }
        if button.background_color.is_some() && button.foreground_color.is_none() {
            apply_only_background_color_to_button(window, &button, default_gc_values)?;
            continue;
        }
        if button.foreground_color.is_some() && button.background_color.is_none() {
            apply_only_foreground_color_to_button(window, &button, default_gc_values)?;
            continue;
        }
            
        apply_colors_to_button(window, &button, default_gc_values)?;
    }
    let labels = &parent_container.labels;
    let label_gc = default_gc_values.get_default_label_context(window);

    for label in labels {
        window.connection.image_text8(
            window.window_id,
            label_gc,
            label.begin_x,
            label.begin_y + 15,
            label.text.as_bytes()
        )?;
    }
    Ok(())
}

fn apply_only_background_color_to_button<Message, C: Connection>(
    window: &XWindow<C>,
    button: &Button<Message>,
    default_gc_values: &GraphicalContextMatrix
) -> Result<(), Box<dyn Error>> {
    for bg in 0..COLORS {
        for fg in 0..COLORS {
            let background_color = Color::get(bg);
            if button.is_background_color_match(&background_color) {
                let background_gc = default_gc_values.gc_ids[bg][bg];
                let foreground_gc = if window.background_color == Color::Black {
                    default_gc_values.gc_ids[bg][0]
                } else {
                    default_gc_values.gc_ids[bg][1]
                };
                button.draw_button(window, background_gc, foreground_gc, None, button.border)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

fn apply_only_foreground_color_to_button<Message, C: Connection>(
    window: &XWindow<C>,
    button: &Button<Message>,
    default_gc_values: &GraphicalContextMatrix
) -> Result<(), Box<dyn Error>> {
    for bg in 0..COLORS {
        for fg in 0..COLORS {
            let foreground_color = Color::get(fg);
            if button.is_foreground_color_match(&foreground_color) {
                let (background_gc, foreground_gc) = if window.background_color == Color::Black {
                    (default_gc_values.gc_ids[1][1], default_gc_values.gc_ids[1][fg])
                } else {
                    let foreground_gc = default_gc_values.gc_ids[0][fg];
                    let background_gc = default_gc_values.gc_ids[0][0];
                    (background_gc, foreground_gc)
                };
                button.draw_button(window, background_gc, foreground_gc, None, button.border)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

fn apply_colors_to_button<Message, C: Connection>(
    window: &XWindow<C>,
    button: &Button<Message>,
    default_gc_values: &GraphicalContextMatrix
) -> Result<(), Box<dyn Error>> {
    let (default_background_gc, default_foreground_gc) = default_gc_values.get_default_button_contexts(window);
    let mut background_gc = default_background_gc;
    let mut foreground_gc = default_background_gc;
    for bg in 0..COLORS {
        for fg in 0..COLORS {
            let background_color = Color::get(bg);
            let foreground_color = Color::get(fg);
            let mut background_found = false;
            let mut foreground_found = false;
            if button.is_background_color_match(&background_color) {
                background_gc = default_gc_values.gc_ids[bg][bg];
                background_found = true;
            }
            if button.is_foreground_color_match(&foreground_color) {
                foreground_gc = default_gc_values.gc_ids[bg][fg];
                foreground_found = true;
            } 
            if background_found && foreground_found {
                button.draw_button(window, background_gc, foreground_gc, None, button.border)?;
                return Ok(());
            }
        }
    }
    button.draw_button(window, background_gc, foreground_gc, None, button.border)?;
    Ok(())
}

fn create_uuid() -> std::io::Result<u128> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open("/dev/urandom")?;
    let mut bytes = [0u8; 16];
    file.read_exact(&mut bytes)?;
    let mut uuid = u128::from_be_bytes(bytes);

    uuid &= !(0xF000_0000_0000_0000u128);
    uuid |= 0x4000_0000_0000_0000u128;
    
    uuid &= !(0xC000_0000_0000_0000u128);
    uuid |= 0x8000_0000_0000_0000u128;
    Ok(uuid)
}

fn generate_ids_for_widgets<Message>(container: &mut WidgetContainer<Message>) -> std::io::Result<()> {
    let uuid = create_uuid()?;
    container.id = uuid;
    for child in &mut container.containers {
        generate_ids_for_widgets(child)?;
    }

    for button in &mut container.buttons {
        let uuid = create_uuid()?;
        button.id = uuid;
    }

    for label in &mut container.labels {
        let uuid = create_uuid()?;
        label.id = uuid;
    }
    Ok(())
}
