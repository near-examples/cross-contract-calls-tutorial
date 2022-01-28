use near_sdk::ext_contract;

#[ext_contract(ext_fleet_receiver)]
trait ExternalFleetReceiver {
    //Method stored on the enemy fleet that is called in the fire function
    fn on_fire(
        &mut self,
        coordinate: u32
    ) -> Promise;
}

#[ext_contract(ext_self)]
trait FleetResolver {
    /*
        resolves the promise of the CCC to the enemy fleet as a part of the fire function
    */
    fn resolve_fire(
        &mut self,
    ) -> bool;
}