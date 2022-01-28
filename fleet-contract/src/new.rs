use crate::*;

#[near_bindgen]
impl Fleet {
    //create a new game if the current one is over already
    pub fn create_new_game(
        &mut self,
        //who you wanna play against
        desired_opponent: AccountId,
        //set of initial ship positions as strings IE - [A3, A4]
        initial_ship_positions: Vec<String>,
    ) {
        let current_player = env::signer_account_id();
        //make sure only 2 positions are being used
        assert_eq!(self.in_progress, false, "Cannot create a new game if one is in progress");
        
        //reset board state
        self.board_state = vec!["empty".to_string(); 16];
        self.internal_populate_board_state(initial_ship_positions);

        //set the current turn to be pirates
        self.current_turn = "pirates".to_string();
        //set the pirate and viking commanders
        self.pirate_commander = if self.fleet_type == "pirates".to_string() { Some(current_player.to_string()) } else { Some(desired_opponent.to_string()) };
        self.viking_commander = if self.fleet_type == "vikings".to_string() { Some(current_player.to_string()) } else { Some(desired_opponent.to_string()) };
        
        //set in progress to true. If cross contract call fails, we reset this to false.
        self.in_progress = true;
    }
}