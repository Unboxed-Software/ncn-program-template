#[cfg(test)]
mod tests {
    use crate::fixtures::{test_builder::TestBuilder, TestResult};

    #[tokio::test]
    async fn cannot_create_epoch_before_starting_valid_epoch() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut ncn_program_client = fixture.ncn_program_client();

        fixture.warp_epoch_incremental(1000).await?;

        const OPERATOR_COUNT: usize = 1;
        const VAULT_COUNT: usize = 1;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let config = ncn_program_client.get_ncn_config(ncn).await?;
        let starting_valid_epoch = config.starting_valid_epoch();

        let bad_epoch = starting_valid_epoch - 1;

        let result = ncn_program_client
            .do_intialize_epoch_state(ncn, bad_epoch)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn cannot_create_after_epoch_marker() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut ncn_program_client = fixture.ncn_program_client();
        const OPERATOR_COUNT: usize = 1;
        const VAULT_COUNT: usize = 1;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, None)
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.snapshot_test_ncn(&test_ncn).await?;
        fixture.vote_test_ncn(&test_ncn).await?;
        fixture.reward_test_ncn(&test_ncn, 10_000).await?;
        fixture.close_epoch_accounts_for_test_ncn(&test_ncn).await?;

        let epoch_marker = ncn_program_client.get_epoch_marker(ncn, epoch).await?;
        assert_eq!(epoch_marker.epoch(), epoch);

        let result = ncn_program_client
            .do_intialize_epoch_state(ncn, epoch)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt1() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut ncn_program_client = fixture.ncn_program_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        {
            fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
            let epoch_state = ncn_program_client.get_epoch_state(ncn, epoch).await?;
            assert_eq!(epoch_state.epoch(), epoch);
        }

        {
            fixture.add_weights_for_test_ncn(&test_ncn).await?;
            let epoch_state = ncn_program_client.get_epoch_state(ncn, epoch).await?;
            assert!(epoch_state.set_weight_progress().is_complete());
            assert_eq!(
                epoch_state.set_weight_progress().tally(),
                VAULT_COUNT as u64
            );
            assert_eq!(
                epoch_state.set_weight_progress().total(),
                VAULT_COUNT as u64
            );
            assert_eq!(epoch_state.vault_count(), VAULT_COUNT as u64);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt2() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut ncn_program_client = fixture.ncn_program_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_weights_for_test_ncn(&test_ncn).await?;

        {
            fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
            let epoch_state = ncn_program_client.get_epoch_state(ncn, epoch).await?;
            assert_eq!(epoch_state.operator_count(), OPERATOR_COUNT as u64);
            assert!(!epoch_state.epoch_snapshot_progress().is_invalid());
        }

        {
            fixture
                .add_operator_snapshots_to_test_ncn(&test_ncn)
                .await?;
            let epoch_state = ncn_program_client.get_epoch_state(ncn, epoch).await?;

            for i in 0..OPERATOR_COUNT {
                assert_eq!(epoch_state.operator_snapshot_progress(i).tally(), 0);
                assert_eq!(
                    epoch_state.operator_snapshot_progress(i).total(),
                    VAULT_COUNT as u64
                );
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt3() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut ncn_program_client = fixture.ncn_program_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_weights_for_test_ncn(&test_ncn).await?;
        fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
        fixture
            .add_operator_snapshots_to_test_ncn(&test_ncn)
            .await?;

        {
            fixture
                .add_vault_operator_delegation_snapshots_to_test_ncn(&test_ncn)
                .await?;
            let epoch_state = ncn_program_client.get_epoch_state(ncn, epoch).await?;

            assert!(epoch_state.epoch_snapshot_progress().is_complete());
            assert_eq!(
                epoch_state.epoch_snapshot_progress().tally(),
                OPERATOR_COUNT as u64
            );
            assert_eq!(
                epoch_state.epoch_snapshot_progress().total(),
                OPERATOR_COUNT as u64
            );

            for i in 0..OPERATOR_COUNT {
                assert_eq!(
                    epoch_state.operator_snapshot_progress(i).tally(),
                    VAULT_COUNT as u64
                );
                assert_eq!(
                    epoch_state.operator_snapshot_progress(i).total(),
                    VAULT_COUNT as u64
                );
                assert!(epoch_state.operator_snapshot_progress(i).is_complete());
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt4() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut ncn_program_client = fixture.ncn_program_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_admin_weights_for_test_ncn(&test_ncn).await?;
        fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
        fixture
            .add_operator_snapshots_to_test_ncn(&test_ncn)
            .await?;
        fixture
            .add_vault_operator_delegation_snapshots_to_test_ncn(&test_ncn)
            .await?;

        {
            fixture.add_ballot_box_to_test_ncn(&test_ncn).await?;
            fixture.cast_votes_for_test_ncn(&test_ncn).await?;

            let epoch_state = ncn_program_client.get_epoch_state(ncn, epoch).await?;

            let clock = fixture.clock().await;
            let epoch_schedule = fixture.epoch_schedule().await;

            assert!(!epoch_state.was_tie_breaker_set());
            assert_eq!(epoch_state.voting_progress().tally(), OPERATOR_COUNT as u64);
            assert_eq!(
                epoch_state.get_slot_consensus_reached().unwrap(),
                clock.slot
            );
            assert_eq!(
                epoch_state
                    .get_epoch_consensus_reached(&epoch_schedule)
                    .unwrap(),
                clock.epoch
            );
            assert!(epoch_state.voting_progress().is_complete());
        }

        Ok(())
    }
}
