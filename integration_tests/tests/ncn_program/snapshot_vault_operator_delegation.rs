#[cfg(test)]
mod tests {

    use crate::fixtures::{test_builder::TestBuilder, TestResult};

    #[tokio::test]
    async fn test_snapshot_vault_operator_delegation() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut vault_client = fixture.vault_program_client();
        let mut ncn_program_client = fixture.ncn_program_client();

        let test_ncn = fixture.create_initial_test_ncn(1, 1, None).await?;
        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;

        fixture.warp_slot_incremental(1000).await?;

        let epoch = fixture.clock().await.epoch;

        ncn_program_client
            .do_full_initialize_weight_table(test_ncn.ncn_root.ncn_pubkey, epoch)
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;

        let vault_root = test_ncn.vaults[0].clone();
        let vault_address = vault_root.vault_pubkey;
        let vault = vault_client.get_vault(&vault_address).await?;

        let mint = vault.supported_mint;
        let weight = 100;

        ncn_program_client
            .do_admin_set_weight(ncn, epoch, mint, weight)
            .await?;

        ncn_program_client
            .do_initialize_epoch_snapshot(ncn, epoch)
            .await?;

        let operator = test_ncn.operators[0].operator_pubkey;

        ncn_program_client
            .do_initialize_operator_snapshot(operator, ncn, epoch)
            .await?;

        ncn_program_client
            .do_snapshot_vault_operator_delegation(vault_address, operator, ncn, epoch)
            .await?;

        Ok(())
    }
}
