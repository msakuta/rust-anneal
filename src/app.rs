use eframe::egui::{self, Color32, Frame, Ui, Vec2};
use egui_plot::{Legend, Line, PlotPoints};

pub(crate) struct AnnealApp {
    cities: Vec<Vec2>,
    visit_order: Vec<usize>,
    iter: usize,
    num_cities: usize,
    temperature: f64,
    starting_temperature: f64,
    cooling_rate: f64,
    record: Vec<f64>,
    paused: bool,
    iter_per_frame: usize,
}

pub(crate) const AREA_WIDTH: f32 = 500.0;
pub(crate) const AREA_HEIGHT: f32 = 500.0;
pub(crate) const AREA_MARGIN: f32 = 10.0;
pub(crate) const SIDE_PANEL_WIDTH: f32 = 300.;
pub(crate) const BOTTOM_PLOT_HEIGHT: f32 = 150.;
const OFFSET: Vec2 = Vec2::new(AREA_MARGIN, AREA_MARGIN * 2.0);

impl AnnealApp {
    pub fn new() -> Self {
        let num_cities = 100;
        let mut cities = vec![];
        for _ in 0..num_cities {
            cities.push(Vec2::new(
                rand::random::<f32>() * AREA_WIDTH,
                rand::random::<f32>() * AREA_HEIGHT,
            ));
        }
        let visit_order = (0..cities.len()).collect();
        Self {
            cities,
            visit_order,
            iter: 0,
            num_cities,
            temperature: 100.0,
            starting_temperature: 100.,
            cooling_rate: 0.0001,
            record: vec![],
            paused: false,
            iter_per_frame: 10,
        }
    }

    fn tick(&mut self) {
        let prev_distance = self.total_distance();
        let idx = rand::random_range(0..self.visit_order.len());
        let mut idx2 = rand::random_range(0..self.visit_order.len() - 1);
        if idx2 >= idx {
            idx2 += 1;
        }
        self.visit_order.swap(idx, idx2);
        let after_distance = self.total_distance();
        let swap = if after_distance > prev_distance {
            let delta = after_distance - prev_distance;
            let prob = (-(delta / self.temperature)).exp();
            prob > rand::random()
        } else {
            true
        };

        if !swap {
            self.visit_order.swap(idx, idx2);
        } else {
            self.record.push(after_distance);
        }

        self.iter += 1;
        self.temperature *= 1. - self.cooling_rate;
    }

    fn total_distance(&self) -> f64 {
        self.visit_order.windows(2).fold(0.0, |acc, pair| {
            acc + (self.cities[pair[0]] - self.cities[pair[1]]).length() as f64
        })
    }

    fn render(&self, ui: &mut Ui) {
        let (_response, painter) =
            ui.allocate_painter(ui.available_size(), eframe::egui::Sense::hover());

        for city in &self.cities {
            painter.circle_filled(
                city.to_pos2() + OFFSET,
                5.0,
                eframe::egui::Color32::from_black_alpha(255),
            );
        }
        let points = self
            .visit_order
            .iter()
            .map(|idx| self.cities[*idx].to_pos2() + OFFSET)
            .collect::<Vec<_>>();
        painter.line(points, (2., Color32::from_rgb(255, 0, 0)));
    }

    fn ui_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                self.reset();
            }

            let paused_label = if self.paused { "Unpause" } else { "Pause" };
            if ui.button(paused_label).clicked() {
                self.paused = !self.paused;
            }
        });

        ui.label(format!("Iter: {}", self.iter));
        ui.label(format!("Temperature: {:.3}", self.temperature));
        ui.label(format!("Total distance: {:.3}", self.total_distance()));

        ui.horizontal(|ui| {
            ui.label("Number of cities:");
            const MAX_CITIES: usize = 1000;
            ui.add(egui::Slider::new(&mut self.num_cities, 2..=MAX_CITIES));
        });

        ui.horizontal(|ui| {
            ui.label("Iterations per frame:");
            const MAX_BATCHES: usize = 50;
            ui.add(egui::Slider::new(&mut self.iter_per_frame, 1..=MAX_BATCHES));
        });

        ui.horizontal(|ui| {
            ui.label("Starting temperature:");
            const MAX_TEMPERATURE: f64 = 1000.;
            ui.add(egui::Slider::new(
                &mut self.starting_temperature,
                (0.)..=MAX_TEMPERATURE,
            ));
        });

        ui.horizontal(|ui| {
            ui.label("Cooling rate:");
            const MAX_COOLING_RATE: f64 = 0.05;
            ui.add(egui::Slider::new(
                &mut self.cooling_rate,
                (0.)..=MAX_COOLING_RATE,
            ));
        });
    }

    fn reset(&mut self) {
        let mut cities = vec![];
        for _ in 0..self.num_cities {
            cities.push(Vec2::new(
                rand::random::<f32>() * AREA_WIDTH,
                rand::random::<f32>() * AREA_HEIGHT,
            ));
        }
        let visit_order = (0..self.num_cities).collect();
        self.cities = cities;
        self.visit_order = visit_order;
        self.temperature = self.starting_temperature;
        self.iter = 0;
        self.record.clear();
    }
}

impl eframe::App for AnnealApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.paused {
            ctx.request_repaint_after(std::time::Duration::from_millis(20));
            for _ in 0..self.iter_per_frame {
                self.tick();
            }
        }

        eframe::egui::SidePanel::right("side_panel")
            .min_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| self.ui_panel(ui));

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.render(ui);
            });
        });

        egui::TopBottomPanel::bottom("weight_plot")
            .resizable(true)
            .min_height(BOTTOM_PLOT_HEIGHT)
            .default_height(BOTTOM_PLOT_HEIGHT)
            .show(ctx, |ui| {
                let plot = egui_plot::Plot::new("plot");
                plot.legend(Legend::default()).show(ui, |plot_ui| {
                    let points: PlotPoints = self
                        .record
                        .iter()
                        .enumerate()
                        .map(|(t, v)| [t as f64, *v])
                        .collect();
                    let line = Line::new("Total distance", points).color(
                        eframe::egui::Color32::from_rgb((200) as u8, (200) as u8, (100) as u8),
                    );
                    plot_ui.line(line);
                });
            });
    }
}
