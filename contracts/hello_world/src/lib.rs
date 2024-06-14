#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, vec, Address, Env, Symbol, Vec};


const ID: Symbol = symbol_short!("ID");

#[derive(Debug,Clone, PartialEq)]
#[contracttype]
pub struct Transaction {
    id: u32,
    sender: Address,
    receiver: Address,
    amount: i128,
    completed: bool
}

#[contract]
pub struct Stellpay;

#[contractimpl]
impl Stellpay {
    pub fn pay_amount(env: Env, sender: Address, receiver: Address, amount: i128) {
        
        receiver.require_auth();
        sender.require_auth();

        let mut id: u32 = env.storage().instance().get(&ID).unwrap_or(0);

        id += 1;

        let sender_token_client = token::Client::new(&env, &sender);

        let contract_token_client = env.current_contract_address();

        sender_token_client.transfer(&sender, &contract_token_client, &amount);

        let transasction = Transaction {
            id,
            sender,
            receiver,
            amount,
            completed: false
        };

        write_transcation(&env, &transasction);

        add_transaction_id(&env, &transasction);   
    }

    pub fn withdraw_amount(env: Env, id: u32) {
        let mut transaction: Transaction = env.storage().persistent().get(&id).unwrap();

        let receiver_token_client = token::Client::new(&env, &transaction.receiver);

        let contract = env.current_contract_address();

        receiver_token_client.transfer(&contract, &transaction.receiver, &transaction.amount);

        transaction.completed = true;

        write_transcation(&env, &transaction);
    }
}

fn write_transcation(env: &Env, transaction: &Transaction) {
    env.storage().persistent().set(&transaction.id, transaction);
}

fn add_transaction_id(env: &Env, transaction: &Transaction) {
    let mut ids: Vec<u32> = env.storage().persistent().get(&transaction.receiver).unwrap_or(vec![env]); 

    ids.push_back(transaction.id);

    env.storage().persistent().set(&transaction.receiver, &ids)

}

mod test;