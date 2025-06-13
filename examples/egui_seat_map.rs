#![cfg(feature = "egui-example")]
use drawing::seat_map::SeatMap;
use drawing::geometrical_shapes::Point;
use eframe::{egui, App, Frame, NativeOptions};

struct SeatMapApp {
    seat_map: SeatMap,
}

impl Default for SeatMapApp {
    fn default() -> Self {
        Self {
            seat_map: SeatMap::new(5, 5, &Point::new(10, 10), 40, 5),
        }
    }
}

impl App for SeatMapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for seat in &self.seat_map.seats {
                let x = seat.rect.a.x as f32;
                let y = seat.rect.a.y as f32;
                let size = (seat.rect.b.x - seat.rect.a.x) as f32;
                let color = match seat.tariff {
                    0 => egui::Color32::GREEN,
                    1 => egui::Color32::BLUE,
                    _ => egui::Color32::RED,
                };
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(size, size)),
                    0.0,
                    color,
                );
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Seat map (egui)",
        NativeOptions::default(),
        Box::new(|_cc| Box::<SeatMapApp>::default()),
    )
}
