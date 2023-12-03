use eframe::egui;

pub fn id_to_color(id: usize) -> egui::Color32 {
    match id {
        0 => egui::Color32::from_rgb(255, 0, 0),
        1 => egui::Color32::from_rgb(0, 255, 0),
        2 => egui::Color32::from_rgb(0, 0, 255),
        3 => egui::Color32::from_rgb(255, 255, 0),
        4 => egui::Color32::from_rgb(255, 0, 255),
        5 => egui::Color32::from_rgb(0, 255, 255),
        6 => egui::Color32::from_rgb(255, 255, 255),
        7 => egui::Color32::from_rgb(255, 128, 0),
        _ => egui::Rgba::TRANSPARENT.into(),
    }
}