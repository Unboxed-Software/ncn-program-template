use crate::{
    getters::{
        get_account, get_all_operators_in_ncn, get_all_vaults_in_ncn, get_is_epoch_completed,
        get_ncn_program_config,
    },
    handler::CliHandler,
};
use anyhow::{anyhow, Ok, Result};
use jito_bytemuck::AccountDeserialize;

use ncn_program_core::{
    ballot_box::BallotBox,
    config::Config as NCNProgramConfig,
    epoch_snapshot::{EpochSnapshot, OperatorSnapshot},
    epoch_state::{EpochState, State},
    vault_registry::VaultRegistry,
    weight_table::WeightTable,
};
use solana_sdk::pubkey::Pubkey;

#[derive(Default)]
pub struct KeeperState {
    pub epoch: u64,
    pub ncn: Pubkey,
    pub vaults: Vec<Pubkey>,
    pub operators: Vec<Pubkey>,
    pub ncn_program_config_address: Pubkey,
    pub vault_registry_address: Pubkey,
    pub epoch_state_address: Pubkey,
    pub weight_table_address: Pubkey,
    pub epoch_snapshot_address: Pubkey,
    pub operator_snapshots_address: Vec<Pubkey>,
    pub ballot_box_address: Pubkey,
    pub epoch_state: Option<Box<EpochState>>,
    pub current_state: Option<State>,
    pub is_epoch_completed: bool,
}

impl KeeperState {
    pub async fn fetch(&mut self, handler: &CliHandler, epoch: u64) -> Result<()> {
        let ncn = *handler.ncn()?;
        self.ncn = ncn;

        let vaults = get_all_vaults_in_ncn(handler).await?;
        self.vaults = vaults;

        let operators = get_all_operators_in_ncn(handler).await?;
        self.operators = operators;

        let (ncn_program_config_address, _, _) =
            NCNProgramConfig::find_program_address(&handler.ncn_program_id, &ncn);
        self.ncn_program_config_address = ncn_program_config_address;

        let (vault_registry_address, _, _) =
            VaultRegistry::find_program_address(&handler.ncn_program_id, &ncn);
        self.vault_registry_address = vault_registry_address;

        let (epoch_state_address, _, _) =
            EpochState::find_program_address(&handler.ncn_program_id, &ncn, epoch);
        self.epoch_state_address = epoch_state_address;

        let (weight_table_address, _, _) =
            WeightTable::find_program_address(&handler.ncn_program_id, &ncn, epoch);
        self.weight_table_address = weight_table_address;

        let (epoch_snapshot_address, _, _) =
            EpochSnapshot::find_program_address(&handler.ncn_program_id, &ncn, epoch);
        self.epoch_snapshot_address = epoch_snapshot_address;

        for operator in self.operators.iter() {
            let (operator_snapshot_address, _, _) = OperatorSnapshot::find_program_address(
                &handler.ncn_program_id,
                operator,
                &ncn,
                epoch,
            );
            self.operator_snapshots_address
                .push(operator_snapshot_address);
        }

        let (ballot_box_address, _, _) =
            BallotBox::find_program_address(&handler.ncn_program_id, &ncn, epoch);
        self.ballot_box_address = ballot_box_address;

        self.update_epoch_state(handler).await?;

        // To ensure that the state is fetched for the correct epoch
        self.epoch = epoch;

        Ok(())
    }

    pub async fn update_epoch_state(&mut self, handler: &CliHandler) -> Result<()> {
        let is_epoch_completed = get_is_epoch_completed(handler, self.epoch).await?;
        if is_epoch_completed {
            self.is_epoch_completed = true;
        } else {
            self.is_epoch_completed = false;
        }

        let raw_account = get_account(handler, &self.epoch_state_address).await?;

        if raw_account.is_none() {
            self.epoch_state = None;
            return Ok(());
        }

        let raw_account = raw_account.unwrap();

        if raw_account.data.len() < EpochState::SIZE {
            self.epoch_state = None;
            return Ok(());
        }

        let account = Box::new(*EpochState::try_from_slice_unchecked(
            raw_account.data.as_slice(),
        )?);
        self.epoch_state = Some(account);

        self.update_current_state(handler).await?;

        Ok(())
    }

    pub async fn ncn_program_config(
        &self,
        handler: &CliHandler,
    ) -> Result<Option<NCNProgramConfig>> {
        let raw_account = get_account(handler, &self.ncn_program_config_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = NCNProgramConfig::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn vault_registry(&self, handler: &CliHandler) -> Result<Option<VaultRegistry>> {
        let raw_account = get_account(handler, &self.vault_registry_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = VaultRegistry::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn weight_table(&self, handler: &CliHandler) -> Result<Option<WeightTable>> {
        let raw_account = get_account(handler, &self.weight_table_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = WeightTable::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn epoch_snapshot(&self, handler: &CliHandler) -> Result<Option<EpochSnapshot>> {
        let raw_account = get_account(handler, &self.epoch_snapshot_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();

            let account = EpochSnapshot::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn operator_snapshot(
        &self,
        handler: &CliHandler,
        operator_index: usize,
    ) -> Result<Option<OperatorSnapshot>> {
        let raw_account =
            get_account(handler, &self.operator_snapshots_address[operator_index]).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = OperatorSnapshot::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn ballot_box(&self, handler: &CliHandler) -> Result<Option<Box<BallotBox>>> {
        let raw_account = get_account(handler, &self.ballot_box_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = Box::new(*BallotBox::try_from_slice_unchecked(
                raw_account.data.as_slice(),
            )?);
            Ok(Some(account))
        }
    }

    pub fn epoch_state(&self) -> Result<&EpochState> {
        self.epoch_state
            .as_ref()
            .map(|boxed| boxed.as_ref())
            .ok_or_else(|| anyhow!("Epoch state does not exist"))
    }

    pub async fn update_current_state(&mut self, handler: &CliHandler) -> Result<()> {
        let rpc_client = handler.rpc_client();
        let current_slot = rpc_client.get_epoch_info().await?.absolute_slot;
        let epoch_schedule = rpc_client.get_epoch_schedule().await?;

        let (valid_slots_after_consensus, epochs_after_consensus_before_close) = {
            let config = get_ncn_program_config(handler).await?;
            (
                config.valid_slots_after_consensus(),
                config.epochs_after_consensus_before_close(),
            )
        };

        let epoch_state = self.epoch_state()?;

        let state = epoch_state.current_state(
            &epoch_schedule,
            valid_slots_after_consensus,
            epochs_after_consensus_before_close,
            current_slot,
        );

        self.current_state = Some(state?);

        Ok(())
    }

    pub fn current_state(&self) -> Result<State> {
        let state = self
            .current_state
            .as_ref()
            .ok_or_else(|| anyhow!("Current state does not exist"))?;

        Ok(*state)
    }
}
