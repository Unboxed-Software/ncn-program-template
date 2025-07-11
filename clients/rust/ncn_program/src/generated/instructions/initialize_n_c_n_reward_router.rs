//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeNCNRewardRouter {
    pub epoch_marker: solana_program::pubkey::Pubkey,

    pub epoch_state: solana_program::pubkey::Pubkey,

    pub ncn: solana_program::pubkey::Pubkey,

    pub ncn_reward_router: solana_program::pubkey::Pubkey,

    pub ncn_reward_receiver: solana_program::pubkey::Pubkey,

    pub account_payer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeNCNRewardRouter {
    pub fn instruction(
        &self,
        args: InitializeNCNRewardRouterInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeNCNRewardRouterInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.epoch_marker,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.epoch_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ncn, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.ncn_reward_router,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.ncn_reward_receiver,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.account_payer,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeNCNRewardRouterInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::NCN_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeNCNRewardRouterInstructionData {
    discriminator: u8,
}

impl InitializeNCNRewardRouterInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 14 }
    }
}

impl Default for InitializeNCNRewardRouterInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeNCNRewardRouterInstructionArgs {
    pub epoch: u64,
}

/// Instruction builder for `InitializeNCNRewardRouter`.
///
/// ### Accounts:
///
///   0. `[]` epoch_marker
///   1. `[]` epoch_state
///   2. `[]` ncn
///   3. `[writable]` ncn_reward_router
///   4. `[writable]` ncn_reward_receiver
///   5. `[writable]` account_payer
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeNCNRewardRouterBuilder {
    epoch_marker: Option<solana_program::pubkey::Pubkey>,
    epoch_state: Option<solana_program::pubkey::Pubkey>,
    ncn: Option<solana_program::pubkey::Pubkey>,
    ncn_reward_router: Option<solana_program::pubkey::Pubkey>,
    ncn_reward_receiver: Option<solana_program::pubkey::Pubkey>,
    account_payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    epoch: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeNCNRewardRouterBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn epoch_marker(&mut self, epoch_marker: solana_program::pubkey::Pubkey) -> &mut Self {
        self.epoch_marker = Some(epoch_marker);
        self
    }
    #[inline(always)]
    pub fn epoch_state(&mut self, epoch_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.epoch_state = Some(epoch_state);
        self
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn ncn_reward_router(
        &mut self,
        ncn_reward_router: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.ncn_reward_router = Some(ncn_reward_router);
        self
    }
    #[inline(always)]
    pub fn ncn_reward_receiver(
        &mut self,
        ncn_reward_receiver: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.ncn_reward_receiver = Some(ncn_reward_receiver);
        self
    }
    #[inline(always)]
    pub fn account_payer(&mut self, account_payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.account_payer = Some(account_payer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = Some(epoch);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitializeNCNRewardRouter {
            epoch_marker: self.epoch_marker.expect("epoch_marker is not set"),
            epoch_state: self.epoch_state.expect("epoch_state is not set"),
            ncn: self.ncn.expect("ncn is not set"),
            ncn_reward_router: self
                .ncn_reward_router
                .expect("ncn_reward_router is not set"),
            ncn_reward_receiver: self
                .ncn_reward_receiver
                .expect("ncn_reward_receiver is not set"),
            account_payer: self.account_payer.expect("account_payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = InitializeNCNRewardRouterInstructionArgs {
            epoch: self.epoch.clone().expect("epoch is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_n_c_n_reward_router` CPI accounts.
pub struct InitializeNCNRewardRouterCpiAccounts<'a, 'b> {
    pub epoch_marker: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_reward_router: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_reward_receiver: &'b solana_program::account_info::AccountInfo<'a>,

    pub account_payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_n_c_n_reward_router` CPI instruction.
pub struct InitializeNCNRewardRouterCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_marker: &'b solana_program::account_info::AccountInfo<'a>,

    pub epoch_state: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_reward_router: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_reward_receiver: &'b solana_program::account_info::AccountInfo<'a>,

    pub account_payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeNCNRewardRouterInstructionArgs,
}

impl<'a, 'b> InitializeNCNRewardRouterCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeNCNRewardRouterCpiAccounts<'a, 'b>,
        args: InitializeNCNRewardRouterInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            epoch_marker: accounts.epoch_marker,
            epoch_state: accounts.epoch_state,
            ncn: accounts.ncn,
            ncn_reward_router: accounts.ncn_reward_router,
            ncn_reward_receiver: accounts.ncn_reward_receiver,
            account_payer: accounts.account_payer,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.epoch_marker.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.epoch_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ncn.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.ncn_reward_router.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.ncn_reward_receiver.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.account_payer.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeNCNRewardRouterInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::NCN_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.epoch_marker.clone());
        account_infos.push(self.epoch_state.clone());
        account_infos.push(self.ncn.clone());
        account_infos.push(self.ncn_reward_router.clone());
        account_infos.push(self.ncn_reward_receiver.clone());
        account_infos.push(self.account_payer.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitializeNCNRewardRouter` via CPI.
///
/// ### Accounts:
///
///   0. `[]` epoch_marker
///   1. `[]` epoch_state
///   2. `[]` ncn
///   3. `[writable]` ncn_reward_router
///   4. `[writable]` ncn_reward_receiver
///   5. `[writable]` account_payer
///   6. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitializeNCNRewardRouterCpiBuilder<'a, 'b> {
    instruction: Box<InitializeNCNRewardRouterCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeNCNRewardRouterCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeNCNRewardRouterCpiBuilderInstruction {
            __program: program,
            epoch_marker: None,
            epoch_state: None,
            ncn: None,
            ncn_reward_router: None,
            ncn_reward_receiver: None,
            account_payer: None,
            system_program: None,
            epoch: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn epoch_marker(
        &mut self,
        epoch_marker: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.epoch_marker = Some(epoch_marker);
        self
    }
    #[inline(always)]
    pub fn epoch_state(
        &mut self,
        epoch_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.epoch_state = Some(epoch_state);
        self
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn ncn_reward_router(
        &mut self,
        ncn_reward_router: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ncn_reward_router = Some(ncn_reward_router);
        self
    }
    #[inline(always)]
    pub fn ncn_reward_receiver(
        &mut self,
        ncn_reward_receiver: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ncn_reward_receiver = Some(ncn_reward_receiver);
        self
    }
    #[inline(always)]
    pub fn account_payer(
        &mut self,
        account_payer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.account_payer = Some(account_payer);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn epoch(&mut self, epoch: u64) -> &mut Self {
        self.instruction.epoch = Some(epoch);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = InitializeNCNRewardRouterInstructionArgs {
            epoch: self.instruction.epoch.clone().expect("epoch is not set"),
        };
        let instruction = InitializeNCNRewardRouterCpi {
            __program: self.instruction.__program,

            epoch_marker: self
                .instruction
                .epoch_marker
                .expect("epoch_marker is not set"),

            epoch_state: self
                .instruction
                .epoch_state
                .expect("epoch_state is not set"),

            ncn: self.instruction.ncn.expect("ncn is not set"),

            ncn_reward_router: self
                .instruction
                .ncn_reward_router
                .expect("ncn_reward_router is not set"),

            ncn_reward_receiver: self
                .instruction
                .ncn_reward_receiver
                .expect("ncn_reward_receiver is not set"),

            account_payer: self
                .instruction
                .account_payer
                .expect("account_payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeNCNRewardRouterCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    epoch_marker: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn_reward_router: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn_reward_receiver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    account_payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    epoch: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
