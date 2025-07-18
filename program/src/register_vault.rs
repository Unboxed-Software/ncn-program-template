use jito_bytemuck::AccountDeserialize;
use jito_restaking_core::{ncn::Ncn, ncn_vault_ticket::NcnVaultTicket};
use jito_vault_core::vault::Vault;
use ncn_program_core::{config::Config, vault_registry::VaultRegistry};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

/// Registers a vault in the vault registry to participate in the consensus mechanism.
///
/// ### Accounts:
/// 1. `[]` config: NCN configuration account
/// 2. `[writable]` vault_registry: The vault registry to update
/// 3. `[]` ncn: The NCN account
/// 4. `[]` vault: The vault to register
/// 5. `[]` ncn_vault_ticket: The connection between NCN and vault from the restaking program
pub fn process_register_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [config, vault_registry, ncn, vault, ncn_vault_ticket] = accounts else {
        msg!("Error: Not enough account keys provided");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config, ncn.key, false)?;
    VaultRegistry::load(program_id, vault_registry, ncn.key, true)?;
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    Vault::load(&jito_vault_program::id(), vault, false)?;
    NcnVaultTicket::load(
        &jito_restaking_program::id(),
        ncn_vault_ticket,
        ncn,
        vault,
        false,
    )?;

    let clock = Clock::get()?;
    let slot = clock.slot;

    let mut vault_registry_data = vault_registry.try_borrow_mut_data()?;
    let vault_registry = VaultRegistry::try_from_slice_unchecked_mut(&mut vault_registry_data)?;

    let vault_data = vault.data.borrow();
    let vault_account = Vault::try_from_slice_unchecked(&vault_data)?;

    if !vault_registry.has_st_mint(&vault_account.supported_mint) {
        msg!("Error: Supported mint not registered");
        return Err(ProgramError::InvalidAccountData);
    }

    vault_registry.register_vault(
        vault.key,
        &vault_account.supported_mint,
        vault_account.vault_index(),
        slot,
    )?;

    Ok(())
}
