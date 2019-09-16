use storm::*;
use crate::tetris_game::*;
use storm::cgmath::*;
use storm::math::AABB2D;

pub struct Label {
    pos: UIPos,
    width: u16,
    height: u16 ,
    sprite: Sprite,
    text: String,
    bounding_box: AABB2D,
}

impl Label {
    pub fn new(pos: UIPos, width: u16, height: u16, text: String) -> Label {
        
        let mut sprite = Sprite::default();
        sprite.size.x = width;
        sprite.size.y = height;

        sprite.pos.x = pos.x;
        sprite.pos.y = pos.y;
        sprite.color = storm::color::BLACK;

        let aabb = AABB2D::new(sprite.pos.x, sprite.pos.y, 
                               sprite.pos.x + width as f32, sprite.pos.y + height as f32);
        Label {
            pos,
            height,
            width,
            sprite,
            text,
            bounding_box: aabb        
        }
    }
}

impl UIElement for Label {
    fn draw(&mut self,  sprites: &mut Vec<Sprite>, texts: &mut Vec<Text>) {
        self.sprite.pos.x = self.pos.x;
        self.sprite.pos.y = self.pos.y;
        self.sprite.pos.z = 0.8;
    
        self.sprite.size.x = self.width;
        self.sprite.size.y = self.height;

        let mut text = Text::default();
        text.set_string(self.text.as_str());
        text.color = storm::color::WHITE;

        text.pos.x = self.pos.x + 17.5;
        text.pos.y = self.pos.y + 20.0;
        text.pos.z = 0.9;

        sprites.push(self.sprite);
        texts.push(text);
    }

    fn bounding_box(&self) -> AABB2D {
        let as_button : &Label = self as &Label;
        return as_button.bounding_box;
    }

    fn click_down(&mut self, _ : Vector2<f32>) {
    }

    fn click_up(&mut self, point : Vector2<f32>) -> bool {
        let bb = self.bounding_box();
        if bb.contains_point(&point) {
            return true;
        }
        return false;
    }
}