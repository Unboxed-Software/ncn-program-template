use std::fmt::{Debug, Formatter};

use jito_restaking_core::{config::Config, ncn_vault_ticket::NcnVaultTicket};
use ncn_program_core::{
    ballot_box::{BallotBox, WeatherStatus},
    constants::WEIGHT,
    epoch_snapshot::{EpochSnapshot, OperatorSnapshot},
    epoch_state::EpochState,
    weight_table::WeightTable,
};
use solana_program::{clock::Clock, native_token::sol_to_lamports, pubkey::Pubkey};
use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account,
    clock::DEFAULT_SLOTS_PER_EPOCH,
    epoch_schedule::EpochSchedule,
    signature::{Keypair, Signer},
};

use super::{ncn_program_client::NCNProgramClient, restaking_client::NcnRoot};
use crate::fixtures::{
    restaking_client::{OperatorRoot, RestakingProgramClient},
    vault_client::{VaultProgramClient, VaultRoot},
    TestResult,
};

/// Represents a complete NCN setup for testing purposes,
/// including the NCN itself, associated operators, and vaults.
pub struct TestNcn {
    pub ncn_root: NcnRoot,
    pub operators: Vec<OperatorRoot>,
    pub vaults: Vec<VaultRoot>,
}

/// Represents a single node within the test NCN setup,
/// detailing its connections and delegation status.
#[allow(dead_code)]
pub struct TestNcnNode {
    pub ncn_root: NcnRoot,
    pub operator_root: OperatorRoot,
    pub vault_root: VaultRoot,

    pub ncn_vault_connected: bool,
    pub operator_vault_connected: bool,
    pub delegation: u64,
}

/// Provides a builder pattern for setting up integration test environments.
/// Manages the ProgramTestContext and offers methods to interact with programs and control the test clock.
pub struct TestBuilder {
    context: ProgramTestContext,
}

impl Debug for TestBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestBuilder",)
    }
}

impl TestBuilder {
    /// Creates a new TestBuilder, initializing the ProgramTest environment.
    /// It adds the NCN, Vault, and Restaking programs to the test context.
    pub async fn new() -> Self {
        let run_as_bpf = std::env::vars().any(|(key, _)| key.eq("SBF_OUT_DIR"));

        let program_test = if run_as_bpf {
            let mut program_test = ProgramTest::new("ncn_program", ncn_program::id(), None);
            program_test.add_program("jito_vault_program", jito_vault_program::id(), None);
            program_test.add_program("jito_restaking_program", jito_restaking_program::id(), None);

            program_test
        } else {
            let mut program_test = ProgramTest::new(
                "ncn_program",
                ncn_program::id(),
                processor!(ncn_program::process_instruction),
            );
            program_test.add_program(
                "jito_vault_program",
                jito_vault_program::id(),
                processor!(jito_vault_program::process_instruction),
            );
            program_test.add_program(
                "jito_restaking_program",
                jito_restaking_program::id(),
                processor!(jito_restaking_program::process_instruction),
            );
            program_test
        };

        Self {
            context: program_test.start_with_context().await,
        }
    }

    /// Fetches an account from the BanksClient.
    pub async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, BanksClientError> {
        self.context.banks_client.get_account(*address).await
    }

    /// Advances the test clock by a specified number of slots.
    pub async fn warp_slot_incremental(
        &mut self,
        incremental_slots: u64,
    ) -> Result<(), BanksClientError> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        self.context
            .warp_to_slot(clock.slot.checked_add(incremental_slots).unwrap())
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    /// Advances the test clock by a specified number of epochs.
    pub async fn warp_epoch_incremental(
        &mut self,
        incremental_epochs: u64,
    ) -> Result<(), BanksClientError> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        self.context
            .warp_to_slot(
                clock
                    .slot
                    .checked_add(DEFAULT_SLOTS_PER_EPOCH * incremental_epochs)
                    .unwrap(),
            )
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    /// Retrieves the current Clock sysvar.
    pub async fn clock(&mut self) -> Clock {
        self.context.banks_client.get_sysvar().await.unwrap()
    }

    /// Retrieves the current EpochSchedule sysvar.
    pub async fn epoch_schedule(&mut self) -> EpochSchedule {
        self.context.banks_client.get_sysvar().await.unwrap()
    }

    /// Creates an NCNProgramClient instance.
    pub fn ncn_program_client(&self) -> NCNProgramClient {
        NCNProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    /// Creates a RestakingProgramClient instance.
    pub fn restaking_program_client(&self) -> RestakingProgramClient {
        RestakingProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    /// Creates a VaultProgramClient instance (alias for vault_program_client).
    pub fn vault_client(&self) -> VaultProgramClient {
        VaultProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    /// Creates a VaultProgramClient instance.
    pub fn vault_program_client(&self) -> VaultProgramClient {
        VaultProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    /// Initializes the config accounts for both the Restaking and Vault programs.
    pub async fn initialize_restaking_and_vault_programs(&mut self) -> TestResult<()> {
        let mut restaking_program_client = self.restaking_program_client();
        let mut vault_program_client = self.vault_program_client();

        vault_program_client.do_initialize_config().await?;
        restaking_program_client.do_initialize_config().await?;

        Ok(())
    }

    /// Initializes a new NCN account using the Restaking program client.
    pub async fn initialize_ncn_account(&mut self) -> TestResult<NcnRoot> {
        let mut restaking_program_client = self.restaking_program_client();

        let ncn_root = restaking_program_client
            .do_initialize_ncn(Some(self.context.payer.insecure_clone()))
            .await?;

        Ok(ncn_root)
    }

    /// Performs initial setup for an NCN, including initializing Vault and Restaking configs and the NCN account itself.
    pub async fn setup_ncn(&mut self) -> TestResult<NcnRoot> {
        let mut restaking_program_client = self.restaking_program_client();
        let mut vault_program_client = self.vault_program_client();

        vault_program_client.do_initialize_config().await?;
        restaking_program_client.do_initialize_config().await?;
        let ncn_root = restaking_program_client
            .do_initialize_ncn(Some(self.context.payer.insecure_clone()))
            .await?;

        Ok(ncn_root)
    }

    /// Creates a basic TestNcn struct with just the NCN root initialized.
    // 1a. Setup Just NCN
    pub async fn create_test_ncn(&mut self) -> TestResult<TestNcn> {
        let ncn_root = self.initialize_ncn_account().await?;

        Ok(TestNcn {
            ncn_root: ncn_root.clone(),
            operators: vec![],
            vaults: vec![],
        })
    }

    /// Adds a specified number of operators to an existing TestNcn setup.
    /// Initializes each operator and establishes the NCN-Operator link (initialization and warmup).
    // 2. Setup Operators
    pub async fn add_operators_to_test_ncn(
        &mut self,
        test_ncn: &mut TestNcn,
        operator_count: usize,
        operator_fees_bps: Option<u16>,
    ) -> TestResult<()> {
        let mut restaking_program_client = self.restaking_program_client();

        for _ in 0..operator_count {
            let operator_root = restaking_program_client
                .do_initialize_operator(operator_fees_bps)
                .await?;

            // ncn <> operator
            restaking_program_client
                .do_initialize_ncn_operator_state(
                    &test_ncn.ncn_root,
                    &operator_root.operator_pubkey,
                )
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            restaking_program_client
                .do_ncn_warmup_operator(&test_ncn.ncn_root, &operator_root.operator_pubkey)
                .await?;
            restaking_program_client
                .do_operator_warmup_ncn(&operator_root, &test_ncn.ncn_root.ncn_pubkey)
                .await?;

            test_ncn.operators.push(operator_root);
        }

        Ok(())
    }

    /// Adds a specified number of vaults to an existing TestNcn setup.
    /// Initializes each vault, establishes NCN-Vault and Operator-Vault links (initialization and warmup),
    /// and mints initial VRTs to the vault depositor.
    // 3. Setup Vaults
    pub async fn add_vaults_to_test_ncn(
        &mut self,
        test_ncn: &mut TestNcn,
        vault_count: usize,
        token_mint: Option<Keypair>,
    ) -> TestResult<()> {
        let mut vault_program_client = self.vault_program_client();
        let mut restaking_program_client = self.restaking_program_client();

        const DEPOSIT_FEE_BPS: u16 = 0;
        const WITHDRAWAL_FEE_BPS: u16 = 0;
        const REWARD_FEE_BPS: u16 = 0;
        let mint_amount: u64 = sol_to_lamports(100_000_000.0);

        let should_generate = token_mint.is_none();
        let pass_through = if token_mint.is_some() {
            token_mint.unwrap()
        } else {
            Keypair::new()
        };

        for _ in 0..vault_count {
            let pass_through = if should_generate {
                Keypair::new()
            } else {
                pass_through.insecure_clone()
            };

            let vault_root = vault_program_client
                .do_initialize_vault(
                    DEPOSIT_FEE_BPS,
                    WITHDRAWAL_FEE_BPS,
                    REWARD_FEE_BPS,
                    9,
                    &self.context.payer.pubkey(),
                    Some(pass_through),
                )
                .await?;

            // vault <> ncn
            restaking_program_client
                .do_initialize_ncn_vault_ticket(&test_ncn.ncn_root, &vault_root.vault_pubkey)
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            restaking_program_client
                .do_warmup_ncn_vault_ticket(&test_ncn.ncn_root, &vault_root.vault_pubkey)
                .await?;
            vault_program_client
                .do_initialize_vault_ncn_ticket(&vault_root, &test_ncn.ncn_root.ncn_pubkey)
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            vault_program_client
                .do_warmup_vault_ncn_ticket(&vault_root, &test_ncn.ncn_root.ncn_pubkey)
                .await?;

            for operator_root in test_ncn.operators.iter() {
                // vault <> operator
                restaking_program_client
                    .do_initialize_operator_vault_ticket(operator_root, &vault_root.vault_pubkey)
                    .await?;
                self.warp_slot_incremental(1).await.unwrap();
                restaking_program_client
                    .do_warmup_operator_vault_ticket(operator_root, &vault_root.vault_pubkey)
                    .await?;
                vault_program_client
                    .do_initialize_vault_operator_delegation(
                        &vault_root,
                        &operator_root.operator_pubkey,
                    )
                    .await?;
            }

            // This mints VRTs to make sure that the vault dose have enough funds for the
            // delegations
            let depositor_keypair = self.context.payer.insecure_clone();
            let depositor = depositor_keypair.pubkey();
            vault_program_client
                .configure_depositor(&vault_root, &depositor, mint_amount)
                .await?;
            vault_program_client
                .do_mint_to(&vault_root, &depositor_keypair, mint_amount, mint_amount)
                .await
                .unwrap();

            test_ncn.vaults.push(vault_root);
        }

        Ok(())
    }

    /// Adds delegations from all vaults to all operators within a TestNcn setup.
    // 4. Setup Delegations
    pub async fn add_delegation_in_test_ncn(
        &mut self,
        test_ncn: &TestNcn,
        delegation_amount: usize,
    ) -> TestResult<()> {
        let mut vault_program_client = self.vault_program_client();

        for vault_root in test_ncn.vaults.iter() {
            for operator_root in test_ncn.operators.iter() {
                vault_program_client
                    .do_add_delegation(
                        vault_root,
                        &operator_root.operator_pubkey,
                        delegation_amount as u64,
                    )
                    .await
                    .unwrap();
            }
        }

        Ok(())
    }

    /// Registers all vaults in the TestNcn with the NCN program's vault registry.
    /// Updates vault state, registers stMints with default weights, and registers the vault itself.
    // 5. Setup Tracked Mints
    pub async fn add_vault_registry_to_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();
        let mut restaking_client = self.restaking_program_client();
        let mut vault_client = self.vault_program_client();

        let restaking_config_address =
            Config::find_program_address(&jito_restaking_program::id()).0;
        let restaking_config = restaking_client
            .get_config(&restaking_config_address)
            .await?;

        let epoch_length = restaking_config.epoch_length();

        self.warp_slot_incremental(epoch_length * 2).await.unwrap();

        for vault in test_ncn.vaults.iter() {
            let ncn = test_ncn.ncn_root.ncn_pubkey;
            let vault = vault.vault_pubkey;

            let operators = test_ncn
                .operators
                .iter()
                .map(|operator| operator.operator_pubkey)
                .collect::<Vec<Pubkey>>();

            vault_client
                .do_full_vault_update(&vault, &operators)
                .await?;

            let st_mint = vault_client.get_vault(&vault).await?.supported_mint;

            let ncn_vault_ticket =
                NcnVaultTicket::find_program_address(&jito_restaking_program::id(), &ncn, &vault).0;

            ncn_program_client
                .do_admin_register_st_mint(ncn, st_mint, WEIGHT)
                .await?;

            ncn_program_client
                .do_register_vault(ncn, vault, ncn_vault_ticket)
                .await?;
        }

        Ok(())
    }

    /// Creates a fully initialized TestNcn setup.
    /// Initializes programs, NCN, operators, vaults, delegations, and registers vaults with the NCN.
    // Intermission: setup just NCN
    pub async fn create_initial_test_ncn(
        &mut self,
        operator_count: usize,
        vault_count: usize,
        operator_fees_bps: Option<u16>,
    ) -> TestResult<TestNcn> {
        self.initialize_restaking_and_vault_programs().await?;

        let mut test_ncn = self.create_test_ncn().await?;

        let mut ncn_program_client = self.ncn_program_client();
        ncn_program_client
            .setup_ncn_program(&test_ncn.ncn_root)
            .await?;

        self.add_operators_to_test_ncn(&mut test_ncn, operator_count, operator_fees_bps)
            .await?;
        self.add_vaults_to_test_ncn(&mut test_ncn, vault_count, None)
            .await?;
        self.add_delegation_in_test_ncn(&test_ncn, 100).await?;
        self.add_vault_registry_to_test_ncn(&test_ncn).await?;

        Ok(test_ncn)
    }

    /// Initializes the EpochState account for the current epoch for the given TestNcn.
    pub async fn add_epoch_state_for_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        // Not sure if this is needed
        self.warp_slot_incremental(1000).await?;

        let clock = self.clock().await;
        let epoch = clock.epoch;
        ncn_program_client
            .do_intialize_epoch_state(test_ncn.ncn_root.ncn_pubkey, epoch)
            .await?;

        Ok(())
    }

    /// Initializes the WeightTable for the current epoch and sets weights based on admin input (default weights).
    // 6a. Admin Set weights
    pub async fn add_admin_weights_for_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let clock = self.clock().await;
        let epoch = clock.epoch;
        ncn_program_client
            .do_full_initialize_weight_table(test_ncn.ncn_root.ncn_pubkey, epoch)
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let vault_registry = ncn_program_client.get_vault_registry(ncn).await?;

        for entry in vault_registry.st_mint_list {
            if entry.is_empty() {
                continue;
            }

            let st_mint = entry.st_mint();
            ncn_program_client
                .do_admin_set_weight(
                    test_ncn.ncn_root.ncn_pubkey,
                    epoch,
                    *st_mint,
                    entry.weight(),
                )
                .await?;
        }

        Ok(())
    }

    /// Initializes the WeightTable for the current epoch and sets weights based on the NCN's vault registry.
    // 6b. Set weights using vault registry
    pub async fn add_weights_for_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let clock = self.clock().await;
        let epoch = clock.epoch;
        ncn_program_client
            .do_full_initialize_weight_table(test_ncn.ncn_root.ncn_pubkey, epoch)
            .await?;

        ncn_program_client
            .do_set_epoch_weights(test_ncn.ncn_root.ncn_pubkey, epoch)
            .await?;

        Ok(())
    }

    /// Initializes the EpochSnapshot account for the current epoch for the given TestNcn.
    // 7. Create Epoch Snapshot
    pub async fn add_epoch_snapshot_to_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let clock = self.clock().await;
        let epoch = clock.epoch;

        ncn_program_client
            .do_initialize_epoch_snapshot(test_ncn.ncn_root.ncn_pubkey, epoch)
            .await?;

        Ok(())
    }

    /// Initializes OperatorSnapshot accounts for all operators in the TestNcn for the current epoch.
    // 8. Create all operator snapshots
    pub async fn add_operator_snapshots_to_test_ncn(
        &mut self,
        test_ncn: &TestNcn,
    ) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let clock = self.clock().await;
        let epoch = clock.epoch;

        let ncn = test_ncn.ncn_root.ncn_pubkey;

        for operator_root in test_ncn.operators.iter() {
            let operator = operator_root.operator_pubkey;

            ncn_program_client
                .do_initialize_operator_snapshot(operator, ncn, epoch)
                .await?;
        }

        Ok(())
    }

    /// Takes snapshots of VaultOperatorDelegation for all active operator-vault pairs in the TestNcn for the current epoch.
    /// Ensures vaults are updated if necessary before snapshotting.
    // 9. Take all VaultOperatorDelegation snapshots
    pub async fn add_vault_operator_delegation_snapshots_to_test_ncn(
        &mut self,
        test_ncn: &TestNcn,
    ) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();
        let mut vault_program_client = self.vault_program_client();

        let clock = self.clock().await;
        let slot = clock.slot;
        let epoch = clock.epoch;
        let ncn = test_ncn.ncn_root.ncn_pubkey;

        let operators_for_update = test_ncn
            .operators
            .iter()
            .map(|operator_root| operator_root.operator_pubkey)
            .collect::<Vec<Pubkey>>();

        for operator_root in test_ncn.operators.iter() {
            let operator = operator_root.operator_pubkey;

            let operator_snapshot = ncn_program_client
                .get_operator_snapshot(operator, ncn, epoch)
                .await?;

            // If operator snapshot is finalized it means that the operator is not active.
            if operator_snapshot.finalized() {
                continue;
            }

            for vault_root in test_ncn.vaults.iter() {
                let vault = vault_root.vault_pubkey;

                let vault_is_update_needed = vault_program_client
                    .get_vault_is_update_needed(&vault, slot)
                    .await?;

                if vault_is_update_needed {
                    vault_program_client
                        .do_full_vault_update(&vault, &operators_for_update)
                        .await?;
                }

                ncn_program_client
                    .do_snapshot_vault_operator_delegation(vault, operator, ncn, epoch)
                    .await?;
            }
        }

        Ok(())
    }

    /// Performs all necessary steps to snapshot the state of the TestNcn for the current epoch.
    /// Initializes epoch state, weight table, epoch snapshot, operator snapshots, and VOD snapshots.
    // Intermission 2 - all snapshots are taken
    pub async fn snapshot_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        self.add_epoch_state_for_test_ncn(test_ncn).await?;
        self.add_weights_for_test_ncn(test_ncn).await?;
        self.add_epoch_snapshot_to_test_ncn(test_ncn).await?;
        self.add_operator_snapshots_to_test_ncn(test_ncn).await?;
        self.add_vault_operator_delegation_snapshots_to_test_ncn(test_ncn)
            .await?;

        Ok(())
    }

    /// Initializes the BallotBox account for the current epoch for the given TestNcn.
    // 10 - Initialize Ballot Box
    pub async fn add_ballot_box_to_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let clock = self.clock().await;
        let epoch = clock.epoch;
        let ncn = test_ncn.ncn_root.ncn_pubkey;

        ncn_program_client
            .do_full_initialize_ballot_box(ncn, epoch)
            .await?;

        Ok(())
    }

    /// Casts votes (default WeatherStatus) for all active operators in the TestNcn for the current epoch.
    // 11 - Cast all votes for active operators
    pub async fn cast_votes_for_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let clock = self.clock().await;
        let epoch = clock.epoch;
        let ncn = test_ncn.ncn_root.ncn_pubkey;

        let weather_status = WeatherStatus::default() as u8;

        for operator_root in test_ncn.operators.iter() {
            let operator = operator_root.operator_pubkey;
            let operator_snapshot = ncn_program_client
                .get_operator_snapshot(operator, ncn, epoch)
                .await?;

            if operator_snapshot.is_active() {
                ncn_program_client
                    .do_cast_vote(
                        ncn,
                        operator,
                        &operator_root.operator_admin,
                        weather_status,
                        epoch,
                    )
                    .await?;
            }
        }

        Ok(())
    }

    /// Performs the voting process for the TestNcn for the current epoch.
    /// Initializes the ballot box and casts votes for all active operators.
    // Intermission 3 - come to consensus
    pub async fn vote_test_ncn(&mut self, test_ncn: &TestNcn) -> TestResult<()> {
        self.add_ballot_box_to_test_ncn(test_ncn).await?;
        self.cast_votes_for_test_ncn(test_ncn).await?;

        Ok(())
    }

    /// Closes all epoch-specific accounts (BallotBox, OperatorSnapshots, EpochSnapshot, WeightTable, EpochState)
    /// for a given epoch after the required cooldown period has passed.
    /// Asserts that the accounts are actually closed (deleted).
    pub async fn close_epoch_accounts_for_test_ncn(
        &mut self,
        test_ncn: &TestNcn,
    ) -> TestResult<()> {
        let mut ncn_program_client = self.ncn_program_client();

        let epoch_to_close = self.clock().await.epoch;
        let ncn: Pubkey = test_ncn.ncn_root.ncn_pubkey;

        let config_account = ncn_program_client.get_ncn_config(ncn).await?;

        // Wait until we can close the accounts
        {
            let epochs_after_consensus_before_close =
                config_account.epochs_after_consensus_before_close();

            self.warp_epoch_incremental(epochs_after_consensus_before_close + 1)
                .await?;
        }

        // Close Accounts in reverse order of creation

        // Ballot Box
        {
            let (ballot_box, _, _) =
                BallotBox::find_program_address(&ncn_program::id(), &ncn, epoch_to_close);

            ncn_program_client
                .do_close_epoch_account(ncn, epoch_to_close, ballot_box)
                .await?;

            let result = self.get_account(&ballot_box).await?;
            assert!(result.is_none());
        }

        // Operator Snapshots
        for operator_root in test_ncn.operators.iter() {
            let operator = operator_root.operator_pubkey;

            let (operator_snapshot, _, _) = OperatorSnapshot::find_program_address(
                &ncn_program::id(),
                &operator,
                &ncn,
                epoch_to_close,
            );

            ncn_program_client
                .do_close_epoch_account(ncn, epoch_to_close, operator_snapshot)
                .await?;

            let result = self.get_account(&operator_snapshot).await?;
            assert!(result.is_none());
        }

        // Epoch Snapshot
        {
            let (epoch_snapshot, _, _) =
                EpochSnapshot::find_program_address(&ncn_program::id(), &ncn, epoch_to_close);

            ncn_program_client
                .do_close_epoch_account(ncn, epoch_to_close, epoch_snapshot)
                .await?;

            let result = self.get_account(&epoch_snapshot).await?;
            assert!(result.is_none());
        }

        // Weight Table
        {
            let (weight_table, _, _) =
                WeightTable::find_program_address(&ncn_program::id(), &ncn, epoch_to_close);

            ncn_program_client
                .do_close_epoch_account(ncn, epoch_to_close, weight_table)
                .await?;

            let result = self.get_account(&weight_table).await?;
            assert!(result.is_none());
        }

        // Epoch State
        {
            let (epoch_state, _, _) =
                EpochState::find_program_address(&ncn_program::id(), &ncn, epoch_to_close);

            ncn_program_client
                .do_close_epoch_account(ncn, epoch_to_close, epoch_state)
                .await?;

            let result = self.get_account(&epoch_state).await?;
            assert!(result.is_none());
        }

        {
            let epoch_marker = ncn_program_client
                .get_epoch_marker(ncn, epoch_to_close)
                .await?;

            assert!(epoch_marker.slot_closed() > 0);
        }

        Ok(())
    }
}
