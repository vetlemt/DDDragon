use std::error;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint};
use tui::style::{Color, Style};
use tui::symbols;
use tui::terminal::Frame;
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, BorderType, Chart, Dataset, GraphType, Paragraph};

use crate::geometry::{Point, Line, Polygon};

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


