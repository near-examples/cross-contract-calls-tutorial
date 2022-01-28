use crate::*;

use near_sdk::{Gas, Balance};

const GAS_FOR_RESOLVE_FIRE: Gas = Gas(10_000_000_000_000);
const GAS_FOR_FIRE: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_FIRE.0);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Fleet {
    /*
        Fire a shot on the opposing fleet. 
        performs a CCC to the opposing fleet which returns the state of the shot.
        Invalid means the state should be reverted.
        Ensure:
            - game is in progress.
            - it is the current fleet's turn.
            - called by the current commander.
            - position hasn't been fired on before.
    */
    #[payable]
    pub fn fire(
        &mut self,
        position: String,
        fleet_id: AccountId,
    ) -> PromiseOrValue<bool> {
        //ensure the game is going on
        assert_eq!(self.in_progress, true, "cannot fire when game is over");
        //ensure it's your turn
        assert_eq!(self.current_turn, self.fleet_type, "Cannot fire when it's not your turn.");

        //get the sender and current player. Make sure the sender is the current commander who owns the fleet
        let sender = env::predecessor_account_id().to_string();
        let curr_player: String = if self.fleet_type == "pirates".to_string() { self.pirate_commander.as_ref().expect("no pirate commander set").to_string() } else { self.viking_commander.as_ref().expect("no viking commander set").to_string() };

        assert_eq!(sender, curr_player.to_string(), "Only the commanders playing can fire");

        //get the coordinate of the attack as an int
        let coordinate = convert_position_to_coords(&position);
       
        //set the current turn (will reset if unsuccessful)
        self.current_turn = if self.current_turn == "pirates".to_string() { "vikings".to_string() } else { "pirates".to_string() };
        
        // Initiating receiver's call and the callback
        ext_fleet_receiver::on_fire(
            coordinate,
            fleet_id.clone(), //contract account to make the call to
            NO_DEPOSIT, //attached deposit
            env::prepaid_gas() - GAS_FOR_FIRE, //attached GAS
        )
        //we then resolve the promise and call nft_resolve_transfer on our own contract
        .then(ext_self::resolve_fire(
            env::current_account_id(), //contract account to make the call to
            NO_DEPOSIT, //attached deposit
            GAS_FOR_RESOLVE_FIRE, //GAS attached to the call
        )).into()
    }

    /*
        Invoked by the opposing fleet once the fire function has been called.
        Ensure: 
            - called via CCC
            - game is active
            - position hasn't been fired on before
    */
    pub fn on_fire(
        &mut self,
        coordinate: u32
    ) -> String {
        let captain = env::signer_account_id();
        let messenger = env::predecessor_account_id();

        //ensure function called via cross contract call
        assert_ne!(captain.to_string(), messenger.to_string(), "Function must be invoked via a cross contract call");

        //check if the game is active
        assert_eq!(self.in_progress, true, "Cannot fire when the game is over.");

        //make sure the coordinate hasn't been fired on already
        let coordinate_state = &self.board_state[(coordinate - 1) as usize];
        if coordinate_state != &"empty".to_string() && coordinate_state != &"ship".to_string() {
            env::panic_str("Cannot fire on the same position twice")
        }

        //change the current turn
        self.current_turn = if self.current_turn == "pirates".to_string() { "vikings".to_string() } else { "pirates".to_string() };
        
        //get the expected caller (captain) by using the current turn. If pirates sent the call, we expect the signer to be the pirate commander
        let expected_captain: String = if self.current_turn == "pirates".to_string() { self.pirate_commander.as_ref().expect("pirate commander not set").to_string() } else { self.viking_commander.as_ref().expect("viking commander not set").to_string() };
        assert_eq!(expected_captain, captain.to_string(), "Only the opposing commander can fire");

        /*
            Check and see what the coordinate sent by the commander hit.
            Should be either a ship or an empty square.
        */
        if coordinate_state == &"ship".to_string() {
            //they hit a ship so we should set the board state to hit and log
            env::log_str("It's a hit!");
            self.board_state[(coordinate - 1) as usize] = "hit".to_string();

            //increment number of holes and check if game over
            self.num_holes += 1;
            if self.num_holes >= 2 {
                //set the winner to be logged and output
                let winner = if self.fleet_type == "pirates".to_string() {"vikings".to_string()} else {"pirates".to_string()};

                env::log_str(&format!("Ship Sank! Game Over - {} Win! To play again, call new_game function.", winner));
                //game is no longer active and we return sunk
                self.in_progress = false;
                return "sunk".to_string();
            }

            return "hit".to_string();
        } else {
            //set state to miss and return
            env::log_str("Missed your shot...");
            self.board_state[(coordinate - 1) as usize] = "miss".to_string();
            return "miss".to_string();
        }
    }

    /*
        Resolve the promise made to the enemy fleet once fired. 
        This should get the value returned by the enemy fleet
        and conditionally return state if invalid was returned.
    */
    #[private]
    pub fn resolve_fire(
        &mut self,
    ) -> String {
        //get the result from the promise
        let result = promise_result_as_success();

        //if the promise was not successful, revert the state and return invalid.
        if result.is_none() {
            env::log_str("Invalid turn - reverting your turn back.");
            self.current_turn = if self.current_turn == "pirates".to_string() { "vikings".to_string() } else { "pirates".to_string() };
            return "invalid".to_string();
        }

        //get the string value of the result
        let string_result: String = near_sdk::serde_json::from_slice::<String>(&result.unwrap()).expect("Cannot get string result");
        
        //set game over if sunk was returned
        if string_result == "sunk".to_string() {
            self.in_progress = false
        }
 
        //return the result
        string_result
    }
}