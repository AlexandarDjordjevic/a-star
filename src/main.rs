use sfml::{graphics::{RenderWindow, Drawable, RenderStates, RenderTarget, RectangleShape, Shape, Color, Transformable}, window::{Style, Event}, system::Vector2f};

struct Position {
    x: f32,
    y: f32,
}

const RECTANGLE_SIDE_SIZE: u32 = 50;

struct Node<'a> {
    position: Position,
    visited: bool,
    shape: RectangleShape<'a>,
}

impl Node<'_> {
    fn new(x: u32, y: u32) -> Self {
        let position: Position = Position { x: (x / RECTANGLE_SIDE_SIZE * RECTANGLE_SIDE_SIZE) as f32 , y: (y / RECTANGLE_SIDE_SIZE * RECTANGLE_SIDE_SIZE) as f32};
        let mut shape = RectangleShape::new();
        shape.set_fill_color(Color::RED);
        shape.set_size(Vector2f::new(
            RECTANGLE_SIDE_SIZE as f32,
            RECTANGLE_SIDE_SIZE as f32,
        ));
        shape.set_position(Vector2f::new(position.x, position.y));
        println!("Construct node at position x:{}, y:{}", position.x, position.y);
        Self {
            position,
            visited: false,
            shape,
        }
    }
}

struct Playground {
    nodes: Vec<Node>,
}

impl Playground {
    fn new() -> Self {

    }
}

impl Drawable for Node<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>
    ) {
        target.draw(&self.shape);
    }
}


fn main() {
    let mut items = Vec::<Box<dyn Drawable>>::new();
    let mut window = RenderWindow::new((800, 800), "A Star", Style::CLOSE, &Default::default());
    let mut index = 0;
    'main_loop: loop {
        let event_opt = window.poll_event();
         match event_opt {
            Some(event) => {
                match event {
                    Event::MouseButtonPressed { button, x, y } => {
                        items.push(Box::new(Node::new(x as u32, y as u32)));
                        index = index + 1;
                    },
                    Event::Closed => {
                        break 'main_loop;
                    }
                    _ => (),
                }
            },
            None => (),
        }
        window.clear(Color::BLACK);
        for item in &items {
            window.draw(item.as_ref());
        }
        window.display();
    }
}
