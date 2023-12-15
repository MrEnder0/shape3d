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
        8 => egui::Color32::from_rgb(255, 0, 128),
        9 => egui::Color32::from_rgb(128, 255, 0),
        10 => egui::Color32::from_rgb(0, 255, 128),
        11 => egui::Color32::from_rgb(128, 0, 255),
        12 => egui::Color32::from_rgb(0, 128, 255),
        13 => egui::Color32::from_rgb(128, 128, 128),
        14 => egui::Color32::from_rgb(128, 128, 0),
        15 => egui::Color32::from_rgb(128, 0, 128),
        16 => egui::Color32::from_rgb(0, 128, 128),
        17 => egui::Color32::from_rgb(0, 0, 128),
        18 => egui::Color32::from_rgb(0, 128, 0),
        19 => egui::Color32::from_rgb(128, 0, 0),
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
