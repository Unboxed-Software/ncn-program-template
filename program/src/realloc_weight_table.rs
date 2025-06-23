use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::loader::load_system_program;
use jito_restaking_core::ncn::Ncn;
use ncn_program_core::{
    account_payer::AccountPayer, config::Config as NcnConfig, epoch_state::EpochState,
    utils::get_new_size, vault_registry::VaultRegistry, weight_table::WeightTable,
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

/// Resizes the weight table account to accommodate more entries.
///
/// ### Parameters:
/// - `epoch`: The target epoch
///
/// ### Accounts:
/// 1. `[writable]` epoch_state: The epoch state account for the target epoch
/// 2. `[]` vault_registry: The vault registry containing registered vaults
/// 3. `[]` ncn: The NCN account
/// 4. `[writable]` weight_table: The weight table to resize
/// 5. `[writable, signer]` account_payer: Account paying for reallocation
/// 6. `[]` system_program: Solana System Program
pub fn process_realloc_weight_table(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    epoch: u64,
) -> ProgramResult {
    msg!(
        "Starting realloc_weight_table instruction for epoch {}",
        epoch
    );

    let [epoch_state, ncn_config, weight_table, ncn, vault_registry, account_payer, system_program] =
        accounts
    else {
        msg!("Error: Not enough account keys provided");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Loading system program");
    load_system_program(system_program)?;
    msg!("Loading NCN account");
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    msg!("Loading epoch state");
    EpochState::load(program_id, epoch_state, ncn.key, epoch, true)?;
    msg!("Loading NCN config");
    NcnConfig::load(program_id, ncn_config, ncn.key, false)?;
    msg!("Loading vault registry");
    VaultRegistry::load(program_id, vault_registry, ncn.key, false)?;
    msg!("Loading account payer");
    AccountPayer::load(program_id, account_payer, ncn.key, true)?;

    let (weight_table_pda, weight_table_bump, _) =
        WeightTable::find_program_address(program_id, ncn.key, epoch);

    if weight_table_pda != *weight_table.key {
        msg!("Error: Weight table account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    if weight_table.data_len() < WeightTable::SIZE {
        let new_size = get_new_size(weight_table.data_len(), WeightTable::SIZE)?;
        msg!(
            "Reallocating weight table from {} bytes to {} bytes",
            weight_table.data_len(),
            new_size
        );
        AccountPayer::pay_and_realloc(program_id, ncn.key, account_payer, weight_table, new_size)?;
    } else {
        msg!("Weight table size is sufficient, no reallocation needed");
    }

    let should_initialize = weight_table.data_len() >= WeightTable::SIZE
        && weight_table.try_borrow_data()?[0] != WeightTable::DISCRIMINATOR;

    if should_initialize {
        msg!("Initializing weight table...");
        let vault_registry_data = vault_registry.data.borrow();
        let vault_registry = VaultRegistry::try_from_slice_unchecked(&vault_registry_data)?;

        let vault_count = vault_registry.vault_count();
        let st_mint_count = vault_registry.st_mint_count();
        let vault_entries = vault_registry.get_vault_entries();
        let mint_entries = vault_registry.get_mint_entries();

        let mut weight_table_data = weight_table.try_borrow_mut_data()?;
        weight_table_data[0] = WeightTable::DISCRIMINATOR;
        let weight_table_account =
            WeightTable::try_from_slice_unchecked_mut(&mut weight_table_data)?;

        weight_table_account.initialize(
            ncn.key,
            epoch,
            Clock::get()?.slot,
            vault_count,
            weight_table_bump,
            vault_entries,
            mint_entries,
        )?;

        // Update Epoch State
        {
            msg!("Updating epoch state...");
            let mut epoch_state_data = epoch_state.try_borrow_mut_data()?;
            let epoch_state_account =
                EpochState::try_from_slice_unchecked_mut(&mut epoch_state_data)?;
            epoch_state_account.update_realloc_weight_table(vault_count, st_mint_count as u64);
        }
    } else {
        msg!("Weight table already initialized, skipping initialization");
    }

    Ok(())
}
