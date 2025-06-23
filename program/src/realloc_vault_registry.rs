use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::loader::load_system_program;
use jito_restaking_core::ncn::Ncn;
use ncn_program_core::{
    account_payer::AccountPayer, config::Config as NcnConfig, utils::get_new_size,
    vault_registry::VaultRegistry,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Resizes the vault registry account to accommodate more vaults.
///
/// ### Accounts:
/// 1. `[]` config: NCN configuration account
/// 2. `[writable]` vault_registry: The vault registry account to resize
/// 3. `[]` ncn: The NCN account
/// 4. `[writable, signer]` account_payer: Account paying for the reallocation
/// 5. `[]` system_program: Solana System Program
pub fn process_realloc_vault_registry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    msg!("Starting realloc_vault_registry instruction");

    let [ncn_config, vault_registry, ncn, account_payer, system_program] = accounts else {
        msg!("Error: Not enough account keys provided");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Loading system program");
    load_system_program(system_program)?;

    msg!("Loading NCN account");
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;

    msg!("Loading NCN config account");
    NcnConfig::load(program_id, ncn_config, ncn.key, false)?;

    msg!("Loading account payer");
    AccountPayer::load(program_id, account_payer, ncn.key, true)?;

    msg!("Deriving vault registry PDA");
    let (vault_registry_pda, vault_registry_bump, mut vault_registry_seeds) =
        VaultRegistry::find_program_address(program_id, ncn.key);
    vault_registry_seeds.push(vec![vault_registry_bump]);

    if vault_registry_pda != *vault_registry.key {
        msg!("Error: Vault registry account is not at the correct PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    let vault_registry_size = vault_registry.data_len();
    if vault_registry_size < VaultRegistry::SIZE {
        let new_size = get_new_size(vault_registry_size, VaultRegistry::SIZE)?;
        msg!(
            "Reallocating vault registry from {} bytes to {} bytes",
            vault_registry_size,
            new_size
        );

        AccountPayer::pay_and_realloc(
            program_id,
            ncn.key,
            account_payer,
            vault_registry,
            new_size,
        )?;
    } else {
        msg!("Vault registry size is sufficient, no reallocation needed");
    }

    let should_initialize = vault_registry.data_len() >= VaultRegistry::SIZE
        && vault_registry.try_borrow_data()?[0] != VaultRegistry::DISCRIMINATOR;

    if should_initialize {
        msg!("Initializing vault registry account");
        let mut vault_registry_data = vault_registry.try_borrow_mut_data()?;
        vault_registry_data[0] = VaultRegistry::DISCRIMINATOR;
        let vault_registry_account =
            VaultRegistry::try_from_slice_unchecked_mut(&mut vault_registry_data)?;
        vault_registry_account.initialize(ncn.key, vault_registry_bump);
    } else {
        msg!("Vault registry already initialized, skipping initialization");
    }

    Ok(())
}
