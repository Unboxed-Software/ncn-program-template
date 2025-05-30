/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type WritableAccount,
} from '@solana/web3.js';
import { NCN_PROGRAM_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const INITIALIZE_OPERATOR_SNAPSHOT_DISCRIMINATOR = 9;

export function getInitializeOperatorSnapshotDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_OPERATOR_SNAPSHOT_DISCRIMINATOR);
}

export type InitializeOperatorSnapshotInstruction<
  TProgram extends string = typeof NCN_PROGRAM_PROGRAM_ADDRESS,
  TAccountEpochMarker extends string | IAccountMeta<string> = string,
  TAccountEpochState extends string | IAccountMeta<string> = string,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountRestakingConfig extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountOperator extends string | IAccountMeta<string> = string,
  TAccountNcnOperatorState extends string | IAccountMeta<string> = string,
  TAccountEpochSnapshot extends string | IAccountMeta<string> = string,
  TAccountOperatorSnapshot extends string | IAccountMeta<string> = string,
  TAccountAccountPayer extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountEpochMarker extends string
        ? ReadonlyAccount<TAccountEpochMarker>
        : TAccountEpochMarker,
      TAccountEpochState extends string
        ? WritableAccount<TAccountEpochState>
        : TAccountEpochState,
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountRestakingConfig extends string
        ? ReadonlyAccount<TAccountRestakingConfig>
        : TAccountRestakingConfig,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountOperator extends string
        ? ReadonlyAccount<TAccountOperator>
        : TAccountOperator,
      TAccountNcnOperatorState extends string
        ? ReadonlyAccount<TAccountNcnOperatorState>
        : TAccountNcnOperatorState,
      TAccountEpochSnapshot extends string
        ? WritableAccount<TAccountEpochSnapshot>
        : TAccountEpochSnapshot,
      TAccountOperatorSnapshot extends string
        ? WritableAccount<TAccountOperatorSnapshot>
        : TAccountOperatorSnapshot,
      TAccountAccountPayer extends string
        ? WritableAccount<TAccountAccountPayer>
        : TAccountAccountPayer,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeOperatorSnapshotInstructionData = {
  discriminator: number;
  epoch: bigint;
};

export type InitializeOperatorSnapshotInstructionDataArgs = {
  epoch: number | bigint;
};

export function getInitializeOperatorSnapshotInstructionDataEncoder(): Encoder<InitializeOperatorSnapshotInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['epoch', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: INITIALIZE_OPERATOR_SNAPSHOT_DISCRIMINATOR,
    })
  );
}

export function getInitializeOperatorSnapshotInstructionDataDecoder(): Decoder<InitializeOperatorSnapshotInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['epoch', getU64Decoder()],
  ]);
}

export function getInitializeOperatorSnapshotInstructionDataCodec(): Codec<
  InitializeOperatorSnapshotInstructionDataArgs,
  InitializeOperatorSnapshotInstructionData
> {
  return combineCodec(
    getInitializeOperatorSnapshotInstructionDataEncoder(),
    getInitializeOperatorSnapshotInstructionDataDecoder()
  );
}

export type InitializeOperatorSnapshotInput<
  TAccountEpochMarker extends string = string,
  TAccountEpochState extends string = string,
  TAccountConfig extends string = string,
  TAccountRestakingConfig extends string = string,
  TAccountNcn extends string = string,
  TAccountOperator extends string = string,
  TAccountNcnOperatorState extends string = string,
  TAccountEpochSnapshot extends string = string,
  TAccountOperatorSnapshot extends string = string,
  TAccountAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  epochMarker: Address<TAccountEpochMarker>;
  epochState: Address<TAccountEpochState>;
  config: Address<TAccountConfig>;
  restakingConfig: Address<TAccountRestakingConfig>;
  ncn: Address<TAccountNcn>;
  operator: Address<TAccountOperator>;
  ncnOperatorState: Address<TAccountNcnOperatorState>;
  epochSnapshot: Address<TAccountEpochSnapshot>;
  operatorSnapshot: Address<TAccountOperatorSnapshot>;
  accountPayer: Address<TAccountAccountPayer>;
  systemProgram?: Address<TAccountSystemProgram>;
  epoch: InitializeOperatorSnapshotInstructionDataArgs['epoch'];
};

export function getInitializeOperatorSnapshotInstruction<
  TAccountEpochMarker extends string,
  TAccountEpochState extends string,
  TAccountConfig extends string,
  TAccountRestakingConfig extends string,
  TAccountNcn extends string,
  TAccountOperator extends string,
  TAccountNcnOperatorState extends string,
  TAccountEpochSnapshot extends string,
  TAccountOperatorSnapshot extends string,
  TAccountAccountPayer extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof NCN_PROGRAM_PROGRAM_ADDRESS,
>(
  input: InitializeOperatorSnapshotInput<
    TAccountEpochMarker,
    TAccountEpochState,
    TAccountConfig,
    TAccountRestakingConfig,
    TAccountNcn,
    TAccountOperator,
    TAccountNcnOperatorState,
    TAccountEpochSnapshot,
    TAccountOperatorSnapshot,
    TAccountAccountPayer,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeOperatorSnapshotInstruction<
  TProgramAddress,
  TAccountEpochMarker,
  TAccountEpochState,
  TAccountConfig,
  TAccountRestakingConfig,
  TAccountNcn,
  TAccountOperator,
  TAccountNcnOperatorState,
  TAccountEpochSnapshot,
  TAccountOperatorSnapshot,
  TAccountAccountPayer,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? NCN_PROGRAM_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    epochMarker: { value: input.epochMarker ?? null, isWritable: false },
    epochState: { value: input.epochState ?? null, isWritable: true },
    config: { value: input.config ?? null, isWritable: false },
    restakingConfig: {
      value: input.restakingConfig ?? null,
      isWritable: false,
    },
    ncn: { value: input.ncn ?? null, isWritable: false },
    operator: { value: input.operator ?? null, isWritable: false },
    ncnOperatorState: {
      value: input.ncnOperatorState ?? null,
      isWritable: false,
    },
    epochSnapshot: { value: input.epochSnapshot ?? null, isWritable: true },
    operatorSnapshot: {
      value: input.operatorSnapshot ?? null,
      isWritable: true,
    },
    accountPayer: { value: input.accountPayer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.epochMarker),
      getAccountMeta(accounts.epochState),
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.restakingConfig),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.operator),
      getAccountMeta(accounts.ncnOperatorState),
      getAccountMeta(accounts.epochSnapshot),
      getAccountMeta(accounts.operatorSnapshot),
      getAccountMeta(accounts.accountPayer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeOperatorSnapshotInstructionDataEncoder().encode(
      args as InitializeOperatorSnapshotInstructionDataArgs
    ),
  } as InitializeOperatorSnapshotInstruction<
    TProgramAddress,
    TAccountEpochMarker,
    TAccountEpochState,
    TAccountConfig,
    TAccountRestakingConfig,
    TAccountNcn,
    TAccountOperator,
    TAccountNcnOperatorState,
    TAccountEpochSnapshot,
    TAccountOperatorSnapshot,
    TAccountAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeOperatorSnapshotInstruction<
  TProgram extends string = typeof NCN_PROGRAM_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    epochMarker: TAccountMetas[0];
    epochState: TAccountMetas[1];
    config: TAccountMetas[2];
    restakingConfig: TAccountMetas[3];
    ncn: TAccountMetas[4];
    operator: TAccountMetas[5];
    ncnOperatorState: TAccountMetas[6];
    epochSnapshot: TAccountMetas[7];
    operatorSnapshot: TAccountMetas[8];
    accountPayer: TAccountMetas[9];
    systemProgram: TAccountMetas[10];
  };
  data: InitializeOperatorSnapshotInstructionData;
};

export function parseInitializeOperatorSnapshotInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeOperatorSnapshotInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 11) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      epochMarker: getNextAccount(),
      epochState: getNextAccount(),
      config: getNextAccount(),
      restakingConfig: getNextAccount(),
      ncn: getNextAccount(),
      operator: getNextAccount(),
      ncnOperatorState: getNextAccount(),
      epochSnapshot: getNextAccount(),
      operatorSnapshot: getNextAccount(),
      accountPayer: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeOperatorSnapshotInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
