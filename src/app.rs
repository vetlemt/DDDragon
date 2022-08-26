use std::error;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint};
use tui::style::{Color, Style};
use tui::symbols;
use tui::terminal::Frame;
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, BorderType, Chart, Dataset, GraphType, Paragraph};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub n: f64,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true , n: 0.49}
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples

        let center = Point::new(0.5, 0.5);
        let top_left = Point::new(0.0, 1.0);
        let top_right = Point::new(1.0, 1.0);
        let bottom_left = Point::new(0.0, 0.0);
        let bottom_right = Point::new(1.0, 0.0);

        let player_height = 0.7;
        let view_correction = 0.5 + player_height;
        let rectangle = Polygon::new(
            vec![
                Point::new(self.n, (self.n ) * view_correction.clone()),
                Point::new(self.n, (1.0-self.n ) * view_correction.clone()),
                Point::new(1.0-self.n, (1.0-self.n ) * view_correction.clone()),
                Point::new(1.0-self.n, (self.n ) * view_correction.clone()),
            ],
            Color::Blue,
        );

        let floor = Polygon::new(
            vec![
                bottom_left.clone(),
                rectangle.vertices[0].clone(),
                rectangle.vertices[3].clone(),
                bottom_right.clone(),
            ],
            Color::White,
        );

        let roof = Polygon::new(
            vec![
                top_left.clone(),
                top_right.clone(),
                rectangle.vertices[2].clone(),
                rectangle.vertices[1].clone(),
            ],
            Color::White,
        );

        let wall_left = Polygon::new(
            vec![
                top_left.clone(),
                rectangle.vertices[1].clone(),
                rectangle.vertices[0].clone(),
                bottom_left.clone(),
            ],
            Color::Yellow,
        );

        let wall_right = Polygon::new(
            vec![
                top_right.clone(),
                bottom_right.clone(),
                rectangle.vertices[3].clone(),
                rectangle.vertices[2].clone(),
            ],
            Color::Yellow,
        );

        let wand = Polygon::new(
            vec![
                Point::new(0.8, 0.0),
                Point::new(0.7, 0.3),
                Point::new(0.73, 0.35),
                Point::new(0.85, 0.0),
                Point::new(0.78, 0.0),
                Point::new(0.7, 0.25),
                Point::new(0.7, 0.3),
            ],
            Color::Magenta,
        );

        let datasets = vec![
            roof.as_dataset(),
            floor.as_dataset(),
            wall_left.as_dataset(),
            wall_right.as_dataset(),
            wand.as_dataset(),
        ];


        frame.render_widget(
            Chart::new(datasets)
                .block(Block::default().title(" DDDragon ").border_type(BorderType::Plain))
                .style(Style::default().fg(Color::White))
                .hidden_legend_constraints((Constraint::Length(0), Constraint::Length(0)))

                .x_axis(Axis::default()
                    //.title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 1.0])
                    .labels(["0.0", "0.5", "1.0"].iter().cloned().map(Span::from).collect()))
                .y_axis(Axis::default()
                    //.title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 1.0])
                    .labels(["0.0", "0.5", "1.0"].iter().cloned().map(Span::from).collect())),
            frame.size(),
        )
    }
}


#[derive(Debug,Clone)]
struct Point {
    x: f64,
    y: f64,
}
impl Point{
    fn new(x: f64, y: f64) -> Point{
        Point{x, y}
    }
    fn add(&self, other: &Point) -> Point{
        Point{x: self.x + other.x, y: self.y + other.y}
    }
    fn subtract(&self, other: &Point) -> Point {
        Point{x: self.x - other.x, y: self.y - other.y}
    }
    fn scale(&self, scalar : f64) -> Point {
        Point{x: self.x * scalar, y: self.y * scalar}
    }
    fn get_ref(&self) -> (&f64, &f64){
        (&self.x, &self.y)
    }
}
impl Into<(f64,f64)> for Point {
    fn into(self) -> (f64,f64) {
        (self.x,self.y)
    }
}

const LINE_POINTS: i32 = 100;
const LINE_RESOLUTION: f64 = 1.0 / (LINE_POINTS as f64);
struct Line{
    points: Vec<(f64, f64)>,
    start: Point,
    end: Point,
    color: Color,
}

impl Line{
    fn new(start: Point, end: Point, color: Color) -> Line{
        Line{
            points: Line::interpolate(&start, &end),
            start,
            end,
            color,
        }
    }
    pub fn interpolate(start: &Point, end: &Point) -> Vec<(f64, f64)> {
        let deltas = end.subtract(&start).scale(LINE_RESOLUTION);
        let data : Vec<(f64, f64)> = (0..LINE_POINTS).map(|i|{
            start.clone().add(&deltas.scale(i as f64)).into()
        }).collect();
        data
    }

    pub fn as_data(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    pub fn as_dataset(&self) -> Dataset{

        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(self.color))
            .data(&self.points)
    }

    pub fn color(mut self, color : Color) -> Self {
        self.color = color;
        self
    }
}

struct Polygon{
    vertices: Vec<Point>,
    points: Vec<(f64, f64)>,
    sides: Vec<Line>,
    color: Color,
}

impl Polygon {
    fn new(vertices: Vec<Point>, color : Color) -> Polygon{
        Polygon{
            vertices: vertices.clone(),
            points: Polygon::generate_points(&vertices, &color),
            sides: Polygon::generate_sides(&vertices, &color),
            color
        }
    }
    fn generate_sides(vertices: &Vec<Point>, color : &Color) -> Vec<Line> {
        let mut sides : Vec<Line> = Vec::new();
        for i in 1..vertices.len() {
            let l = Line::new(
                vertices[i-1].clone(),
                vertices[i].clone(),
                *color
            );
            sides.push(l);
        }
        let l = Line::new(
            vertices[vertices.len()-1].clone(),
            vertices[0].clone(),
            *color
        );
        sides.push(l);
        sides
    }

    fn generate_points(vertices: &Vec<Point>, color : &Color) -> Vec<(f64, f64)> {
        let mut sides = Polygon::generate_sides(&vertices, &color);
        let mut points : Vec<(f64, f64)> = Vec::new();
        for i in 0..sides.len() {
            points.append(&mut sides[i].points);
        }
        points
    }

    fn as_dataset(&self) -> Dataset {
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(self.color))
            .data(&self.points)
    }

}
