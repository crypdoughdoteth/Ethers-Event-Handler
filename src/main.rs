use ethers::abi::Tokenizable;
use ethers::prelude::*;
use ethers::signers::LocalWallet;

use dotenv::dotenv;
use eyre::Result;

use ethers::contract::{Contract, EthEvent};
use ethers::core::{
    abi::Abi,
    types::{Address, U256},
};
use serde_json;
use ethers::providers::Middleware;

// Wrapper type for calling and handling events when you expect a given event to be emitted from the function you are calling on a smart contract
pub struct EventHandler<M, X, E>{
    provider: M, 
    contract: Contract<M>,
    fn_name: String,
    args: X,
    event: E,
}

impl <M: Middleware, X: Tokenizable + std::marker::Send + Clone, E: EthEvent> EventHandler<M, X, E> {
    //first we get a new Event Handler which takes provider, the contract instance, and function signature
    fn new(provider: M, contract: Contract<M>, fn_name: String, args: X, event: E) -> Self {
        Self {
            provider,
            contract,
            fn_name,
            args,
            event
        }
    }
    //then call eh_call on your new EventHandler
    async fn eh_call(self: Self) -> Result<Vec<E>, ContractError<M>> 
        where ethers::contract::ContractError<M>: From<<M as Middleware>::Error>{
        // get block number
        let block_number = &self.provider.get_block_number().await?;
        //build method
        self.contract.method::<_, X>(&self.fn_name, self.args)?.send().await?.confirmations(6).await.unwrap();
        //handle event given block_number and event E
        let res = self.contract
            .event::<E>()
            .from_block(block_number)
            .query()
            .await?;
        Ok(res)
    }
}


fn main() {
    todo!();
}
