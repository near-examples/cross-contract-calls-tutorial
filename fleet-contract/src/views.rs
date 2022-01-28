use crate::*;

#[near_bindgen]
impl Fleet {
    //Check whose turn it is to play
    pub fn get_cur_turn(self) -> String {
        self.current_turn
    }

    //Check who is the current player
    pub fn get_cur_commanders(self) -> String {
        format!("Pirate Commander --> {:?} Viking Commander --> {:?}", self.pirate_commander, self.viking_commander)
    }

    //Check who is the current player
    pub fn get_board_state(self) -> Vec<String> {
        self.board_state
    }
}
