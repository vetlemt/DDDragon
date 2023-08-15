use std::borrow::BorrowMut;
use std::error;
use std::f64::consts::PI;
use futures::future::join_all;
use threadpool::ThreadPool;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint};
use tui::style::{Color, Style};
use tui::symbols;
use tui::terminal::Frame;
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, BorderType, Chart, Dataset, GraphType, Paragraph};
use crate::quaternions::Quaternion;


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
    
    pub world: WorldMetrics,

    polygons: Vec<Polygon>,
}

impl Default for App {
    fn default() -> Self {
        Self { 
            running: true , 
            world: WorldMetrics::default(), 
            polygons: Vec::new()
        }
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
        self.polygons.clear();

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


        let tm: [[f64; 3]; 3] = MatrixFactory::rotate((0.0).into(),(0.0).into(),(0.0).into());
        let tm2: [[f64; 3]; 3] = MatrixFactory::rotate((0.0/2.0).into(),(0.0/2.0).into(),(0.0).into());

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
            let offset = -PI/10.0;
            let angle = n as f64 *2.0* step + offset;
            let scale = 1.0f64;
            (angle.cos() * scale, angle.sin() * scale , 0.0)
        }).collect();

        let mut pentagram = Polygon::new(
            pentagram_vert.clone(),
            Color::Red,
            (0.0,0.0,7.0),
            tm2,
        );

        let mut pentagram2 = Polygon::new(
            pentagram_vert,
            Color::Red,
            (0.0,0.0,7.0),
            tm2,
        );
        let ftr = ( 1.0, 1.0, 1.0);  // naming : (front || back) && (top || bottom) && (left || right)
        let ftl = (-1.0, 1.0, 1.0);
        let fbr = ( 1.0,-1.0, 1.0);
        let fbl = (-1.0,-1.0, 1.0);
        let btr = ( 1.0, 1.0,-1.0);
        let btl = (-1.0, 1.0,-1.0);
        let bbr = ( 1.0,-1.0,-1.0);
        let bbl = (-1.0,-1.0,-1.0);

        
        let mut unit_cube = Polyhedron::new(
            vec![
                Polygon::new(
                    vec![
                        ftr, fbr, fbl, ftl
                    ],
                    Color::Blue,
                    (0.0,0.0,7.0),tm2
                ),
                Polygon::new(
                    vec![
                        ftr, fbr, bbr, btr
                    ],
                    Color::Green,
                    (0.0,0.0,7.0),tm2
                ),
                Polygon::new(
                    vec![                        
                        btr, bbr, bbl, btl
                    ],
                    Color::LightYellow,
                    (0.0,0.0,7.0),tm2
                ),
                Polygon::new(
                    vec![
                        ftl, fbl, bbl, btl
                    ],
                    Color::Magenta,
                    (0.0,0.0,7.0),tm2
                ),

            ]
        );

        let pentaface: Points3d = (0..5).into_iter().map(|n| {
            let step = PI/5.0;
            let offset = -PI/10.0 * 0.0;
            let angle = n as f64 *2.0* step + offset;
            let scale = 0.5f64;
            (angle.cos() * scale, 0.0, angle.sin() * scale )
        }).collect();


        let phi = (1.0 + 5.0f64.sqrt())/2.0;
        let phi_i = 1.0 / phi;
        let dodecavertexes = vec![
            (1.0   , 1.0   , 1.0  ),
            (1.0   , 1.0   , -1.0 ),
            (1.0   , -1.0  , 1.0  ),
            (1.0   , -1.0  , -1.0 ),

            (0.0   , phi_i, phi   ),
            (0.0   , phi_i, -phi  ),
            (0.0   , -phi_i, phi  ),
            (0.0   , -phi_i, -phi ),
            
            (-1.0   , 1.0   , 1.0  ),
            (-1.0   , 1.0   , -1.0 ),
            (-1.0   , -1.0  , 1.0  ),
            (-1.0   , -1.0  , -1.0 ),

            (phi_i , phi  , 0.0   ),
            (phi_i , -phi  , 0.0  ),
            (-phi_i , phi  , 0.0  ),
            (-phi_i , -phi  , 0.0 ),
            
            (phi   , 0.0  , phi_i ),
            (phi   , 0.0  , -phi_i),
            (-phi  , 0.0  , phi_i ),
            (-phi  , 0.0  , -phi_i),
        ];

        let len = |(a0,a1,a2) : Point3d|   -> f64 {
            ((a0*a0) + (a1*a1) + (a2*a2)).sqrt()
        };
        let sub = |(a0,a1,a2): Point3d, (b0,b1,b2) : Point3d|   -> Point3d {
            (b0-a0 , b1-a1 , b2-a2)
        };

        let mut dodecaskeleton : Points3d = Vec::new();

        for v0 in dodecavertexes.clone() {
            let mut shortest_distance = 10.0;
            let mut closest_vertex : Point3d = v0;
            for v1 in dodecavertexes.clone() {
                let distance = len(sub(v0,v1));
                if distance < shortest_distance { 
                    shortest_distance = distance;
                    closest_vertex = v1;
                }
            } 
            dodecaskeleton.push(closest_vertex);
        }

        // let mut dodecahedron = Polygon::new(dodecaskeleton, Color::Green, (0.0,0.0,10.0), tm);
        // dodecahedron.render(&self); 
        // self.polygons.push(dodecahedron);
        
        // let mut dodecafaces : Vec<Polygon> = Vec::new();
        // for i in 0..12 {
        //     dodecafaces.push(
        //         Polygon::new(
        //             vec![
        //                 dodecavertexes.get(i % 20).unwrap().clone(),
        //                 dodecavertexes.get((i+3) % 20).unwrap().clone(),
        //                 dodecavertexes.get((i+6) % 20).unwrap().clone(),
        //                 dodecavertexes.get((i+9) % 20).unwrap().clone(),
        //                 dodecavertexes.get((i+12) % 20).unwrap().clone(),
        //             ], 
        //             Color::Green, 
        //             (0.0,0.0,10.0),tm 
        //         )
        //     )
        // }

        // let mut dodecahedron = Polyhedron::new(
        //     dodecafaces,
        // );

        // pentagram2.render(&self);
        // unit_cube.render(&self);
        // pentagram.render(&self);
        // onebyone.render(&self);

        // self.polygons.push(pentagram2);
        self.polygons.push(pentagram);
        // self.polygons.push(onebyone);

        // dodecahedron.render(&self);
        for p in unit_cube.polygons {
            self.polygons.push(p)
        }


        //remove things that are too close or behind the camera
        self.polygons.retain(|p| {
            p.center.2 + p.offset.2 + self.world.world_translation_z > 1.0
        });

        self.render_polygons();

        self.polygons.sort_by(|a,b| {   // to render in the correct order
            b.center.2.partial_cmp(&a.center.2.to_owned()).unwrap()
        });

        let datasets = self.polygons.iter().rev().map(|p|{
            p.as_dataset()
        }).collect();
        
        frame.render_widget(
            Chart::new(datasets)
                .block(Block::default().title(" 3T ").border_type(BorderType::Plain))
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

    fn render_polygons(&mut self) {
        (0..self.polygons.len()).for_each( |i | {
            self.polygons.get_mut(i).unwrap().render(self.world.clone());
        })
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

#[derive(Debug)]
struct Polygon{
    vertices: Points3d,
    points: Points3d,
    color: Color,
    offset: Point3d,
    transformation: Matrix,
    projection: Points2d,
    center /*of gravity*/: Point3d,
    q : Quaternion
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
    fn generate_sides_and_center(&mut self) -> Vec<Line> {
        let (c0, c1, c2) = self.center.borrow_mut();
        let mut sides : Vec<Line> = Vec::new();
        for i in 1..self.vertices.len() {
            let l = Line::new(
                self.vertices[i-1].clone(),
                self.vertices[i].clone(),
                self.color
            );
            let (l0, l1, l2) = l.center;
            *c0 += l0; *c1 += l1; *c2 += l2;
            sides.push(l);
        }
        let l = Line::new(
            self.vertices[self.vertices.len()-1].clone(),
            self.vertices[0].clone(),
            self.color
        );
        let (l0, l1, l2) = l.center;
        *c0 += l0; *c1 += l1; *c2 += l2;
        
        sides.push(l);
        
        let n = sides.len() as f64;
        *c0 /= n; *c1 /= n; *c2 /= n;
        
        sides
    }


    fn generate_points(&mut self) {
        let mut sides = self.generate_sides_and_center();
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

    fn render(&mut self, world: WorldMetrics){
        self.generate_points();
        self.transform(&world);
        self.project(&world);
    }
    
    fn project(&mut self, world: &WorldMetrics){
        let fov = PI/4.0; // 90deg
        let ez = 1.0/((fov/2.0).tan());
        self.projection = self.points.iter().map(|a| {
            project_point(a.clone(), (0.0, 0.0, ez), (world.camera_pitch,world.camera_yaw,0.0)) 
        }).collect()
    } 

    pub fn transform(&mut self, world: &WorldMetrics) {
        let q = &self.q;
        for i in 0..self.points.len(){
            let a = self.points.get(i).unwrap();

            let (offx, offy, offz) = &self.offset;

            let (xx,yy,zz) = q.rotate_point(a.into());

            let xx = tm[0][0]*x + tm[0][1]*y + tm[0][2]*z + offx + world.world_translation_x;  
            let yy = tm[1][0]*x + tm[1][1]*y + tm[1][2]*z + offy + world.world_translation_y;  
            let zz = tm[2][0]*x + tm[2][1]*y + tm[2][2]*z + offz + world.world_translation_z;  
            
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

    async fn render(&mut self, world: WorldMetrics) {
        let futures = async move {
            (0..self.polygons.len()).for_each( |i | {
                self.polygons.get_mut(i).unwrap().render(world.clone());
            })
        };

        futures.await;
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
    
    let (cx, cy, cz): Point3d = (tx.cos(),ty.cos(),tz.cos()); //not to be confused with the position of the camera (ccx, ccy, ccz)
    let (sx, sy, sz): Point3d = (tx.sin(),ty.sin(),tz.sin());

    let (ccx, ccy, ccz): Point3d = (0.0,0.0,0.0); // add this as a parameter later if necessary. It's only added to the code for easy migration.
    
    let x = ax-ccx;
    let y = ay-ccy;
    let z = az-ccz;

    let dx = cy * ((sz*y) + (cz*x)) - (sy*z);
    let dy = sx * ( (cy*z) + sy*( (sz*y) + (cz*x) ) ) + cx*( (cz*y) - (sz*x) );
    let dz = cx * ( (cy*z) + sy*( (sz*y) + (cz*x) ) ) - sx*( (cz*y) - (sz*x) );

    let bx = (ez/dz)*dx + ex;
    let by = (ez/dz)*dy + ey;
    
    (bx,by) // projected point
}

#[derive(Debug,Clone)]
pub struct WorldMetrics {
    pub camera_pitch: f64,
    pub camera_yaw: f64,
    pub camera_roll: f64,
    pub world_translation_x: f64,
    pub world_translation_y: f64,
    pub world_translation_z: f64,
}

impl Default for WorldMetrics {
    fn default() -> Self {
        WorldMetrics{
            camera_pitch: 0.0,
            camera_yaw: 0.0,
            camera_roll: 0.0,
            world_translation_x: 0.0,
            world_translation_y: 0.0,
            world_translation_z: 0.0,
        }
    }
}