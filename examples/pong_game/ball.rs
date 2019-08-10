use storm::*;

//Writing this for record keepign
pub struct Ball {
    pub ball_token: SpriteDescription,
}

impl Ball {
    pub fn new(ball_token: SpriteDescription) -> Ball {
        Ball {
            ball_token,
        }
    }
}
