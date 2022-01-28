use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue, promise_result_as_success
};

pub use crate::fire::*;
pub use crate::internal::*;
pub use crate::callbacks::*;

mod internal;
mod callbacks;
mod views; 
mod fire;
mod new;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Fleet {
    //contract owner
    pub owner_id: String,

    //keep track of the state of the current board
    pub board_state: Vec<String>,

    //keep track of the pirate fleet commander (to lock two players in a game)
    pub pirate_commander: Option<String>,

    //keep track of the viking fleet commander (to lock two players in a game)
    pub viking_commander: Option<String>,

    //check if the game is in progress
    pub in_progress: bool,

    //keep track of whose turn it is - either pirates or vikings
    pub current_turn: String,

    //keep track of the number of holes in our ship
    pub num_holes: u8,

    //is the fleet a pirate or viking fleet?
    pub fleet_type: String,
}

#[near_bindgen]
impl Fleet {
    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, current_player: AccountId, fleet_type: String) -> Self {
        //ensure user inputs either vikings or pirates
        if fleet_type != "vikings".to_string() && fleet_type != "pirates".to_string() {
            env::panic_str("must input a valid fleet type");
        }

        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //set the owner_id field equal to the passed in owner_id. 
            owner_id: owner_id.to_string(),

            //set the initial board state to all empty. We will initialize ships after
            board_state: vec!["empty".to_string(); 16],
            
            //set the pirate commander if the fleet is pirate
            pirate_commander: if fleet_type == "pirates".to_string() { None } else { Some(current_player.to_string()) },
            
            //set the viking commander if the fleet is vikings
            viking_commander: if fleet_type == "vikings".to_string() { None } else { Some(current_player.to_string()) },

            //set in progress to true
            in_progress: false,

            //default the current turn to pirates
            current_turn: "pirates".to_string(),

            //set the ship to have 0 holes to start
            num_holes: 0,

            fleet_type
        };

        //return the Contract object
        this
    }
}