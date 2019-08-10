pub struct BattleMove {
    pub number_of_moves: usize,
    pub affected_stat: String,//TODO: Turn this into an ENUM 
    pub whole_number_or_percentage: bool,//TODO: Turn this into an ENUM
    pub amount: u8//a percentage is out of 100
}

impl BattleMove {
    pub fn new(number_of_moves: usize,
                affected_stat: String,
                whole_number_or_percentage: bool,
                amount: u8) -> BattleMove {
        BattleMove {
            number_of_moves,
            affected_stat,
            whole_number_or_percentage,
            amount
        }
    }
}