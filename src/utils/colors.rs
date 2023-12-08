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