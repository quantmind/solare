

pub trait MM {
    // add liquidity to the market maker
    fn add(&mut self, token_a_amount: u128, token_b_amount: u128);
    // withdraw from the liquity pool
    fn withdraw(&mut self, token_a_amount: u128, token_b_amount: u128);
}

// A uniswap v2 market maker
pub struct UniSwap;


impl MM for UniSwap {
    fn add(&mut self, _token_a_amount: u128, _token_b_amount: u128) {

    }

    fn withdraw(&mut self, _token_a_amount: u128, _token_b_amount: u128) {

    }
}
