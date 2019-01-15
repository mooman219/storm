pub struct BattlefieldTile {
    character_name: Option<String>,
}

impl BattlefieldTile {
    pub fn new() -> BattlefieldTile {
        BattlefieldTile { character_name: None }
    }

    //we denote empty as not having a character on it
    pub fn is_empty(&self) -> bool {
        self.character_name.is_none()
    }

    //be careful of usage, this proforms an alloations
    pub fn get_character(&self) -> Option<String> {
        self.character_name.clone()
    }

    //this will try to place the character on the tile, if they do not make it
    pub fn attempt_place_character_on_tile(&mut self, name: String) -> bool {
        if self.is_empty() {
            self.character_name = Some(name);
            return true;
        }
        false
    }

    //this will zero out the character name on this tile, returning it if the tile was not empty to start with
    pub fn remove_character(&mut self) -> Option<String> {
        let hold = self.character_name.clone();
        self.character_name = None;
        hold
    }

    pub fn get_icon(&self) -> char {
        match self.character_name.as_ref() {
            Some(character_name) => {
                //We want the first character in the name of the character
                //this assume none zero length character names
                return character_name.chars().next().unwrap();
            },
            None => {
                //'e' is the empty tile character
                return '\u{2592}';
            },
        }
    }
}
