use std::collections::HashMap;
use tactics::party_manager::Character;

pub struct Caravan {
    lead: String,
    party_members: HashMap<String, Character>,
    active_roster: Vec<String>,
}

impl Caravan {
    //you must have a lead character for a caravan, they will
    //also automatically added to the active roster
    pub fn new(party_lead: Character) -> Caravan {
        //the party is set of character that a player has to field at any momemnt
        let mut party = HashMap::new();

        //we enforce now empty parties, a parties is about half of a game, the other being the map
        let lead_name = party_lead.name.clone();
        party.insert(party_lead.name.clone(), party_lead);

        Caravan {
            party_members: party,
            lead: lead_name.clone(), //we keep this as a record for later
            active_roster: vec![lead_name],
        }
    }

    pub fn add_party_member(&mut self, name: String, character: Character) {
        self.party_members.insert(name, character);
    }

    //this does not have any locks against adding someone in the active rostor who is already there,
    //would be best if this was only call with sanatized input
    pub fn add_to_active_roster(&mut self, name: String) {
        self.active_roster.push(name);
    }

    pub fn get_active_roster(&self) -> &Vec<String> {
        &self.active_roster
    }

    pub fn is_party_member(&self, name: &String) -> bool {
        self.party_members.contains_key(name)
    }

    //this will look for the index of the name passed in and then remove it from the active roster
    //list
    //TODO: avoid having to do a Binary Search to find the index of the character we are looking to remove
    pub fn remove_character_from_active_roster(&mut self, name: &String) {
        if self.active_roster.contains(name) {
            let result = self.active_roster.binary_search(name);
            match result {
                Ok(index) => {
                    self.active_roster.remove(index);
                },
                Err(_e) => {},
            }
        }
    }

    pub fn get_party(&self) -> &HashMap<String, Character> {
        return &self.party_members;
    }

    //this is for when we only need to compare is a string is the same as the leads name
    //saves having to create a whole new string to just return and compare later with
    pub fn is_lead(&self, name: &String) -> bool {
        *name == self.lead
    }

    pub fn get_lead(&self) -> String {
        self.lead.clone()
    }

    //this will copy the characters in the active roster, THIS IS VERY EXSPENSIVE
    //TODO: NOT HAVE THIS BE EXSPENSIVE
    pub fn create_clone_of_active_roster(&self) -> HashMap<String, Character> {
        let mut party_copy = HashMap::new();
        for name in &self.active_roster {
            party_copy.insert(name.clone(), self.party_members.get(name).unwrap().clone());
        }
        party_copy
    }
}
