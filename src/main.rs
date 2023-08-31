use sfml::{
    graphics::{
        Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
enum NodeType {
    Start,
    Destination,
    Wall,
    Visited,
    Empty,
}

const RECTANGLE_SIDE_SIZE: u32 = 30;

#[derive(Debug)]
struct Node<'a> {
    position: Position,
    r#type: NodeType,
    shape: RectangleShape<'a>,
}

impl Node<'_> {
    fn new(x: u32, y: u32, r#type: NodeType) -> Self {
        let mut shape = RectangleShape::new();
        shape.set_size(Vector2f::new(
            RECTANGLE_SIDE_SIZE as f32,
            RECTANGLE_SIDE_SIZE as f32,
        ));
        shape.set_position(Vector2f::new(
            (x * RECTANGLE_SIDE_SIZE) as f32,
            (y * RECTANGLE_SIDE_SIZE) as f32,
        ));
        Self {
            r#type,
            position: Position { x, y },
            shape,
        }
    }

    fn set_type(&mut self, r#type: NodeType) {
        self.r#type = r#type;
        match self.r#type {
            NodeType::Start => self.shape.set_fill_color(Color::BLUE),
            NodeType::Destination => self.shape.set_fill_color(Color::GREEN),
            NodeType::Wall => self.shape.set_fill_color(Color::RED),
            NodeType::Empty => self.shape.set_fill_color(Color::WHITE),
            NodeType::Visited => self.shape.set_fill_color(Color::YELLOW),
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

struct Playground<'p> {
    nodes: Vec<Box<Node<'p>>>,
    window: RenderWindow,
    state: PlaygroundState,
    width: u32,
    height: u32,
}

impl<'p> Playground<'p> {
    fn new(width: u32, height: u32) -> Self {
        let mut nodes = Vec::<Box<Node<'p>>>::new();
        for i in 0..(height * width) {
            println!("{}. Add node at position {},{}", i, i % width, i / height);
            nodes.push(Box::new(Node::new(i % width, i / height, NodeType::Empty)));
        }

        Self {
            state: PlaygroundState::SelectStartPoint,
            nodes,
            width,
            height,
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
                        scan: _,
                    } => match code {
                        Key::Enter => self.state = PlaygroundState::Play,
                        _ => (),
                    },
                    Event::MouseButtonPressed { button: _, x, y } => match self.state {
                        PlaygroundState::SelectStartPoint => {
                            let node = self.get_node(
                                x as u32 / RECTANGLE_SIDE_SIZE,
                                y as u32 / RECTANGLE_SIDE_SIZE,
                            );
                            match node {
                                Some(node) => {
                                    if node.r#type == NodeType::Empty {
                                        node.set_type(NodeType::Start);
                                    }
                                }
                                None => println!("Cannot find desired node!"),
                            }
                            self.state = PlaygroundState::SelectDestination;
                        }
                        PlaygroundState::SelectDestination => {
                            let node = self.get_node(
                                x as u32 / RECTANGLE_SIDE_SIZE,
                                y as u32 / RECTANGLE_SIDE_SIZE,
                            );
                            match node {
                                Some(node) => {
                                    if node.r#type == NodeType::Empty {
                                        node.set_type(NodeType::Destination);
                                    }
                                }
                                None => println!("Cannot find desired node!"),
                            }
                            self.state = PlaygroundState::BuildWall;
                        }
                        PlaygroundState::BuildWall => {
                            let node = self.get_node(
                                x as u32 / RECTANGLE_SIDE_SIZE,
                                y as u32 / RECTANGLE_SIDE_SIZE,
                            );
                            match node {
                                Some(node) => {
                                    if node.r#type == NodeType::Empty {
                                        node.set_type(NodeType::Wall);
                                    }
                                }
                                None => println!("Cannot find desired node!"),
                            }
                        }
                        PlaygroundState::Play => {
                            let adj_nodes = self.find_adjacent_nodes(
                                x as u32 / RECTANGLE_SIDE_SIZE,
                                y as u32 / RECTANGLE_SIDE_SIZE,
                            );
                            match adj_nodes {
                                Some(nodes) => {
                                    println!("Size {}", nodes.len());
                                    for node in nodes.into_iter() {
                                        println!(
                                            "Adjust color for node {} {}",
                                            node.position.x, node.position.y
                                        );
                                        node.set_type(NodeType::Visited);
                                    }
                                }
                                _ => (),
                            }
                        }
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

    fn get_node(&mut self, x: u32, y: u32) -> Option<&mut Box<Node<'p>>> {
        println!("Get node at {},{}", x, y);
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&mut self.nodes[(x + y * self.width) as usize])
    }

    fn find_adjacent_nodes(&mut self, x: u32, y: u32) -> Option<Vec<&mut Box<Node<'p>>>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let nodes: Vec<&mut Box<Node<'_>>> = self
            .nodes
            .iter_mut()
            .filter(|node| {
                let mut result = (node.r#type == NodeType::Empty
                    || node.r#type == NodeType::Destination)
                    && (node.position.x != x || node.position.y != y);
                if x > 0 {
                    result = result && node.position.x >= (x - 1);
                }
                if y > 0 {
                    result = result && node.position.y >= (y - 1);
                }
                result = result && node.position.x <= (x + 1) && node.position.y <= (y + 1);
                result
            })
            .collect();
        Some(nodes)
    }
}

fn main() {
    let mut playground = Playground::new(20, 20);
    playground.run();
}
