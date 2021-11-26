use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::instruction::Instruction;
use solana_program::instruction::AccountMeta;
use solana_program::program::{invoke_signed};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
 /* transfer nftttt */
 pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    let source_account_info = next_account_info(accounts_iter)?;//source
    let owner_account_info = next_account_info(accounts_iter)?;//owner
    let dist_account_info = next_account_info(accounts_iter)?;//dis
    let authority_account_info = next_account_info(accounts_iter)?;// authority cros
    let addres_program_account_info = next_account_info(accounts_iter)?;
    let program_id_account_info = next_account_info(accounts_iter)?;//spltoken
    let nonce =_instruction_data[0];
    let amount :u64=1;
    let expected_allocated_key =
    Pubkey::create_program_address(&[&addres_program_account_info.key.to_bytes()[..32], &[nonce]], program_id)?;
    if *authority_account_info .key != expected_allocated_key {
    // allocated key does not match the derived address
    return Err(ProgramError::InvalidArgument);
    }
     let mut buf = Vec::new();
    let instruction:u8 = 3;//transfer
    let mut vac_accounts = Vec::new();
    buf.push(instruction);

    buf.extend_from_slice(&amount.to_le_bytes());
    vac_accounts.push(AccountMeta::new(*source_account_info.key, false));
    vac_accounts.push(AccountMeta::new(*dist_account_info.key, false));
    vac_accounts.push(AccountMeta::new(*owner_account_info.key, true));
    vac_accounts.push(AccountMeta::new_readonly(*program_id_account_info.key, false));
    
     let ix = Instruction {
        accounts:vac_accounts,
        program_id: *program_id_account_info.key,
        data: buf,
   };
    invoke_signed(&ix, 
    &[source_account_info.clone(),dist_account_info.clone(),owner_account_info.clone(),
    program_id_account_info.clone(),addres_program_account_info.clone() ],
    &[&[&addres_program_account_info.key.to_bytes()[..32], &[nonce]]]
    )? ; 
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
