use crate::*;

#[ext_contract(ext_self)]
trait ContractResolvers {
    //Resolves the promise of the cross contract call to the receiver contract for when increment is called 
    fn resolve_increment(
        &mut self,
        //we introduce an authorized ID for logging the transfer event
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        //we introduce the approval map so we can keep track of what the approvals were before the transfer
        approved_account_ids: HashMap<AccountId, u64>,
        //we introduce a memo for logging the transfer event
        memo: Option<String>,
    ) -> bool;
}

#[near_bindgen]
impl Contract {

    #[private]
    fn resolve_increment(&mut self) -> i8 {
        // check promise result
		if promise_result_as_success().is_none() { 
            log!("CCC wasn't successful. Reverting state.");
            self.val -= 1;
        }
        log!("CCC was successful");
        return self.val;
    }

	/*
        private method used to resolve the promise when calling nft_transfer_payout. This will take the payout object and 
        check to see if it's authentic and there's no problems. If everything is fine, it will pay the accounts. If there's a problem,
        it will refund the buyer for the price. 
    */
    #[private]
    pub fn resolve_offer(
        &mut self,
        maker_id: AccountId,
		taker_id: AccountId,
		token_id: String,
		contract_id: AccountId,
		offer_amount: U128,
		updated_at: u64,
        payout_amount: U128,
		market_amount: Balance,
    ) -> U128 {
        let mut valid_payout_object = true; 

        

		// get standard payout data from nft_transfer payout promise
		let Payout{ mut payout } = near_sdk::serde_json::from_slice::<Payout>(&result).unwrap_or_else(|_| {
            valid_payout_object = false;
            env::log_str("not a valid payout object. Sending taker full offer amount.");
            Payout{payout: HashMap::new()}
        });

        //we'll check if length of the payout object is > 10 or it's empty. In either case, we return None
        if payout.len() > 10 || payout.is_empty() {
            valid_payout_object = false;
            env::log_str(  "Cannot have more than 10 royalties. Sending taker full offer amount.");
        }
        
        //start with the remainder equal to the offer amount.
        let mut remainder = payout_amount.0;
        
        //loop through the payout and subtract the values from the remainder. 
        for &value in payout.values() {
            //checked sub checks for overflow or any errors and returns None if there are problems
            remainder = remainder.checked_sub(value.0).unwrap_or_else(|| {
                valid_payout_object = false;
                if valid_payout_object != false {
                    env::log_str("Payout object resulted in a payout larger than offer amount. Sending taker full offer amount.");
                }
                0
            });
        }

        //if invalid payout object, refund the taker.
        if valid_payout_object == false {
            payout = HashMap::from([(taker_id.clone(), payout_amount)]);
        }
        
        // NEAR payouts
        for (receiver_id, amount) in payout {
            Promise::new(receiver_id).transfer(amount.0);
        }
		
        // Log the serialized json.
        env::log_str(&EventLog {
            // The data related with the event stored in a vector.
            event: EventLogVariant::ResolveOffer(OfferLog {
                contract_id,	
				token_id,
				maker_id,
				taker_id,
				amount: offer_amount,
				updated_at,
            }),
        }.to_string());

		//increment market balance if all went well.
		self.market_balance += market_amount;

        //return the amount payed out
        payout_amount
    }

	//withdraw callback to ensure that the promise was successful when withdrawing the market balance
    #[private]
	pub fn on_withdraw_balance(&mut self, prev_balance: Balance) {
		if is_promise_success() {
			return
		}
		self.market_balance = prev_balance;
		env::log_str("Unexpected error when withdrawing market balance.");
	}

	//withdraw storage callback to ensure that the promise was successful when withdrawing storage amounts for users
    #[private]
	pub fn on_withdraw_offer_storage(&mut self, owner_id: AccountId, prev_storage_count: u64) {
		if is_promise_success() {
			return
		}
		self.offer_storage_by_owner_id.insert(&owner_id, &prev_storage_count);
		env::log_str("Unexpected error when withdrawing offer storage.");
	}
}