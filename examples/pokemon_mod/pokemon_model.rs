use crate::pokemon_mod::BattleMove;

pub struct Pokemon {
    pub health: usize,
    pub name: String,
    pub attack: usize,
    pub battle_moves: [BattleMove; 1]
}
impl Pokemon {
    pub fn new(health: usize, name: String, attack: usize, battle_moves: [BattleMove; 1]) -> Pokemon {
        Pokemon {
            health,
            name,
            attack,
            battle_moves
        }
    }
}