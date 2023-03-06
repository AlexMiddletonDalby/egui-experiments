#[derive(Clone)]
pub struct Character{
    pub name: String,
    pub is_selected: bool,
    pub image_index: i32,
}

impl Character{
    pub fn new (new_name: String, new_image_index: i32) -> Character{
        return Character {name: new_name, is_selected: true, image_index: new_image_index};
    }
}