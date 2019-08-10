use crate::pokemon_mod::Pokemon;

pub struct Battle {
    fighting_pokemon: [Pokemon;2],
    order: usize
}

impl Battle {
    pub fn new(fighting_pokemon: [Pokemon; 2]) -> Battle {
        Battle {
            fighting_pokemon,
            order: 0
        }
    }

    pub fn tick_battle(&mut self)  {
        self.print_pokemon_action_options();

        self.order += 1;
    }

    pub fn print_pokemon_action_options(&self) {
        let current_pokemon = &self.fighting_pokemon[self.order % 2];
        for mv in current_pokemon.battle_moves.iter() {
            println!("{} Move as {} moves left", mv.affected_stat, mv.number_of_moves);
        }
    }
}