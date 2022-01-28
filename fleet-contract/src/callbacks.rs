use near_sdk::ext_contract;

#[ext_contract(ext_fleet_receiver)]
trait ExternalFleetReceiver {
    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn on_fire(
        &mut self,
        coordinate: u32
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
    ) -> bool;
}