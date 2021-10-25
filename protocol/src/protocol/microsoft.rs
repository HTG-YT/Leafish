use crate::protocol::Error;
use crate::protocol::login::{Account, AccountImpl, AccountType};

pub struct MicrosoftAccount {}

impl AccountImpl for MicrosoftAccount {
    fn login(&self, username: &str, password: &str, token: &str) -> Result<Account, Error> {
        todo!()
    }

    fn join_server(&self, account: &Account, server_id: &str, shared_key: &[u8], public_key: &[u8]) -> Result<(), Error> {
        todo!()
    }

    fn refresh(&self, account: Account, token: &str) -> Result<Account, Error> {
        todo!()
    }

    fn append_head_img_data(&self, account: &mut Account) -> Result<(), Error> {
        todo!()
    }
}