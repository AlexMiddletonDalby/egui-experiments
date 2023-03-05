use eframe::egui;
use egui_extras::RetainedImage;
use rand::Rng;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
            "Ironman gg",
        options,
        Box::new(|_cc| Box::new(IronmanGg::default())),
    )
}

#[derive(Clone)]
struct Character{
    name: String,
    is_selected: bool,
    image_index: i32,
}

impl Character{
    fn new (new_name: String, new_image_index: i32) -> Character{
        return Character {name: new_name, is_selected: true, image_index: new_image_index};
    }
}

struct IronmanGg {
    images: Vec<RetainedImage>,

    roster: Vec<Character>,
    requested_roster_size: i32,

    characters: Vec<Character>,
}

fn load_images() -> Vec<RetainedImage> {
    return vec![
        RetainedImage::from_image_bytes ("resources/dr-mario.png",
                                         include_bytes!("resources/dr-mario.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/mario.png",
                                         include_bytes!("resources/mario.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/luigi.png",
                                         include_bytes!("resources/luigi.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/bowser.png",
                                         include_bytes!("resources/bowser.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/peach.png",
                                         include_bytes!("resources/peach.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/yoshi.png",
                                         include_bytes!("resources/yoshi.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/dk.png",
                                         include_bytes!("resources/dk.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/captain-falcon.png",
                                         include_bytes!("resources/captain-falcon.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/ganondorf.png",
                                         include_bytes!("resources/ganondorf.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/falco.png",
                                         include_bytes!("resources/falco.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/fox.png",
                                         include_bytes!("resources/fox.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/ness.png",
                                         include_bytes!("resources/ness.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/ice-climbers.png",
                                         include_bytes!("resources/ice-climbers.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/kirby.png",
                                         include_bytes!("resources/kirby.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/samus.png",
                                         include_bytes!("resources/samus.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/zelda.png",
                                         include_bytes!("resources/zelda.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/link.png",
                                         include_bytes!("resources/link.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/young-link.png",
                                         include_bytes!("resources/young-link.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/pichu.png",
                                         include_bytes!("resources/pichu.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/pikachu.png",
                                         include_bytes!("resources/pikachu.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/jigglypuff.png",
                                         include_bytes!("resources/jigglypuff.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/mewtwo.png",
                                         include_bytes!("resources/mewtwo.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/mr-game-and-watch.png",
                                         include_bytes!("resources/mr-game-and-watch.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/marth.png",
                                         include_bytes!("resources/marth.png")).unwrap(),
        RetainedImage::from_image_bytes ("resources/roy.png",
                                         include_bytes!("resources/roy.png")).unwrap(),

    ];
}

impl Default for IronmanGg {
    fn default() -> Self {
        return Self {
            images: load_images(),

            roster: vec![],
            requested_roster_size: 1,
            characters: vec![Character::new("Dr. Mario".to_owned(), 0),
                             Character::new("Mario".to_owned(), 1),
                             Character::new("Luigi".to_owned(), 2),
                             Character::new("Bowser".to_owned(), 3),
                             Character::new("Peach".to_owned(), 4),
                             Character::new("Yoshi".to_owned(), 5),
                             Character::new("DK".to_owned(), 6),
                             Character::new("Captain Falcon".to_owned(), 7),
                             Character::new("Ganondorf".to_owned(), 8),
                             Character::new("Falco".to_owned(), 9),
                             Character::new("Fox".to_owned(), 10),
                             Character::new("Ness".to_owned(), 11),
                             Character::new("Ice Climbers".to_owned(), 12),
                             Character::new("Kirby".to_owned(), 13),
                             Character::new("Samus".to_owned(), 14),
                             Character::new("Zelda".to_owned(), 15),
                             Character::new("Link".to_owned(), 16),
                             Character::new("Young Link".to_owned(), 17),
                             Character::new("Pichu".to_owned(), 18),
                             Character::new("Pikachu".to_owned(), 19),
                             Character::new("Jigglypuff".to_owned(), 20),
                             Character::new("Mewtwo".to_owned(), 21),
                             Character::new("Mr. Game & Watch".to_owned(), 22),
                             Character::new("Marth".to_owned(), 23),
                             Character::new("Roy".to_owned(), 24)],
        };
    }
}

impl eframe::App for IronmanGg {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("Ironman Roster Generator");

            ui.horizontal_wrapped(|ui| {
                for character in &mut self.characters {
                    let image = self.images.get(character.image_index as usize).unwrap();
                    if ui.add(egui::ImageButton::new(image.texture_id(ctx), image.size_vec2())
                        .selected(character.is_selected))
                        .clicked() {
                            character.is_selected = !character.is_selected;
                        }
                }
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Roster size:");
                ui.add(egui::DragValue::new(&mut self.requested_roster_size).clamp_range(1..=self.characters.len()))
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked()
                {
                    self.roster.clear();

                    let mut selected_characters: Vec<Character> = vec![];

                    for character in &self.characters {
                        if character.is_selected {
                            selected_characters.push(character.clone());
                        }
                    }

                    if ! selected_characters.is_empty() {
                        for _ in 0..self.requested_roster_size {
                            let index = rand::thread_rng().gen_range(0..selected_characters.len());
                            self.roster.push(selected_characters[index].clone());
                        }
                    }
                }
            });

            ui.horizontal_wrapped(|ui| {
                for roster_character in &mut self.roster {
                    ui.label(&roster_character.name);
                }
            })
        });
    }
}
