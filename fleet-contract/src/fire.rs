use crate::*;

use near_sdk::{Gas, Balance};

const GAS_FOR_RESOLVE_FIRE: Gas = Gas(10_000_000_000_000);
const GAS_FOR_FIRE: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_FIRE.0);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Fleet {
    //implementation of the transfer call method. This will transfer the NFT and call a method on the reciver_id contract
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
            coordinate,
            env::current_account_id(), //contract account to make the call to
            NO_DEPOSIT, //attached deposit
            GAS_FOR_RESOLVE_FIRE, //GAS attached to the call
        )).into()
    }

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


        let coordinate_state = &self.board_state[(coordinate - 1) as usize];
        //make sure the coordinate hasn't been fired on already
        if coordinate_state != &"empty".to_string() && coordinate_state != &"ship".to_string() {
            env::panic_str("Cannot fire on the same position twice")
        }

        self.current_turn = if self.current_turn == "pirates".to_string() { "vikings".to_string() } else { "pirates".to_string() };
        let expected_captain: String = if self.current_turn == "pirates".to_string() { self.pirate_commander.as_ref().expect("pirate commander not set").to_string() } else { self.viking_commander.as_ref().expect("viking commander not set").to_string() };
        
        assert_eq!(expected_captain, captain.to_string(), "Only the opposing commander can fire");

        //if it's a ship, set the state to hit and return
        if coordinate_state == &"ship".to_string() {
            env::log_str("It's a hit!");
            self.board_state[(coordinate - 1) as usize] = "hit".to_string();

            //increment number of holes and check if game over
            self.num_holes += 1;
            if self.num_holes >= 2 {
                let winner = if self.fleet_type == "pirates".to_string() {"vikings".to_string()} else {"pirates".to_string()};

                env::log_str(&format!("Ship Sank! Game Over - {} Win! To play again, call new_game function.", winner));
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

    #[private]
    pub fn resolve_fire(
        &mut self,
        coordinate: u32
    ) -> String {
        let result = promise_result_as_success();

        if result.is_none() {
            env::log_str("Invalid turn - reverting your turn back.");
            self.current_turn = if self.current_turn == "pirates".to_string() { "vikings".to_string() } else { "pirates".to_string() };
            return "invalid".to_string();
        }

        let string_result: String = near_sdk::serde_json::from_slice::<String>(&result.unwrap()).expect("Cannot get string result");
        
        //set game over if sunk was returned
        if string_result == "sunk".to_string() {
            self.in_progress = false
        }

        string_result
    }
}