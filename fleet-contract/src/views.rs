use crate::*;

#[near_bindgen]
impl Fleet {
    //Check whose turn it is to play
    pub fn get_cur_turn(self) -> String {
        self.current_turn
    }

    //Check who the current commanders are
    pub fn get_cur_commanders(self) -> String {
        format!("Pirate Commander --> {:?} Viking Commander --> {:?}", self.pirate_commander, self.viking_commander)
    }

    //Check the current state of the board
    pub fn get_board_state(self) -> Vec<String> {
        self.board_state
    }
}
