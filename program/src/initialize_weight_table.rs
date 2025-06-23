use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::{load_system_account, load_system_program};
use jito_restaking_core::ncn::Ncn;
use ncn_program_core::{
    account_payer::AccountPayer, constants::MAX_REALLOC_BYTES, epoch_marker::EpochMarker,
    epoch_state::EpochState, vault_registry::VaultRegistry, weight_table::WeightTable,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Initializes the weight table for a specific epoch, which will store the importance weights of different tokens.
///
/// ### Parameters:
/// - `epoch`: The target epoch
///
/// ### Accounts:
/// 1. `[]` epoch_marker: Marker account to prevent duplicate initialization
/// 2. `[]` epoch_state: The epoch state account for the target epoch
/// 3. `[]` vault_registry: The vault registry containing registered vaults
/// 4. `[]` ncn: The NCN account
/// 5. `[writable]` weight_table: The weight table account to initialize
/// 6. `[writable, signer]` account_payer: Account paying for initialization
/// 7. `[]` system_program: Solana System Program
pub fn process_initialize_weight_table(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    epoch: u64,
) -> ProgramResult {
    msg!("Starting initialize_weight_table instruction");
    let [epoch_marker, epoch_state, vault_registry, ncn, weight_table, account_payer, system_program] =
        accounts
    else {
        msg!("Error: Not enough account keys provided");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Loading and validating epoch state for epoch: {}", epoch);
    EpochState::load_and_check_is_closing(program_id, epoch_state, ncn.key, epoch, false)?;
    msg!("Loading vault registry account");
    VaultRegistry::load(program_id, vault_registry, ncn.key, false)?;
    msg!("Loading NCN account");
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    msg!("Loading account payer");
    AccountPayer::load(program_id, account_payer, ncn.key, true)?;
    msg!("Checking epoch marker does not exist for epoch: {}", epoch);
    EpochMarker::check_dne(program_id, epoch_marker, ncn.key, epoch)?;

    msg!("Loading system account for weight table");
    load_system_account(weight_table, true)?;
    msg!("Loading system program");
    load_system_program(system_program)?;

    msg!("Getting vault counts");
    let vault_count = {
        let ncn_data = ncn.data.borrow();
        let ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;
        let count = ncn.vault_count();
        msg!("NCN vault count: {}", count);
        count
    };

    let vault_registry_count = {
        let vault_registry_data = vault_registry.data.borrow();
        let vault_registry = VaultRegistry::try_from_slice_unchecked(&vault_registry_data)?;
        let count = vault_registry.vault_count();
        msg!("Vault registry count: {}", count);
        count
    };

    if vault_count != vault_registry_count {
        msg!(
            "Error: Vault count mismatch - NCN: {}, Registry: {}",
            vault_count,
            vault_registry_count
        );
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Deriving weight table PDA");
    let (weight_table_pubkey, weight_table_bump, mut weight_table_seeds) =
        WeightTable::find_program_address(program_id, ncn.key, epoch);
    weight_table_seeds.push(vec![weight_table_bump]);

    if weight_table_pubkey.ne(weight_table.key) {
        msg!("Error: Incorrect weight table PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing Weight Table {} for NCN: {} at epoch: {}",
        weight_table.key,
        ncn.key,
        epoch
    );
    AccountPayer::pay_and_create_account(
        program_id,
        ncn.key,
        account_payer,
        weight_table,
        system_program,
        program_id,
        MAX_REALLOC_BYTES as usize,
        &weight_table_seeds,
    )?;

    Ok(())
}
