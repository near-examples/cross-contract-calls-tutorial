use crate::*;

//trait outlining the restrictions extension for the counter contract
pub trait Restrictions {
    //ensure that the calling account has access to modify the counter's state
    fn assert_access(&self);

    //allow a specific account access to modify the counter's state. This can only be called by the owner of the contract
    fn allow_user(&mut self, account_id: AccountId);

    ///deny a specific account access to modify the counter's state. This can only be called by the owner of the contract
    fn deny_user(&mut self, account_id: AccountId);
}

#[near_bindgen]
impl Restrictions for Counter {
    //Check and make sure that the calling account has access to modify the counter's state
    fn assert_access(&self) {
        require!(self.allow_list.contains(&env::predecessor_account_id()) == true, "Only users with access can modify the counter's state");
    }

    //allow a specific account access to modify the counter's state. This can only be called by the owner of the contract
    fn allow_user(&mut self, account_id: AccountId) {
        assert_eq!(
            &env::predecessor_account_id().to_string(),
            &self.owner_id.to_string(),
            "Owner's method"
        );

        self.allow_list.insert(&account_id);
    }

    //deny a specific account access to modify the counter's state. This can only be called by the owner of the contract
    fn deny_user(&mut self, account_id: AccountId) {
        assert_eq!(
            &env::predecessor_account_id().to_string(),
            &self.owner_id.to_string(),
            "Owner's method"
        );

        self.allow_list.remove(&account_id);
    }
}

