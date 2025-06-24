use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::loader::{load_system_account, load_system_program};
use jito_restaking_core::ncn::Ncn;
use ncn_program_core::{
    account_payer::AccountPayer, config::Config, epoch_marker::EpochMarker, epoch_state::EpochState,
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

/// Initializes the state for a specific epoch, creating a tracking mechanism for that epoch's lifecycle.
///
/// ### Parameters:
/// - `epoch`: The target epoch
///
/// ### Accounts:
/// 1. `[writable]` epoch_marker: Marker account to prevent duplicate initialization
/// 2. `[writable]` epoch_state: The epoch state account to initialize
/// 3. `[]` ncn: The NCN account
/// 4. `[writable, signer]` account_payer: Account paying for initialization
/// 5. `[]` system_program: Solana System Program
pub fn process_initialize_epoch_state(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    epoch: u64,
) -> ProgramResult {
    let [epoch_marker, epoch_state, config, ncn, account_payer, system_program] = accounts else {
        msg!("Error: Not enough account keys provided");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let current_epoch = Clock::get()?.epoch;
    if epoch > current_epoch {
        msg!("Error: Cannot initialize epoch state for future epoch");
        return Err(ProgramError::InvalidArgument);
    }

    load_system_account(epoch_state, true)?;
    load_system_program(system_program)?;

    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    Config::load(program_id, config, ncn.key, false)?;
    AccountPayer::load(program_id, account_payer, ncn.key, true)?;
    EpochMarker::check_dne(program_id, epoch_marker, ncn.key, epoch)?;

    let config_data = config.try_borrow_data()?;
    let config_account = Config::try_from_slice_unchecked(&config_data)?;
    if config_account.starting_valid_epoch() > epoch {
        msg!("Error: This epoch is before the starting_valid_epoch");
        return Err(ProgramError::InvalidArgument);
    }

    let (epoch_state_pda, epoch_state_bump, mut epoch_state_seeds) =
        EpochState::find_program_address(program_id, ncn.key, epoch);
    epoch_state_seeds.push(vec![epoch_state_bump]);

    if epoch_state_pda != *epoch_state.key {
        msg!("Error: Invalid epoch state PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    AccountPayer::pay_and_create_account(
        program_id,
        ncn.key,
        account_payer,
        epoch_state,
        system_program,
        program_id,
        EpochState::SIZE,
        &epoch_state_seeds,
    )?;

    let mut epoch_state_data = epoch_state.try_borrow_mut_data()?;
    epoch_state_data[0] = EpochState::DISCRIMINATOR;
    let epoch_state_account = EpochState::try_from_slice_unchecked_mut(&mut epoch_state_data)?;

    let current_slot = Clock::get()?.slot;
    epoch_state_account.initialize(ncn.key, epoch, epoch_state_bump, current_slot);

    epoch_state_account.update_realloc_epoch_state();

    Ok(())
}
