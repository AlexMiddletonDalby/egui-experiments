use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
            "Ironman gg",
        options,
        Box::new(|_cc| Box::new(IronmanGg::default())),
    )
}

struct Character{
    name: String,
    is_selected: bool
}

impl Character{
    fn new (new_name: String) -> Character{
        return Character {name: new_name, is_selected: false};
    }
}

struct IronmanGg {
    characters: Vec<Character>
}

impl Default for IronmanGg {
    fn default() -> Self {
        return Self {
            characters: vec![Character::new("Dr. Mario".to_owned()),
                             Character::new("Mario".to_owned())]
        }
    }
}

impl eframe::App for IronmanGg {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("Ironman");
            ui.horizontal(|ui| {
                for character in &mut self.characters {
                    ui.checkbox(&mut character.is_selected, &character.name);
                }
            })
        });
    }
}
