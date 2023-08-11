use std::error;
use std::f64::consts::PI;
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
    pub m: f64,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true , n: 0.0, m: 0.0}
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

        frame.size().area();

        let res_x = frame.size().width as f64;
        let res_y = frame.size().height as f64;

        // println!("ratio {}", self.n);
        let character_ratio = 1.8;
        let aspect_ratio = res_x / res_y / character_ratio;
        let aspect_padding = (aspect_ratio-1.0)/2.0;

        let x_left = -1.0-aspect_padding;
        let x_right = 1.0+aspect_padding;

        let x_left_label = format!("{x_left:0.2}");
        let x_right_label = format!("{x_right:0.2}");
        let x_middle_label = format!("{:0.2}", (x_left + x_right)/2.0);


        let tm = MatrixFactory::rotate((0.0).into(),self.n.into(),(self.m).into());

        let mut triangle = Polygon::new(
            vec![
                Point::new(0.0 , 0.25),
                Point::new(0.25, -0.25),
                Point::new(-0.25, -0.25),
            ],
            Color::Yellow,
            (0.0,0.0),
            tm,
        );

        let mut onebyone = Polygon::new(
            vec![
                Point::new(-1.0,-1.0),
                Point::new(-1.0,1.0),
                Point::new(1.0,1.0),
                Point::new(1.0,-1.0),
            ],
            Color::Gray,
            (0.0,0.0),MatrixFactory::identity(),
        );

        let pentagram_vert: Vec<Point> = (0..5).into_iter().map(|n| {
            let step = 2.0*PI/5.0;
            let offset = -PI/10.0 * 0.0;
            let angle = n as f64 *2.0* step + offset;
            let scale = 0.5f64;
            Point { x: angle.cos() * scale, y: angle.sin() * scale }
        }).collect();

        let mut pentagram = Polygon::new(
            pentagram_vert,
            Color::Red,
            (0.0,0.0),
            tm,
        );

        pentagram.render();
        onebyone.render();

        let datasets = vec![
            pentagram.as_dataset(),
            onebyone.as_dataset(),
            // roof.as_dataset(),
            // floor.as_dataset(),
            // wall_left.as_dataset(),
            // wall_right.as_dataset(),
            // wand.as_dataset(),
        ];

        // println!("x: {res_x}");
        // println!("y: {res_y}");
        // println!("x/y: {aspect_ratio}");

        frame.render_widget(
            Chart::new(datasets)
                .block(Block::default().title(" DDDragon ").border_type(BorderType::Plain))
                .style(Style::default().fg(Color::White))
                .hidden_legend_constraints((Constraint::Length(0), Constraint::Length(0)))

                .x_axis(Axis::default()
                    //.title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([x_left, x_right])
                    .labels([x_left_label, x_middle_label, x_right_label].iter().cloned().map(Span::from).collect()))
                .y_axis(Axis::default()
                    //.title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([-1.0, 1.0])
                    .labels(["-1.0", "0", "1.0"].iter().cloned().map(Span::from).collect())),
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
    offset: (f64,f64),
    transformation: Matrix,
}

impl Polygon {
    fn new(vertices: Vec<Point>, color : Color, offset: (f64,f64), transformation: Matrix) -> Polygon{
        Polygon{
            vertices: vertices.clone(),
            points: Polygon::generate_points(&vertices, &color),
            sides: Polygon::generate_sides(&vertices, &color),
            color,
            offset,
            transformation

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

    fn render(&mut self){
        self.points = Polygon::generate_points(&self.vertices, &self.color);
        for  i in 0..self.points.len() {
            let (x,y) = self.points.get(i).unwrap();
            let z = 0f64;

            let tm = &self.transformation;
            let (offx, offy) = &self.offset;

            let xx = tm[0][0]*x + tm[0][1]*y + tm[0][2]*z + offx;  
            let yy = tm[1][0]*x + tm[1][1]*y + tm[1][2]*z + offy;  
            
            let (x,y) = self.points.get_mut(i).unwrap();

            *x = xx; *y = yy;
        }
    }


    pub fn transform(&mut self, tm: Matrix) {
        for i in 0..self.points.len(){
            let (x,y) = self.points.get(i).unwrap();
            let z = 0f64;

            let xx = tm[0][0]*x + tm[0][1]*y + tm[0][2]*z;  
            let yy = tm[1][0]*x + tm[1][1]*y + tm[1][2]*z;  
            let zz = tm[2][0]*x + tm[2][1]*y + tm[2][2]*z;  

            let (x,y) = self.points.get_mut(i).unwrap();

            *x = xx; *y = yy;
            
        }
    }

}



type Matrix = [[f64;3];3];

struct MatrixFactory;
impl MatrixFactory {
    fn identity() -> Matrix {
        [
            [1f64, 0f64, 0f64],
            [0f64, 1f64, 0f64],
            [0f64, 0f64, 1f64],
        ]
    }

    fn rotate_x(theta: Radians) -> Matrix {
        let cos = theta.data.cos();
        let sin = theta.data.sin();
        [ 
            [1f64, 0f64, 0f64],
            [0f64, cos , -sin],
            [0f64, sin , cos ]
        ]
    }

    fn rotate_y(theta: Radians) -> Matrix {
        let cos = theta.data.cos();
        let sin = theta.data.sin();
        [ 
            [cos , 0f64, sin ],
            [0f64, 1f64, 0f64],
            [-sin, 0f64, cos ]
        ]
    }

    fn rotate_z(theta: Radians) -> Matrix {
        let cos = theta.data.cos();
        let sin = theta.data.sin();
        [ 
            [cos , -sin, 0f64],
            [sin ,  cos, 0f64],
            [0f64, 0f64, 1f64]
        ]
    }

    fn rotate(roll: Radians, pitch: Radians, yaw: Radians)  -> Matrix {
        let sina = yaw.data.sin();
        let cosa = yaw.data.cos();
        let sinb = pitch.data.sin();
        let cosb = pitch.data.cos();
        let sing = roll.data.sin();
        let cosg = roll.data.cos();

        [ 
            [
                cosa*cosb, 
                (cosa*sinb*sing) - (sina*cosg), 
                (cosa*sinb*cosg) + (sina*sing)
            ],
            [
                sina*cosb, 
                (sina*sinb*sing) + (cosa*cosg), 
                (sina*sinb*cosg) - (cosa*sing)
            ],
            [
                -sinb, 
                cosb*sing, 
                cosb*cosg
            ],
        ]
    }
}


struct Radians {
    data: f64
}

struct Degrees {
    data: f64
}

impl From<f64> for Radians {
    fn from(value: f64) -> Self {
        Radians { data: value }
    }
}

impl Into<Radians> for Degrees {
    fn into(self) -> Radians {
        Radians { data: self.data*PI/180f64 }
    }
}

impl Into<Degrees> for Radians {
    fn into(self) -> Degrees {
        Degrees { data: self.data*180f64/PI }
    }
}