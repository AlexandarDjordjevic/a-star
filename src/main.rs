use sfml::{
    graphics::{
        Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};

struct Position {
    x: f32,
    y: f32,
}

enum NodeType {
    Start,
    Destination,
    Wall,
}

const RECTANGLE_SIDE_SIZE: u32 = 30;

struct Node<'a> {
    position: Position,
    visited: bool,
    r#type: NodeType,
    shape: RectangleShape<'a>,
}

impl Node<'_> {
    fn new(x: u32, y: u32, r#type: NodeType) -> Self {
        let position: Position = Position {
            x: (x / RECTANGLE_SIDE_SIZE * RECTANGLE_SIDE_SIZE) as f32,
            y: (y / RECTANGLE_SIDE_SIZE * RECTANGLE_SIDE_SIZE) as f32,
        };
        let mut shape = RectangleShape::new();
        match r#type {
            NodeType::Start => shape.set_fill_color(Color::BLUE),
            NodeType::Destination => shape.set_fill_color(Color::GREEN),
            NodeType::Wall => shape.set_fill_color(Color::RED),
        }
        shape.set_size(Vector2f::new(
            RECTANGLE_SIDE_SIZE as f32,
            RECTANGLE_SIDE_SIZE as f32,
        ));
        shape.set_position(Vector2f::new(position.x, position.y));
        println!(
            "Construct node at position x:{}, y:{}",
            position.x, position.y
        );
        Self {
            r#type,
            position,
            visited: false,
            shape,
        }
    }
}

impl Drawable for Node<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw(&self.shape);
    }
}

enum PlaygroundState {
    SelectStartPoint,
    SelectDestination,
    BuildWall,
    Play,
}

struct Playground<'a> {
    nodes: Vec<Box<Node<'a>>>,
    window: RenderWindow,
    state: PlaygroundState,
}

impl Playground<'_> {
    fn new(width: u32, height: u32) -> Self {
        Self {
            state: PlaygroundState::SelectStartPoint,
            nodes: Vec::new(),
            window: RenderWindow::new(
                (width * RECTANGLE_SIDE_SIZE, height * RECTANGLE_SIDE_SIZE),
                "A Star",
                Style::CLOSE,
                &Default::default(),
            ),
        }
    }

    fn run(&mut self) {
        'main_loop: loop {
            match self.window.poll_event() {
                Some(event) => match event {
                    Event::KeyPressed {
                        code,
                        alt: _,
                        ctrl: _,
                        shift: _,
                        system: _,
                    } => match code {
                        Key::Enter => self.state = PlaygroundState::Play,
                        _ => (),
                    },
                    Event::MouseButtonPressed { button: _, x, y } => match self.state {
                        PlaygroundState::SelectStartPoint => {
                            self.nodes.push(Box::new(Node::new(
                                x as u32,
                                y as u32,
                                NodeType::Start,
                            )));
                            self.state = PlaygroundState::SelectDestination;
                        }
                        PlaygroundState::SelectDestination => {
                            self.nodes.push(Box::new(Node::new(
                                x as u32,
                                y as u32,
                                NodeType::Destination,
                            )));
                            self.state = PlaygroundState::BuildWall;
                        }
                        PlaygroundState::BuildWall => {
                            self.nodes.push(Box::new(Node::new(
                                x as u32,
                                y as u32,
                                NodeType::Wall,
                            )));
                        }
                        PlaygroundState::Play => todo!(),
                    },
                    Event::Closed => {
                        break 'main_loop;
                    }
                    _ => (),
                },
                None => (),
            }

            self.render();
        }
    }

    fn render(&mut self) {
        self.window.clear(Color::BLACK);
        for node in &self.nodes {
            self.window.draw(node.as_ref());
        }
        self.window.display();
    }
}

fn main() {
    let mut playground = Playground::new(20, 20);
    playground.run();
}
