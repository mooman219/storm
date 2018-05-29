use tactics::party_manager::Caravan;

//this view should display all characters that a caravan has
//display the most basic information about them
//allow for the player to swap them in and out of the active roster
pub struct PartyView {}

impl PartyView {
    pub fn new() -> PartyView {
        PartyView {}
    }

    pub fn draw(&self, caravan: &Caravan) {
        let party = caravan.get_party();

        println!("The Party Roster");
        let mut count = 0;
        for (_, v) in party {
            println!("-=========================-");
            if caravan.is_lead(&v.name) {
                println!("{} Lead : {}", count, v.name);
            } else {
                println!("{} Party Member: {}", count, v.name);
            }

            count += 1;
        }
        println!("-=========================-");
    }
}
