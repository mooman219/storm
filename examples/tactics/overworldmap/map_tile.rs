pub enum TileType {
    Nothing,//An empty tile with no encounter
    Battle,//this will be a battle encounter
    Shop,//any kind of merchant
    PersonEncounter//This will be like a dialogue encounter, you could pick up party members this way
}

impl TileType {
    pub fn draw(&self) -> char{
        match self {
            &TileType::Battle => {
                return 'B';
            },
            &TileType::Nothing => {
                return 'N';
            },
            &TileType::PersonEncounter => {
                return 'E';
            },
            &TileType::Shop => {
                return 'S'
            }
        }
    }
}

pub struct MapTile {
    tile_type: TileType,
    has_been_flipped: bool,//tiles start without the player knowing what they are, so we keep track of that
    has_party_on_it: bool
}

impl MapTile {
    pub fn new(tile_type: TileType) -> MapTile {
        MapTile {
            tile_type,
            has_been_flipped: false,
            has_party_on_it: false
        }
    }

    pub fn flip_tile(&mut self) {
        self.has_been_flipped = true;
    }

    pub fn party_on_tile(&mut self) {
        self.has_party_on_it = true;
    }

    pub fn party_left_tile(&mut self) {
        self.has_party_on_it = false;
    }

    //we call it draw, but it will just report to the overworld_map what value it is, and that will draw it
    pub fn draw(&self) -> char {
        if self.has_party_on_it {
            'P'
        }
        else if self.has_been_flipped {
            self.tile_type.draw()
        }
        else {
            //this is the FULL BLOCK █ unicode character
            '\u{2588}'
        }
    }

}