use anyhow::Result;
use ncn_program_core::{
    account_payer::AccountPayer, constants::MAX_OPERATORS, epoch_state::AccountStatus,
};
use solana_metrics::datapoint_info;
use solana_sdk::{clock::DEFAULT_SLOTS_PER_EPOCH, native_token::lamports_to_sol};

use crate::{
    getters::{
        get_account_payer, get_all_operators_in_ncn, get_all_tickets, get_all_vaults_in_ncn,
        get_ballot_box, get_current_epoch_and_slot, get_epoch_snapshot, get_epoch_state,
        get_is_epoch_completed, get_ncn_program_config, get_operator, get_operator_snapshot,
        get_vault, get_vault_config, get_vault_operator_delegation, get_vault_registry,
        get_weight_table,
    },
    handler::CliHandler,
};

pub const fn format_stake_weight(value: u128) -> f64 {
    value as f64
}

pub fn format_token_amount(value: u64) -> f64 {
    lamports_to_sol(value)
}

pub async fn emit_error(title: String, error: String, message: String, keeper_epoch: u64) {
    datapoint_info!(
        "ncn-program-keeper-error",
        ("command-title", title, String),
        ("error", error, String),
        ("message", message, String),
        ("keeper-epoch", keeper_epoch, i64),
    );
}

pub async fn emit_heartbeat(tick: u64, run_operations: bool, emit_metrics: bool) {
    if run_operations {
        datapoint_info!(
            "ncn-program-keeper-keeper-heartbeat-operations",
            ("tick", tick, i64),
        );
    }

    if emit_metrics {
        datapoint_info!(
            "ncn-program-keeper-keeper-heartbeat-metrics",
            ("tick", tick, i64),
        );
    }
}

#[allow(clippy::large_stack_frames)]
pub async fn emit_ncn_metrics(handler: &CliHandler, start_of_loop: bool) -> Result<()> {
    emit_ncn_metrics_epoch_slot(handler).await?;

    if start_of_loop {
        emit_ncn_metrics_tickets(handler).await?;
        emit_ncn_metrics_vault_operator_delegation(handler).await?;
        emit_ncn_metrics_operators(handler).await?;
        emit_ncn_metrics_vault_registry(handler).await?;
        emit_ncn_metrics_config(handler).await?;
        emit_ncn_metrics_account_payer(handler).await?;
        // emit_ncn_metrics_opted_in_validators(handler).await?;
    }

    Ok(())
}

pub async fn emit_ncn_metrics_epoch_slot(handler: &CliHandler) -> Result<()> {
    let ncn = handler.ncn()?;
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let epoch_percentage =
        (current_slot as f64 % DEFAULT_SLOTS_PER_EPOCH as f64) / DEFAULT_SLOTS_PER_EPOCH as f64;

    datapoint_info!(
        "ncn-program-keeper-em-epoch-slot",
        ("current-epoch", current_epoch, i64),
        ("current-slot", current_slot, i64),
        ("epoch-percentage", epoch_percentage, f64),
        ("ncn", ncn.to_string(), String),
    );

    Ok(())
}

pub async fn emit_ncn_metrics_account_payer(handler: &CliHandler) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;

    let (account_payer_address, _, _) =
        AccountPayer::find_program_address(&handler.ncn_program_id, handler.ncn()?);
    let account_payer = get_account_payer(handler).await?;

    datapoint_info!(
        "ncn-program-keeper-em-account-payer",
        ("current-epoch", current_epoch, i64),
        ("current-slot", current_slot, i64),
        ("account-payer", account_payer_address.to_string(), String),
        ("balance", account_payer.lamports, i64),
        ("balance-sol", lamports_to_sol(account_payer.lamports), f64),
    );

    Ok(())
}

pub async fn emit_ncn_metrics_tickets(handler: &CliHandler) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let vault_epoch_length = {
        let vault_config = get_vault_config(handler).await?;
        vault_config.epoch_length()
    };
    let all_tickets = get_all_tickets(handler).await?;

    for ticket in all_tickets {
        let (staked_amount, cooling_down_amount, total_security) = ticket.delegation();
        let vault_delegation_state = ticket.vault_account.delegation_state;

        datapoint_info!(
            "ncn-program-keeper-em-ticket",
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("operator", ticket.operator.to_string(), String),
            ("vault", ticket.vault.to_string(), String),
            (
                "ticket-id",
                format!(
                    "{}-{}-{}",
                    ticket.ncn.to_string(),
                    ticket.vault.to_string(),
                    ticket.operator.to_string()
                ),
                String
            ),
            ("ncn-vault", ticket.ncn_vault(), i64),
            ("vault-ncn", ticket.vault_ncn(), i64),
            ("ncn-operator", ticket.ncn_operator(), i64),
            ("operator-ncn", ticket.operator_ncn(), i64),
            ("operator-vault", ticket.operator_vault(), i64),
            ("vault-operator", ticket.vault_operator(), i64),
            // Delegation Info
            ("vod-staked-amount", format_token_amount(staked_amount), f64),
            (
                "vod-cooling-down-amount",
                format_token_amount(cooling_down_amount),
                f64
            ),
            (
                "vod-total-security",
                format_token_amount(total_security),
                f64
            ),
            // Vault Info
            (
                "vault-st-mint",
                ticket.vault_account.supported_mint.to_string(),
                String
            ),
            (
                "vault-tokens-deposited",
                format_token_amount(ticket.vault_account.tokens_deposited()),
                f64
            ),
            ("vault-vrt-supply", ticket.vault_account.vrt_supply(), i64),
            (
                "vault-vrt-cooling-down-amount",
                format_token_amount(ticket.vault_account.vrt_cooling_down_amount()),
                f64
            ),
            (
                "vault-vrt-enqueued-for-cooldown-amount",
                format_token_amount(ticket.vault_account.vrt_enqueued_for_cooldown_amount()),
                f64
            ),
            (
                "vault-vrt-ready-to-claim-amount",
                format_token_amount(ticket.vault_account.vrt_ready_to_claim_amount()),
                f64
            ),
            (
                "vault-is-update-needed",
                ticket
                    .vault_account
                    .is_update_needed(current_slot, vault_epoch_length)?,
                bool
            ),
            (
                "vault-operator-count",
                ticket.vault_account.operator_count(),
                i64
            ),
            ("vault-ncn-count", ticket.vault_account.ncn_count(), i64),
            ("vault-config-epoch-length", vault_epoch_length, i64),
            // Vault Total Delegation
            (
                "vault-total-staked-amount",
                format_token_amount(vault_delegation_state.staked_amount()),
                f64
            ),
            (
                "vod-total-cooling-down-amount",
                format_token_amount(vault_delegation_state.cooling_down_amount()),
                f64
            ),
            (
                "vod-total-total-security",
                format_token_amount(vault_delegation_state.total_security()?),
                f64
            ),
        );
    }

    Ok(())
}

pub async fn emit_ncn_metrics_vault_operator_delegation(handler: &CliHandler) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let all_operators = get_all_operators_in_ncn(handler).await?;
    let all_vaults = get_all_vaults_in_ncn(handler).await?;

    for operator in all_operators.iter() {
        for vault in all_vaults.iter() {
            let result = get_vault_operator_delegation(handler, vault, operator).await;

            if result.is_err() {
                continue;
            }
            let vault_operator_delegation = result?;

            datapoint_info!(
                "ncn-program-keeper-em-vault-operator-delegation",
                ("current-epoch", current_epoch, i64),
                ("current-slot", current_slot, i64),
                ("vault", vault.to_string(), String),
                ("operator", operator.to_string(), String),
                (
                    "delegation",
                    format_token_amount(
                        vault_operator_delegation
                            .delegation_state
                            .total_security()?
                    ),
                    f64
                ),
            );
        }
    }

    Ok(())
}

pub async fn emit_ncn_metrics_operators(handler: &CliHandler) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let all_operators = get_all_operators_in_ncn(handler).await?;
    let ballot_box_result = get_ballot_box(handler, current_epoch).await;

    for operator in all_operators {
        let operator_account = get_operator(handler, &operator).await?;

        // Emitting here so all operators get a trackable has_voted metric for alerts to avoid NoData issue
        let operator_has_voted = ballot_box_result.as_ref().map_or(false, |ballot_box| {
            ballot_box.operator_votes().iter().any(|operator_vote| {
                operator_vote.operator() == &operator && !operator_vote.is_empty()
            })
        });

        datapoint_info!(
            "ncn-program-keeper-em-operator",
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("operator", operator.to_string(), String),
            (
                "fee",
                Into::<u16>::into(operator_account.operator_fee_bps) as i64,
                i64
            ),
            ("vault-count", operator_account.vault_count(), i64),
            ("ncn-count", operator_account.ncn_count(), i64),
            ("has-voted", operator_has_voted as i64, i64)
        );
    }

    Ok(())
}

pub async fn emit_ncn_metrics_vault_registry(handler: &CliHandler) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let vault_registry = get_vault_registry(handler).await?;

    datapoint_info!(
        "ncn-program-keeper-em-vault-registry",
        ("current-epoch", current_epoch, i64),
        ("current-slot", current_slot, i64),
        ("st-mints", vault_registry.st_mint_count(), i64),
        ("vaults", vault_registry.vault_count(), i64)
    );

    for vault in vault_registry.vault_list {
        if vault.is_empty() {
            continue;
        }

        let vault_account = get_vault(handler, vault.vault()).await?;

        datapoint_info!(
            "ncn-program-keeper-em-vault-registry-vault",
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("vault", vault.vault().to_string(), String),
            ("st-mint", vault.st_mint().to_string(), String),
            ("index", vault.vault_index(), i64),
            (
                "tokens-deposited",
                format_token_amount(vault_account.tokens_deposited()),
                f64
            ),
            (
                "vrt-supply",
                format_token_amount(vault_account.vrt_supply()),
                f64
            ),
            ("operator-count", vault_account.operator_count(), i64),
            ("ncn-count", vault_account.ncn_count(), i64),
        );
    }

    for st_mint in vault_registry.st_mint_list {
        datapoint_info!(
            "ncn-program-keeper-em-vault-registry-st-mint",
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("st-mint", st_mint.st_mint().to_string(), String),
            ("weight", st_mint.weight().to_string(), String),
        );
    }

    Ok(())
}

pub async fn emit_ncn_metrics_config(handler: &CliHandler) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;

    let config = get_ncn_program_config(handler).await?;

    datapoint_info!(
        "ncn-program-keeper-em-config",
        ("current-epoch", current_epoch, i64),
        ("current-slot", current_slot, i64),
        (
            "epochs-after-consensus-before-close",
            config.epochs_after_consensus_before_close(),
            i64
        ),
        ("epochs-before-stall", config.epochs_before_stall(), i64),
        ("starting-valid-epoch", config.starting_valid_epoch(), i64),
        (
            "valid-slots-after-consensus",
            config.valid_slots_after_consensus(),
            i64
        ),
        (
            "tie-breaker-admin",
            config.tie_breaker_admin.to_string(),
            String
        ),
    );

    Ok(())
}

macro_rules! emit_epoch_datapoint {
    ($name:expr, $is_current_epoch:expr, $($fields:tt),*) => {
        // Always emit the standard metric
        datapoint_info!($name, $($fields),*);

        // If it's the current epoch, also emit with "-current" suffix
        if $is_current_epoch {
            datapoint_info!(
                concat!($name, "-current"),
                $($fields),*
            );
        }
    };
}

#[allow(clippy::large_stack_frames)]
pub async fn emit_epoch_metrics(handler: &CliHandler, epoch: u64) -> Result<()> {
    emit_epoch_metrics_state(handler, epoch).await?;
    emit_epoch_metrics_weight_table(handler, epoch).await?;
    emit_epoch_metrics_epoch_snapshot(handler, epoch).await?;
    emit_epoch_metrics_operator_snapshot(handler, epoch).await?;
    emit_epoch_metrics_ballot_box(handler, epoch).await?;

    Ok(())
}

#[allow(clippy::large_stack_frames)]
pub async fn emit_epoch_metrics_ballot_box(handler: &CliHandler, epoch: u64) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let is_current_epoch = current_epoch == epoch;

    let valid_slots_after_consensus = {
        let config = get_ncn_program_config(handler).await?;

        config.valid_slots_after_consensus()
    };

    let ballot_box_result = get_ballot_box(handler, epoch).await;
    let epoch_snapshot_result = get_epoch_snapshot(handler, epoch).await;

    if let Ok(ballot_box) = ballot_box_result {
        if let Ok(epoch_snapshot) = epoch_snapshot_result {
            let total_stake_weight = epoch_snapshot.stake_weights().stake_weight();

            for operator_vote in ballot_box.operator_votes() {
                if operator_vote.is_empty() {
                    continue;
                }

                let ballot_index = operator_vote.ballot_index();
                let ballot_tally = ballot_box.ballot_tallies()[ballot_index as usize];
                let vote = format!("{:?}", ballot_tally.ballot().status());
                ballot_tally.stake_weights().stake_weight();

                emit_epoch_datapoint!(
                    "ncn-program-keeper-ee-ballot-box-votes",
                    is_current_epoch,
                    ("current-epoch", current_epoch, i64),
                    ("current-slot", current_slot, i64),
                    ("keeper-epoch", epoch, i64),
                    ("operator", operator_vote.operator().to_string(), String),
                    ("slot-voted", operator_vote.slot_voted(), i64),
                    ("ballot-index", ballot_index, i64),
                    (
                        "operator-stake-weight",
                        format_stake_weight(operator_vote.stake_weights().stake_weight()),
                        f64
                    ),
                    (
                        "ballot-stake-weight",
                        format_stake_weight(ballot_tally.stake_weights().stake_weight()),
                        f64
                    ),
                    (
                        "total-stake-weight",
                        format_stake_weight(total_stake_weight),
                        f64
                    ),
                    ("vote", vote, String)
                );
            }

            for tally in ballot_box.ballot_tallies() {
                if !tally.is_valid() {
                    continue;
                }

                let vote = format!("{:?}", tally.ballot().status());

                emit_epoch_datapoint!(
                    "ncn-program-keeper-ee-ballot-box-tally",
                    is_current_epoch,
                    ("current-epoch", current_epoch, i64),
                    ("current-slot", current_slot, i64),
                    ("keeper-epoch", epoch, i64),
                    ("ballot-index", tally.index(), i64),
                    ("tally", tally.tally(), i64),
                    (
                        "stake-weight",
                        format_stake_weight(tally.stake_weights().stake_weight()),
                        f64
                    ),
                    (
                        "total-stake-weight",
                        format_stake_weight(total_stake_weight),
                        f64
                    ),
                    ("vote", vote, String)
                );
            }

            let (winning_ballot_string, winning_stake_weight, winning_tally) = {
                if ballot_box.has_winning_ballot() {
                    let ballot_tally = ballot_box.get_winning_ballot_tally().unwrap();
                    (
                        format!("{:?}", ballot_tally.ballot().status()),
                        ballot_tally.stake_weights().stake_weight(),
                        ballot_tally.tally(),
                    )
                } else {
                    ("None".to_string(), 0, 0)
                }
            };

            emit_epoch_datapoint!(
                "ncn-program-keeper-ee-ballot-box",
                is_current_epoch,
                ("current-epoch", current_epoch, i64),
                ("current-slot", current_slot, i64),
                ("keeper-epoch", epoch, i64),
                ("unique-ballots", ballot_box.unique_ballots(), i64),
                ("operators-voted", ballot_box.operators_voted(), i64),
                ("has-winning-ballot", ballot_box.has_winning_ballot(), bool),
                ("winning-ballot", winning_ballot_string, String),
                (
                    "winning-stake-weight",
                    format_stake_weight(winning_stake_weight),
                    f64
                ),
                ("winning-tally", winning_tally, i64),
                (
                    "total-stake-weight",
                    format_stake_weight(total_stake_weight),
                    f64
                ),
                (
                    "is-voting-valid",
                    ballot_box.is_voting_valid(current_slot, valid_slots_after_consensus)?,
                    bool
                )
            );
        }
    }

    Ok(())
}

pub async fn emit_epoch_metrics_operator_snapshot(handler: &CliHandler, epoch: u64) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let is_current_epoch = current_epoch == epoch;

    let all_operators = get_all_operators_in_ncn(handler).await?;

    for operator in all_operators.iter() {
        let result = get_operator_snapshot(handler, operator, epoch).await;

        if let Ok(operator_snapshot) = result {
            emit_epoch_datapoint!(
                "ncn-program-keeper-ee-operator-snapshot",
                is_current_epoch,
                ("current-epoch", current_epoch, i64),
                ("current-slot", current_slot, i64),
                ("keeper-epoch", epoch, i64),
                ("operator", operator.to_string(), String),
                ("is-finalized", operator_snapshot.finalized(), bool),
                ("is-active", operator_snapshot.is_active(), bool),
                (
                    "ncn-operator-index",
                    operator_snapshot.ncn_operator_index(),
                    i64
                ),
                (
                    "operator-fee-bps",
                    operator_snapshot.operator_fee_bps(),
                    i64
                ),
                (
                    "valid-operator-vault-delegations",
                    operator_snapshot.valid_operator_vault_delegations(),
                    i64
                ),
                (
                    "vault-operator-delegation-count",
                    operator_snapshot.vault_operator_delegation_count(),
                    i64
                ),
                (
                    "vault-operator-delegations-registered",
                    operator_snapshot.vault_operator_delegations_registered(),
                    i64
                ),
                (
                    "stake-weight",
                    format_stake_weight(operator_snapshot.stake_weights().stake_weight()),
                    f64
                ),
                ("slot-finalized", operator_snapshot.slot_finalized(), i64)
            );
        }
    }

    Ok(())
}

pub async fn emit_epoch_metrics_epoch_snapshot(handler: &CliHandler, epoch: u64) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let is_current_epoch = current_epoch == epoch;

    let result = get_epoch_snapshot(handler, epoch).await;

    if let Ok(epoch_snapshot) = result {
        emit_epoch_datapoint!(
            "ncn-program-keeper-ee-epoch-snapshot",
            is_current_epoch,
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("keeper-epoch", epoch, i64),
            (
                "total-stake-weight",
                format_stake_weight(epoch_snapshot.stake_weights().stake_weight()),
                f64
            ),
            (
                "valid-operator-vault-delegations",
                epoch_snapshot.valid_operator_vault_delegations(),
                i64
            ),
            (
                "operators-registered",
                epoch_snapshot.operators_registered(),
                i64
            ),
            ("operator-count", epoch_snapshot.operator_count(), i64),
            ("vault-count", epoch_snapshot.vault_count(), i64)
        );
    }

    Ok(())
}

pub async fn emit_epoch_metrics_weight_table(handler: &CliHandler, epoch: u64) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let is_current_epoch = current_epoch == epoch;

    let result = get_weight_table(handler, epoch).await;

    if let Ok(weight_table) = result {
        for entry in weight_table.table() {
            if entry.is_empty() {
                continue;
            }

            emit_epoch_datapoint!(
                "ncn-program-keeper-ee-weight-table-entry",
                is_current_epoch,
                ("current-epoch", current_epoch, i64),
                ("current-slot", current_slot, i64),
                ("keeper-epoch", epoch, i64),
                ("st-mint", entry.st_mint().to_string(), String),
                ("weight", format_stake_weight(entry.weight()), f64)
            );
        }

        emit_epoch_datapoint!(
            "ncn-program-keeper-ee-weight-table",
            is_current_epoch,
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("keeper-epoch", epoch, i64),
            ("weight-count", weight_table.mint_count(), i64),
            ("vault-count", weight_table.vault_count(), i64),
            ("weight-count", weight_table.weight_count(), i64)
        );
    }

    Ok(())
}

#[allow(clippy::large_stack_frames)]
pub async fn emit_epoch_metrics_state(handler: &CliHandler, epoch: u64) -> Result<()> {
    let (current_epoch, current_slot) = get_current_epoch_and_slot(handler).await?;
    let is_current_epoch = current_epoch == epoch;

    let is_epoch_completed = get_is_epoch_completed(handler, epoch).await?;

    if is_epoch_completed {
        emit_epoch_datapoint!(
            "ncn-program-keeper-ee-state",
            is_current_epoch,
            ("current-epoch", current_epoch, i64),
            ("current-slot", current_slot, i64),
            ("keeper-epoch", epoch, i64),
            ("current-state-string", "Complete", String),
            ("current-state", -1, i64),
            ("is-complete", true, bool)
        );

        return Ok(());
    }

    let state = get_epoch_state(handler, epoch).await?;
    let current_state = {
        let (valid_slots_after_consensus, epochs_after_consensus_before_close) = {
            let config = get_ncn_program_config(handler).await?;
            (
                config.valid_slots_after_consensus(),
                config.epochs_after_consensus_before_close(),
            )
        };
        let epoch_schedule = handler.rpc_client().get_epoch_schedule().await?;

        if state.set_weight_progress().tally() > 0 {
            let weight_table = get_weight_table(handler, epoch).await?;
            state.current_state_patched(
                &epoch_schedule,
                valid_slots_after_consensus,
                epochs_after_consensus_before_close,
                weight_table.st_mint_count() as u64,
                current_slot,
            )
        } else {
            state.current_state(
                &epoch_schedule,
                valid_slots_after_consensus,
                epochs_after_consensus_before_close,
                current_slot,
            )
        }
    }?;

    let mut operator_snapshot_dne = 0;
    let mut operator_snapshot_open = 0;
    let mut operator_snapshot_closed = 0;
    for i in 0..MAX_OPERATORS {
        let operator_snapshot_status = state.account_status().operator_snapshot(i)?;

        match operator_snapshot_status {
            AccountStatus::DNE => operator_snapshot_dne += 1,
            AccountStatus::Closed => operator_snapshot_closed += 1,
            _ => operator_snapshot_open += 1,
        }
    }

    emit_epoch_datapoint!(
        "ncn-program-keeper-ee-state",
        is_current_epoch,
        ("current-epoch", current_epoch, i64),
        ("current-slot", current_slot, i64),
        ("keeper-epoch", epoch, i64),
        ("is-complete", false, bool),
        (
            "current-state-string",
            format!("{:?}", current_state),
            String
        ),
        ("current-state", current_state as u8, i64),
        ("operator-count", state.operator_count(), i64),
        ("vault-count", state.vault_count(), i64),
        (
            "slot-consensus-reached",
            state.slot_consensus_reached(),
            i64
        ),
        (
            "set-weight-progress-tally",
            state.set_weight_progress().tally(),
            i64
        ),
        (
            "set-weight-progress-total",
            state.set_weight_progress().total(),
            i64
        ),
        (
            "epoch-snapshot-progress-tally",
            state.epoch_snapshot_progress().tally(),
            i64
        ),
        (
            "epoch-snapshot-progress-total",
            state.epoch_snapshot_progress().total(),
            i64
        ),
        (
            "voting-progress-tally",
            state.voting_progress().tally(),
            i64
        ),
        (
            "voting-progress-total",
            state.voting_progress().total(),
            i64
        ),
        // Account status
        (
            "epoch-state-account-status",
            state.account_status().epoch_state()?,
            i64
        ),
        (
            "weight-table-account-status",
            state.account_status().weight_table()?,
            i64
        ),
        (
            "epoch-snapshot-account-status",
            state.account_status().epoch_snapshot()?,
            i64
        ),
        (
            "ballot-box-account-status",
            state.account_status().ballot_box()?,
            i64
        ),
        ("operator-snapshot-account-dne", operator_snapshot_dne, i64),
        (
            "operator-snapshot-account-open",
            operator_snapshot_open,
            i64
        ),
        (
            "operator-snapshot-account-closed",
            operator_snapshot_closed,
            i64
        )
    );

    Ok(())
}
