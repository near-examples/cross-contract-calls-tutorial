use near_sdk::ext_contract;

#[ext_contract(ext_fleet_receiver)]
trait ExternalFleetReceiver {
    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn on_fire(
        &mut self,
        coordinate: u32
    ) -> Promise;

    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn on_create_new_game(
        &mut self,
        //who you wanna play against
        desired_opponent: AccountId,
        //set of initial ship positions as strings IE - [A3, A4]
        initial_ship_positions: Vec<String>,
    ) -> Promise;
}

#[ext_contract(ext_self)]
trait FleetResolver {
    /*
        resolves the promise of the cross contract call to the receiver contract
        this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn resolve_fire(
        &mut self,
        coordinate: u32
    ) -> bool;

    fn resolve_create_new_game(
        &mut self,
        //who you wanna play against
        desired_opponent: AccountId,
        //set of initial ship positions as strings IE - [A3, A4]
        initial_ship_positions: Vec<String>,
    ) -> bool;
}