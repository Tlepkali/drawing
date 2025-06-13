#![cfg(feature = "iced-example")]
use drawing::seat_map::SeatMap;
use drawing::geometrical_shapes::Point;
use iced::widget::canvas::{self, Canvas, Frame, Geometry};
use iced::{executor, Application, Command, Element, Settings, Theme};
use iced::{Point as IPoint, Rectangle, Size, Color};

pub fn main() -> iced::Result {
    SeatMapApp::run(Settings::default())
}

struct SeatMapApp {
    seat_map: SeatMap,
    cache: canvas::Cache,
}

impl Application for SeatMapApp {
    type Message = ();
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                seat_map: SeatMap::new(5, 5, &Point::new(10, 10), 40, 5),
                cache: canvas::Cache::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Seat map (iced)")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Canvas::new(self).into()
    }
}

impl canvas::Program<()> for SeatMapApp {
    type State = ();

    fn draw(&self, _state: &Self::State, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            for seat in &self.seat_map.seats {
                let x = seat.rect.a.x as f32;
                let y = seat.rect.a.y as f32;
                let size = (seat.rect.b.x - seat.rect.a.x) as f32;
                let color = match seat.tariff {
                    0 => Color::from_rgb(0.0, 1.0, 0.0),
                    1 => Color::from_rgb(0.0, 0.0, 1.0),
                    _ => Color::from_rgb(1.0, 0.0, 0.0),
                };
                let rect = canvas::Path::rectangle(IPoint::new(x, y), Size::new(size, size));
                frame.fill(&rect, color);
            }
        });
        vec![geometry]
    }
}

