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

type Point3d = (f64,f64,f64);
type Point2d = (f64,f64);

type Points3d = Vec<(f64,f64,f64)>;
type Points2d = Vec<(f64,f64)>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub a: f64,
    pub b: f64,

    // global offsets
    pub posx: f64,
    pub posy: f64,
    pub posz: f64,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true , a: 0.0, b: 0.0, posx:0.0, posy:0.0, posz:0.0}
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

        let player_height = 0.1;

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


        let tm: [[f64; 3]; 3] = MatrixFactory::rotate((self.a).into(),(self.b).into(),(0.0).into());
        let tm2: [[f64; 3]; 3] = MatrixFactory::rotate((self.a/2.0).into(),(self.b/2.0).into(),(0.0).into());

        // let mut triangle = Polygon::new(
        //     vec![
        //         Point::new(0.0 , 0.25),
        //         Point::new(0.25, -0.25),
        //         Point::new(-0.25, -0.25),
        //     ],
        //     Color::Yellow,
        //     (0.0,0.0),
        //     tm,
        // );

        let mut onebyone = Polygon::new(
            vec![
                (-1.0, -1.0, 0.0),
                (-1.0, 1.0 , 0.0),
                (1.0 , 1.0 , 0.0),
                (1.0 , -1.0, 0.0),
            ],
            Color::Gray,
            (1.0,1.0,7.0),MatrixFactory::identity(),
        );

        let pentagram_vert: Points3d = (0..5).into_iter().map(|n| {
            let step = 2.0*PI/5.0;
            let offset = -PI/10.0 * 0.0;
            let angle = n as f64 *2.0* step + offset;
            let scale = 0.5f64;
            (angle.cos() * scale, angle.sin() * scale , 0.0)
        }).collect();

        let mut pentagram = Polygon::new(
            pentagram_vert.clone(),
            Color::LightMagenta,
            (1.0,1.0,5.0),
            tm,
        );

        let mut pentagram2 = Polygon::new(
            pentagram_vert,
            Color::Red,
            (1.0,1.0,3.0),
            tm2,
        );
        let ftr = (1.0,1.0,0.0);  // naming : (front || back) && (top || bottom) && (left || right)
        let ftl = (-1.0,1.0,0.0);
        let fbr = (1.0,-1.0,0.0);
        let fbl = (-1.0,-1.0,0.0);
        let btr = (1.0,1.0,-1.0);
        let btl = (-1.0,1.0,-1.0);
        let bbr = (1.0,-1.0,-1.0);
        let bbl = (-1.0,-1.0,-1.0);

        
        let mut unit_cube = Polyhedron::new(
            vec![
                Polygon::new(
                    vec![
                        ftr, fbr, fbl, ftl
                    ],
                    Color::Gray,
                    (player_height,0.0,7.0),tm
                ),
                Polygon::new(
                    vec![
                        ftr, fbr, bbr, btr
                    ],
                    Color::Green,
                    (player_height,0.0,7.0),tm
                ),
                Polygon::new(
                    vec![                        
                        btr, bbr, bbl, btl
                    ],
                    Color::Blue,
                    (player_height,0.0,7.0),tm
                ),
                Polygon::new(
                    vec![
                        ftl, fbl, bbl, btl
                    ],
                    Color::Magenta,
                    (player_height,0.0,7.0),tm
                ),

            ]
        );

        pentagram2.render(&self);
        unit_cube.render(&self);
        pentagram.render(&self);
        onebyone.render(&self);
        // onebyone.render();

        let mut datasets = vec![
            onebyone.as_dataset(),
            pentagram.as_dataset(),
            pentagram2.as_dataset(),
        ];

        datasets.append(unit_cube.as_datasets().as_mut());





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



const LINE_POINTS: i32 = 200;
const LINE_RESOLUTION: f64 = 1.0 / (LINE_POINTS as f64);
struct Line{
    points: Points3d,
    start: Point3d,
    end: Point3d,
    center: Point3d,
    color: Color,
}

impl Line{
    fn new(start: Point3d, end: Point3d, color: Color) -> Line{
        Line{
            points: Line::interpolate(&start, &end),
            start,
            end,
            center: Line::center_point_of(&start, &end),
            color,
        }
    }

    

    fn center_point_of(a: &Point3d, b: &Point3d) -> Point3d {
        let (a0, a1, a2) = a;
        let (b0, b1, b2) = b;
        let c = |a,b| -> f64 {(a+b)/2.0};
        (c(a0, b0),c(a1, b1),c(a2, b2))
    }

    pub fn interpolate(start: &Point3d, end: &Point3d) -> Points3d {
        let (endx, endy, endz) = end;
        let (startx, starty, startz) = start;
        let (deltax, deltay, deltaz): Point3d = ((endx-startx)*LINE_RESOLUTION,(endy-starty)*LINE_RESOLUTION,(endz-startz)*LINE_RESOLUTION);  //end.subtract(&start).scale(LINE_RESOLUTION);
        
        let data : Points3d = (0..LINE_POINTS).map(|i|{
            (
                (startx + (deltax*(i as f64))), 
                (starty + (deltay*(i as f64))), 
                (startz + (deltaz*(i as f64))),
            )
            //start.clone().add(&deltas.scale(i as f64)).into()
        }).collect();
        data
    }

    // pub fn as_data(&self) -> Vec<(f64, f64)> {
    //     self.points.clone()
    // }

    // pub fn as_dataset(&self) -> Dataset{

    //     Dataset::default()
    //         .marker(symbols::Marker::Braille)
    //         .graph_type(GraphType::Line)
    //         .style(Style::default().fg(self.color))
    //         .data(&self.points)
    // } 
 

    // pub fn color(mut self, color : Color) -> Self {
    //     self.color = color;
    //     self
    // }
}


struct Polygon{
    vertices: Points3d,
    points: Points3d,
    color: Color,
    offset: Point3d,
    transformation: Matrix,
    projection: Points2d,
    center: Point3d
}

impl Polygon {
    fn new(vertices: Points3d, color : Color, offset: Point3d, transformation: Matrix) -> Self{
        Self{
            vertices: vertices.clone(),
            center: (0.0f64,0.0f64,0.0f64),
            points: Vec::new(),
            color,
            offset,
            transformation,
            projection: Vec::new(),
        }
    }
    fn generate_sides(&mut self) -> Vec<Line> {
        let mut sides : Vec<Line> = Vec::new();
        for i in 1..self.vertices.len() {
            let l = Line::new(
                self.vertices[i-1].clone(),
                self.vertices[i].clone(),
                self.color
            );
            sides.push(l);
        }
        let l = Line::new(
            self.vertices[self.vertices.len()-1].clone(),
            self.vertices[0].clone(),
            self.color
        );
        sides.push(l);
        sides
    }

    fn generate_points(&mut self) {
        let mut sides = self.generate_sides();
        let mut points : Points3d = Vec::new();
        for i in 0..sides.len() {
            points.append(&mut sides[i].points);
        }
        self.points = points;
    }

    fn as_dataset(&self) -> Dataset {
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(self.color))
            .data(&self.projection)
    }

    fn render(&mut self, app: &App){
        self.generate_points();
        self.transform(&app);
        self.project(app);
    }
    
    fn project(&mut self, app: &App){
        let fov = PI/4.0; // 90deg
        let ez = 1.0/((fov/2.0).tan());
        self.projection = self.points.iter().map(|a| {
            project_point(a.clone(), (0.0, 0.0, ez), (0.0,0.0,0.0)) 
        }).collect()
    } 

    pub fn transform(&mut self, app: &App) {
        let tm = &self.transformation;
        for i in 0..self.points.len(){
            let (x,y,z) = self.points.get(i).unwrap();

            let (offx, offy, offz) = &self.offset;

            let xx = tm[0][0]*x + tm[0][1]*y + tm[0][2]*z + offx + app.posx;  
            let yy = tm[1][0]*x + tm[1][1]*y + tm[1][2]*z + offy + app.posy;  
            let zz = tm[2][0]*x + tm[2][1]*y + tm[2][2]*z + offz + app.posz;  
            
            let (x,y, z) = self.points.get_mut(i).unwrap();

            *x = xx; *y = yy; *z = zz;
        }
    }

}


struct Polyhedron {
    polygons: Vec<Polygon>,
}


impl Polyhedron {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Polyhedron { 
            polygons,
        }
    }

    fn as_datasets(&self) -> Vec<Dataset> {
        self.polygons.iter().map(|p|{
            Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(p.color))
            .data(&p.projection)
        }).collect()
    }

    fn render(&mut self, app: &App) {
        for i in 0..self.polygons.len(){
            self.polygons.get_mut(i).unwrap().render(app)
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


fn project_point(a: Point3d, e: Point3d, t: Point3d) -> Point2d {
    let (ax, ay, az): Point3d = a; // point to be projected
    let (ex, ey, ez): Point3d = e; // display's surface position relative to the camera position <0,0,0>
    let (tx, ty, tz): Point3d = t; // the angles of the camera (Tait-Brian angles)
    
    let (cx, cy, cz): Point3d = (tx.cos(),ty.cos(),tz.cos());
    let (sx, sy, sz): Point3d = (tx.sin(),ty.sin(),tz.sin());
    
    let x = ax-cx;
    let y = ay-cy;
    let z = az-cz;

    let dx = cy * ((sz*y) + (cz*x)) - (sy*z);
    let dy = sx * ( (cy*z) + sy*( (sz*y) + (cz*x) ) ) + cx*( (cz*y) - (sz*x) );
    let dz = cx * ( (cy*z) + sy*( (sz*y) + (cz*x) ) ) - sx*( (cz*y) - (sz*x) );

    let bx = (ez/dz)*dx + ex;
    let by = (ez/dz)*dy + ey;
    
    (bx,by) // projected point
}