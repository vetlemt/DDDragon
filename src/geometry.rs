use tui::style::{Style, Color};
use tui::symbols;
use tui::widgets::{Dataset, GraphType};

#[derive(Debug,Clone,Copy)]
pub struct Point {
    x: f64,
    y: f64,
}
impl Point{
    pub fn new(x: f64, y: f64) -> Point{
        Point{x, y}
    }
    pub fn add(&self, other: &Point) -> Point{
        Point{x: self.x + other.x, y: self.y + other.y}
    }
    pub fn subtract(&self, other: &Point) -> Point {
        Point{x: self.x - other.x, y: self.y - other.y}
    }
    pub fn scale(&self, scalar : f64) -> Point {
        Point{x: self.x * scalar, y: self.y * scalar}
    }
}
impl Into<(f64,f64)> for Point {
    fn into(self) -> (f64,f64) {
        (self.x,self.y)
    }
}

const LINE_POINTS: i32 = 20;
const LINE_RESOLUTION: f64 = 1.0 / (LINE_POINTS as f64);
pub struct Line{
    points: Vec<(f64, f64)>,
    start: Point,
    end: Point,
    color: Color,
}

impl Line{
    pub fn new(start: Point, end: Point, color: Color) -> Line{
        Line{
            points: Line::interpolate(&start, &end),
            start,
            end,
            color,
        }
    }
    fn interpolate(start: &Point, end: &Point) -> Vec<(f64, f64)> {
        let deltas = end.subtract(&start).scale(LINE_RESOLUTION);
        let data : Vec<(f64, f64)> = (0..(LINE_POINTS+1)).map(|i|{
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

pub struct Polygon{
    pub vertices: Vec<Point>,
    pub points: Vec<(f64, f64)>,
    pub sides: Vec<Line>,
    pub color: Color,
}

impl Polygon {
    pub fn new(vertices: Vec<Point>, color : Color) -> Polygon{
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

    pub fn as_dataset(&self) -> Dataset {
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(self.color))
            .data(&self.points)
    }
}
