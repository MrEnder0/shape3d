use eframe::egui;
use rand::Rng;
use std::collections::HashMap;

pub struct ColorCache {
    // ID -> Color
    pub colors: HashMap<usize, egui::Color32>,
}

impl ColorCache {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }

    pub fn get_color(&mut self, id: usize) -> egui::Color32 {
        if let std::collections::hash_map::Entry::Vacant(e) = self.colors.entry(id) {
            let color = gen_color();
            e.insert(color);
            color
        } else {
            self.colors[&id]
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            colors: self.colors.clone(),
        }
    }
}

fn gen_color() -> egui::Color32 {
    let mut rng = rand::thread_rng();

    egui::Color32::from_rgb(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
}

pub fn mix_colors(colors: Vec<egui::Color32>) -> egui::Color32 {
    let mut r: usize = 0;
    let mut g: usize = 0;
    let mut b: usize = 0;

    for color in colors.iter() {
        r += color.r() as usize;
        g += color.g() as usize;
        b += color.b() as usize;
    }

    egui::Color32::from_rgb(
        (r / colors.len()) as u8,
        (g / colors.len()) as u8,
        (b / colors.len()) as u8,
    )
}
