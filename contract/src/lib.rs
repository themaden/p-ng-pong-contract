#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait PingPongContract {
    #[init]
    fn init(&self, ping_amount: BigUint, duration_in_seconds: u64, accepted_payment_token_id: TokenIdentifier) {
        self.ping_amount().set(&ping_amount);
        self.duration_in_seconds().set(duration_in_seconds);
        self.accepted_payment_token_id().set(&accepted_payment_token_id);
    }

    #[payable("*")]
    #[endpoint]
    fn ping(&self) {
        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier == self.accepted_payment_token_id().get(),
            "Invalid payment token"
        );
        require!(
            payment.amount == self.ping_amount().get(),
            "Invalid ping amount"
        );
        require!(
            !self.did_user_ping(&self.blockchain().get_caller()).get(),
            "User already pinged"
        );

        let current_timestamp = self.blockchain().get_block_timestamp();
        self.user_ping_timestamp(&self.blockchain().get_caller())
            .set(current_timestamp);
        self.did_user_ping(&self.blockchain().get_caller()).set(true);
    }

    #[endpoint]
    fn pong(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            self.did_user_ping(&caller).get(),
            "User must ping first"
        );

        let ping_timestamp = self.user_ping_timestamp(&caller).get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp >= ping_timestamp + self.duration_in_seconds().get(),
            "Pong not yet available"
        );

        let ping_amount = self.ping_amount().get();
        self.send().direct_esdt(
            &caller,
            &self.accepted_payment_token_id().get(),
            0,
            &ping_amount,
        );

        self.did_user_ping(&caller).clear();
        self.user_ping_timestamp(&caller).clear();

        self.pong_event(&caller, &ping_amount);
    }

    #[view(didUserPing)]
    fn did_user_ping(&self, user: &ManagedAddress) -> SingleValueMapper<bool>;

    #[view(getPongEnableTimestamp)]
    fn get_pong_enable_timestamp(&self, user: &ManagedAddress) -> u64 {
        self.user_ping_timestamp(user).get() + self.duration_in_seconds().get()
    }

    #[view(getTimeToPong)]
    fn get_time_to_pong(&self, user: &ManagedAddress) -> SCResult<u64> {
        let pong_enable_timestamp = self.get_pong_enable_timestamp(user);
        let current_timestamp = self.blockchain().get_block_timestamp();
        if current_timestamp >= pong_enable_timestamp {
            Ok(0)
        } else {
            Ok(pong_enable_timestamp - current_timestamp)
        }
    }

    #[view(getAcceptedPaymentToken)]
    #[storage_mapper("acceptedPaymentToken")]
    fn accepted_payment_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPingAmount)]
    #[storage_mapper("pingAmount")]
    fn ping_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getDurationTimestamp)]
    #[storage_mapper("durationTimestamp")]
    fn duration_in_seconds(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("userPingTimestamp")]
    fn user_ping_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[event("pong")]
    fn pong_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}