use std::collections::HashMap;

use eframe::egui::{Align2, Color32, FontId, Frame, Ui, Vec2, pos2};
use statrs::function::gamma::gamma;

pub(crate) struct AnnealApp {
    cities: Vec<Vec2>,
    visit_order: Vec<usize>,
    attempts: usize,
    temperature: f64,
    record: HashMap<Vec<usize>, f64>,
    paused: bool,
}

const AREA_WIDTH: f32 = 500.0;
const AREA_HEIGHT: f32 = 500.0;
const OFFSET: Vec2 = Vec2::new(10.0, 10.0);
const TICKS_PER_UPDATE: usize = 10;

impl AnnealApp {
    pub fn new() -> Self {
        let mut cities = vec![];
        for _ in 0..100 {
            cities.push(Vec2::new(
                rand::random::<f32>() * AREA_WIDTH,
                rand::random::<f32>() * AREA_HEIGHT,
            ));
        }
        let visit_order = (0..cities.len()).collect();
        Self {
            cities,
            visit_order,
            attempts: 0,
            temperature: 100.0,
            record: HashMap::new(),
            paused: false,
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
            self.record.insert(self.visit_order.clone(), after_distance);
        }

        self.attempts += 1;
        self.temperature *= 0.9999;
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

        let total_distance = self.total_distance();

        let attempts = self.attempts;
        let percentage = (self.record.len() as f64 / gamma(self.cities.len() as f64)) * 100.0;
        let temperature = self.temperature;

        painter.text(
            pos2(10., AREA_HEIGHT),
            Align2::LEFT_TOP,
            format!(
                r#"Attempts: {attempts}, {percentage}% of total solution space
Temperature: {temperature}
Total distance: {}"#,
                total_distance
            ),
            FontId::proportional(16.),
            Color32::BLACK,
        );
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
    }

    fn reset(&mut self) {
        let mut cities = vec![];
        for _ in 0..100 {
            cities.push(Vec2::new(
                rand::random::<f32>() * AREA_WIDTH,
                rand::random::<f32>() * AREA_HEIGHT,
            ));
        }
        let visit_order = (0..cities.len()).collect();
        self.cities = cities;
        self.visit_order = visit_order;
        self.temperature = 1000.;
        self.attempts = 0;
        self.record.clear();
    }
}

impl eframe::App for AnnealApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.paused {
            ctx.request_repaint_after(std::time::Duration::from_millis(20));
            for _ in 0..TICKS_PER_UPDATE {
                self.tick();
            }
        }

        eframe::egui::SidePanel::right("side_panel")
            .min_width(200.)
            .show(ctx, |ui| self.ui_panel(ui));

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.render(ui);
            });
        });
    }
}
