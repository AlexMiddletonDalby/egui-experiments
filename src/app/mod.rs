mod character;
mod images;

use egui_extras::RetainedImage;
use rand::Rng;

use character::Character;

pub struct IronmanGg {
    images: Vec<RetainedImage>,

    team_a_roster: Vec<Character>,
    team_b_roster: Vec<Character>,
    requested_roster_size: i32,

    characters: Vec<Character>,
}

impl Default for IronmanGg {
    fn default() -> Self {
        return Self {
            images: images::load(),

            team_a_roster: vec![],
            team_b_roster: vec![],
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
                ui.add(egui::DragValue::new(&mut self.requested_roster_size).clamp_range(1..=self.characters.len()));

                if ui.button("Generate").clicked()
                {
                    self.team_a_roster.clear();
                    self.team_b_roster.clear();

                    let mut selected_characters: Vec<Character> = vec![];

                    for character in &self.characters {
                        if character.is_selected {
                            selected_characters.push(character.clone());
                        }
                    }

                    if ! selected_characters.is_empty() {
                        for _ in 0..self.requested_roster_size {
                            let team_a_index = rand::thread_rng().gen_range(0..selected_characters.len());
                            self.team_a_roster.push(selected_characters[team_a_index].clone());

                            let team_b_index = rand::thread_rng().gen_range(0..selected_characters.len());
                            self.team_b_roster.push(selected_characters[team_b_index].clone());
                        }
                    }
                }
            });

            ui.add_space(20.0);

            ui.heading("Team A:");
            ui.horizontal_wrapped(|ui| {
                for roster_character in &mut self.team_a_roster {
                    let image = self.images.get(roster_character.image_index as usize).unwrap();
                    ui.add(egui::Image::new(image.texture_id(ctx), image.size_vec2()));
                }
            });

            ui.heading("Team B:");
            ui.horizontal_wrapped(|ui| {
                for roster_character in &mut self.team_b_roster {
                    let image = self.images.get(roster_character.image_index as usize).unwrap();
                    ui.add(egui::Image::new(image.texture_id(ctx), image.size_vec2()));
                }
            });
        });
    }
}
