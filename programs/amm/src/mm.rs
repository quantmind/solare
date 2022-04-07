

pub trait MM {
    fn trade(&mut self, ctx: Context<Trade>) -> Result<()>;
}

pub struct UniSwap;
