// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/submittable";

import type {
  ApiTypes,
  AugmentedSubmittable,
  SubmittableExtrinsic,
  SubmittableExtrinsicFunction,
} from "@polkadot/api-base/types";
import type {
  Bytes,
  Compact,
  Option,
  Vec,
  bool,
  u128,
  u16,
  u32,
  u64,
} from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type {
  AccountId32,
  Call,
  H256,
  MultiAddress,
  Perbill,
} from "@polkadot/types/interfaces/runtime";
import type {
  CumulusPrimitivesParachainInherentParachainInherentData,
  DanceboxRuntimeOriginCaller,
  DanceboxRuntimeProxyType,
  DanceboxRuntimeSessionKeys,
  SpWeightsWeightV2Weight,
  TpAuthorNotingInherentOwnParachainInherentData,
  TpContainerChainGenesisDataContainerChainGenesisData,
} from "@polkadot/types/lookup";

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> =
  SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> =
  SubmittableExtrinsicFunction<ApiType>;

declare module "@polkadot/api-base/types/submittable" {
  interface AugmentedSubmittables<ApiType extends ApiTypes> {
    authorInherent: {
      /**
       * This inherent is a workaround to run code after the "real" inherents
       * have executed, but before transactions are executed.
       */
      kickOffAuthorshipValidation: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    authorityAssignment: {
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    authorNoting: {
      setAuthor: AugmentedSubmittable<
        (
          paraId: u32 | AnyNumber | Uint8Array,
          blockNumber: u32 | AnyNumber | Uint8Array,
          author: AccountId32 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32, u32, AccountId32]
      >;
      setLatestAuthorData: AugmentedSubmittable<
        (
          data:
            | TpAuthorNotingInherentOwnParachainInherentData
            | { relayStorageProof?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [TpAuthorNotingInherentOwnParachainInherentData]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    balances: {
      /**
       * Set the regular balance of a given account.
       *
       * The dispatch origin for this call is `root`.
       */
      forceSetBalance: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          newFree: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>]
      >;
      /**
       * Exactly as `transfer_allow_death`, except the origin must be root and
       * the source account may be specified.
       */
      forceTransfer: AugmentedSubmittable<
        (
          source:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, MultiAddress, Compact<u128>]
      >;
      /**
       * Unreserve some balance from a user by force.
       *
       * Can only be called by ROOT.
       */
      forceUnreserve: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          amount: u128 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, u128]
      >;
      /**
       * Set the regular balance of a given account; it also takes a reserved
       * balance but this must be the same as the account's current reserved balance.
       *
       * The dispatch origin for this call is `root`.
       *
       * WARNING: This call is DEPRECATED! Use `force_set_balance` instead.
       */
      setBalanceDeprecated: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          newFree: Compact<u128> | AnyNumber | Uint8Array,
          oldReserved: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>, Compact<u128>]
      >;
      /**
       * Alias for `transfer_allow_death`, provided only for name-wise compatibility.
       *
       * WARNING: DEPRECATED! Will be released in approximately 3 months.
       */
      transfer: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>]
      >;
      /**
       * Transfer the entire transferable balance from the caller account.
       *
       * NOTE: This function only attempts to transfer _transferable_ balances.
       * This means that any locked, reserved, or existential deposits (when
       * `keep_alive` is `true`), will not be transferred by this function. To
       * ensure that this function results in a killed account, you might need
       * to prepare the account by removing any reference counters, storage
       * deposits, etc...
       *
       * The dispatch origin of this call must be Signed.
       *
       * - `dest`: The recipient of the transfer.
       * - `keep_alive`: A boolean to determine if the `transfer_all` operation
       *   should send all of the funds the account has, causing the sender
       *   account to be killed (false), or transfer everything except at least
       *   the existential deposit, which will guarantee to keep the sender
       *   account alive (true).
       */
      transferAll: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          keepAlive: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, bool]
      >;
      /**
       * Transfer some liquid free balance to another account.
       *
       * `transfer_allow_death` will set the `FreeBalance` of the sender and
       * receiver. If the sender's account is below the existential deposit as a
       * result of the transfer, the account will be reaped.
       *
       * The dispatch origin for this call must be `Signed` by the transactor.
       */
      transferAllowDeath: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>]
      >;
      /**
       * Same as the [`transfer_allow_death`][`transfer_allow_death`] call, but
       * with a check that the transfer will not kill the origin account.
       *
       * 99% of the time you want
       * [`transfer_allow_death`][`transfer_allow_death`] instead.
       *
       * [`transfer_allow_death`]: struct.Pallet.html#method.transfer
       */
      transferKeepAlive: AugmentedSubmittable<
        (
          dest:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          value: Compact<u128> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Compact<u128>]
      >;
      /**
       * Upgrade a specified account.
       *
       * - `origin`: Must be `Signed`.
       * - `who`: The account to be upgraded.
       *
       * This will waive the transaction fee if at least all but 10% of the
       * accounts needed to be upgraded. (We let some not have to be upgraded
       * just in order to allow for the possibililty of churn).
       */
      upgradeAccounts: AugmentedSubmittable<
        (
          who: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<AccountId32>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    collatorAssignment: {
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    collatorSelection: {
      /**
       * Deregister `origin` as a collator candidate. Note that the collator can
       * only leave on session change. The `CandidacyBond` will be unreserved
       * immediately.
       *
       * This call will fail if the total number of candidates would drop below
       * `MinCandidates`.
       *
       * This call is not available to `Invulnerable` collators.
       */
      leaveIntent: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /**
       * Register this account as a collator candidate. The account must (a)
       * already have registered session keys and (b) be able to reserve the
       * `CandidacyBond`.
       *
       * This call is not available to `Invulnerable` collators.
       */
      registerAsCandidate: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /** Set the candidacy bond amount. */
      setCandidacyBond: AugmentedSubmittable<
        (bond: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [u128]
      >;
      /**
       * Set the ideal number of collators (not including the invulnerables). If
       * lowering this number, then the number of running collators could be
       * higher than this figure. Aside from that edge case, there should be no
       * other way to have more collators than the desired number.
       */
      setDesiredCandidates: AugmentedSubmittable<
        (max: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      /** Set the list of invulnerable (fixed) collators. */
      setInvulnerables: AugmentedSubmittable<
        (
          updated: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<AccountId32>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    configuration: {
      /**
       * Setting this to true will disable consistency checks for the
       * configuration setters. Use with caution.
       */
      setBypassConsistencyCheck: AugmentedSubmittable<
        (updated: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [bool]
      >;
      setCollatorsPerContainer: AugmentedSubmittable<
        (
          updated: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      setMaxCollators: AugmentedSubmittable<
        (
          updated: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      setMaxOrchestratorCollators: AugmentedSubmittable<
        (
          updated: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      setMinOrchestratorCollators: AugmentedSubmittable<
        (
          updated: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    maintenanceMode: {
      /**
       * Place the chain in maintenance mode
       *
       * Weight cost is:
       *
       * - One DB read to ensure we're not already in maintenance mode
       * - Three DB writes - 1 for the mode, 1 for suspending xcm execution, 1 for the event
       */
      enterMaintenanceMode: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /**
       * Return the chain to normal operating mode
       *
       * Weight cost is:
       *
       * - One DB read to ensure we're in maintenance mode
       * - Three DB writes - 1 for the mode, 1 for resuming xcm execution, 1 for the event
       */
      resumeNormalOperation: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    parachainInfo: {
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    parachainSystem: {
      /**
       * Authorize an upgrade to a given `code_hash` for the runtime. The
       * runtime can be supplied later.
       *
       * The `check_version` parameter sets a boolean flag for whether or not
       * the runtime's spec version and name should be verified on upgrade.
       * Since the authorization only has a hash, it cannot actually perform the
       * verification.
       *
       * This call requires Root origin.
       */
      authorizeUpgrade: AugmentedSubmittable<
        (
          codeHash: H256 | string | Uint8Array,
          checkVersion: bool | boolean | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [H256, bool]
      >;
      /**
       * Provide the preimage (runtime binary) `code` for an upgrade that has
       * been authorized.
       *
       * If the authorization required a version check, this call will ensure
       * the spec name remains unchanged and that the spec version has increased.
       *
       * Note that this function will not apply the new `code`, but only attempt
       * to schedule the upgrade with the Relay Chain.
       *
       * All origins are allowed.
       */
      enactAuthorizedUpgrade: AugmentedSubmittable<
        (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /**
       * Set the current validation data.
       *
       * This should be invoked exactly once per block. It will panic at the
       * finalization phase if the call was not invoked.
       *
       * The dispatch origin for this call must be `Inherent`
       *
       * As a side effect, this function upgrades the current validation
       * function if the appropriate time has come.
       */
      setValidationData: AugmentedSubmittable<
        (
          data:
            | CumulusPrimitivesParachainInherentParachainInherentData
            | {
                validationData?: any;
                relayChainState?: any;
                downwardMessages?: any;
                horizontalMessages?: any;
              }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [CumulusPrimitivesParachainInherentParachainInherentData]
      >;
      sudoSendUpwardMessage: AugmentedSubmittable<
        (message: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    proxy: {
      /**
       * Register a proxy account for the sender that is able to make calls on its behalf.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * Parameters:
       *
       * - `proxy`: The account that the `caller` would like to make a proxy.
       * - `proxy_type`: The permissions allowed for this proxy account.
       * - `delay`: The announcement period required of the initial proxy. Will
       *   generally be zero.
       */
      addProxy: AugmentedSubmittable<
        (
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          proxyType:
            | DanceboxRuntimeProxyType
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | number
            | Uint8Array,
          delay: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, DanceboxRuntimeProxyType, u32]
      >;
      /**
       * Publish the hash of a proxy-call that will be made in the future.
       *
       * This must be called some number of blocks before the corresponding
       * `proxy` is attempted if the delay associated with the proxy
       * relationship is greater than zero.
       *
       * No more than `MaxPending` announcements may be made at any one time.
       *
       * This will take a deposit of `AnnouncementDepositFactor` as well as
       * `AnnouncementDepositBase` if there are no other pending announcements.
       *
       * The dispatch origin for this call must be _Signed_ and a proxy of `real`.
       *
       * Parameters:
       *
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `call_hash`: The hash of the call to be made by the `real` account.
       */
      announce: AugmentedSubmittable<
        (
          real:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          callHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, H256]
      >;
      /**
       * Spawn a fresh new account that is guaranteed to be otherwise
       * inaccessible, and initialize it with a proxy of `proxy_type` for
       * `origin` sender.
       *
       * Requires a `Signed` origin.
       *
       * - `proxy_type`: The type of the proxy that the sender will be registered
       *   as over the new account. This will almost always be the most
       *   permissive `ProxyType` possible to allow for maximum flexibility.
       * - `index`: A disambiguation index, in case this is called multiple times
       *   in the same transaction (e.g. with `utility::batch`). Unless you're
       *   using `batch` you probably just want to use `0`.
       * - `delay`: The announcement period required of the initial proxy. Will
       *   generally be zero.
       *
       * Fails with `Duplicate` if this has already been called in this
       * transaction, from the same sender, with the same parameters.
       *
       * Fails if there are insufficient funds to pay for deposit.
       */
      createPure: AugmentedSubmittable<
        (
          proxyType:
            | DanceboxRuntimeProxyType
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | number
            | Uint8Array,
          delay: u32 | AnyNumber | Uint8Array,
          index: u16 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [DanceboxRuntimeProxyType, u32, u16]
      >;
      /**
       * Removes a previously spawned pure proxy.
       *
       * WARNING: **All access to this account will be lost.** Any funds held in
       * it will be inaccessible.
       *
       * Requires a `Signed` origin, and the sender account must have been
       * created by a call to `pure` with corresponding parameters.
       *
       * - `spawner`: The account that originally called `pure` to create this account.
       * - `index`: The disambiguation index originally passed to `pure`. Probably `0`.
       * - `proxy_type`: The proxy type originally passed to `pure`.
       * - `height`: The height of the chain when the call to `pure` was processed.
       * - `ext_index`: The extrinsic index in which the call to `pure` was processed.
       *
       * Fails with `NoPermission` in case the caller is not a previously
       * created pure account whose `pure` call has corresponding parameters.
       */
      killPure: AugmentedSubmittable<
        (
          spawner:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          proxyType:
            | DanceboxRuntimeProxyType
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | number
            | Uint8Array,
          index: u16 | AnyNumber | Uint8Array,
          height: Compact<u32> | AnyNumber | Uint8Array,
          extIndex: Compact<u32> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [
          MultiAddress,
          DanceboxRuntimeProxyType,
          u16,
          Compact<u32>,
          Compact<u32>
        ]
      >;
      /**
       * Dispatch the given `call` from an account that the sender is authorised
       * for through `add_proxy`.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * Parameters:
       *
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `force_proxy_type`: Specify the exact proxy type to be used and checked
       *   for this call.
       * - `call`: The call to be made by the `real` account.
       */
      proxy: AugmentedSubmittable<
        (
          real:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          forceProxyType:
            | Option<DanceboxRuntimeProxyType>
            | null
            | Uint8Array
            | DanceboxRuntimeProxyType
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | number,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Option<DanceboxRuntimeProxyType>, Call]
      >;
      /**
       * Dispatch the given `call` from an account that the sender is authorized
       * for through `add_proxy`.
       *
       * Removes any corresponding announcement(s).
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * Parameters:
       *
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `force_proxy_type`: Specify the exact proxy type to be used and checked
       *   for this call.
       * - `call`: The call to be made by the `real` account.
       */
      proxyAnnounced: AugmentedSubmittable<
        (
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          real:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          forceProxyType:
            | Option<DanceboxRuntimeProxyType>
            | null
            | Uint8Array
            | DanceboxRuntimeProxyType
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | number,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, MultiAddress, Option<DanceboxRuntimeProxyType>, Call]
      >;
      /**
       * Remove the given announcement of a delegate.
       *
       * May be called by a target (proxied) account to remove a call that one
       * of their delegates (`delegate`) has announced they want to execute. The
       * deposit is returned.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * Parameters:
       *
       * - `delegate`: The account that previously announced the call.
       * - `call_hash`: The hash of the call to be made.
       */
      rejectAnnouncement: AugmentedSubmittable<
        (
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          callHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, H256]
      >;
      /**
       * Remove a given announcement.
       *
       * May be called by a proxy account to remove a call they previously
       * announced and return the deposit.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * Parameters:
       *
       * - `real`: The account that the proxy will make a call on behalf of.
       * - `call_hash`: The hash of the call to be made by the `real` account.
       */
      removeAnnouncement: AugmentedSubmittable<
        (
          real:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          callHash: H256 | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, H256]
      >;
      /**
       * Unregister all proxy accounts for the sender.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * WARNING: This may be called on accounts created by `pure`, however if
       * done, then the unreserved fees will be inaccessible. **All access to
       * this account will be lost.**
       */
      removeProxies: AugmentedSubmittable<
        () => SubmittableExtrinsic<ApiType>,
        []
      >;
      /**
       * Unregister a proxy account for the sender.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * Parameters:
       *
       * - `proxy`: The account that the `caller` would like to remove as a proxy.
       * - `proxy_type`: The permissions currently enabled for the removed proxy account.
       */
      removeProxy: AugmentedSubmittable<
        (
          delegate:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          proxyType:
            | DanceboxRuntimeProxyType
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | number
            | Uint8Array,
          delay: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, DanceboxRuntimeProxyType, u32]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    registrar: {
      /**
       * Deregister container-chain.
       *
       * If a container-chain is registered but not marked as
       * valid_for_collating, this will remove it from `PendingVerification` as well.
       */
      deregister: AugmentedSubmittable<
        (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      /** Mark container-chain valid for collating */
      markValidForCollating: AugmentedSubmittable<
        (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [u32]
      >;
      /** Register container-chain */
      register: AugmentedSubmittable<
        (
          paraId: u32 | AnyNumber | Uint8Array,
          genesisData:
            | TpContainerChainGenesisDataContainerChainGenesisData
            | {
                storage?: any;
                name?: any;
                id?: any;
                forkId?: any;
                extensions?: any;
                properties?: any;
              }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u32, TpContainerChainGenesisDataContainerChainGenesisData]
      >;
      /** Set boot_nodes for this para id */
      setBootNodes: AugmentedSubmittable<
        (
          paraId: u32 | AnyNumber | Uint8Array,
          bootNodes: Vec<Bytes> | (Bytes | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [u32, Vec<Bytes>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rootTesting: {
      /** A dispatch that will fill the block weight up to the given ratio. */
      fillBlock: AugmentedSubmittable<
        (
          ratio: Perbill | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Perbill]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    session: {
      /**
       * Removes any session key(s) of the function caller.
       *
       * This doesn't take effect until the next session.
       *
       * The dispatch origin of this function must be Signed and the account
       * must be either be convertible to a validator ID using the chain's
       * typical addressing system (this usually means being a controller
       * account) or directly convertible into a validator ID (which usually
       * means being a stash account).
       *
       * ## Complexity
       *
       * - `O(1)` in number of key types. Actual cost depends on the number of
       *   length of `T::Keys::key_ids()` which is fixed.
       */
      purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Sets the session key(s) of the function caller to `keys`. Allows an
       * account to set its session key prior to becoming a validator. This
       * doesn't take effect until the next session.
       *
       * The dispatch origin of this function must be signed.
       *
       * ## Complexity
       *
       * - `O(1)`. Actual cost depends on the number of length of
       *   `T::Keys::key_ids()` which is fixed.
       */
      setKeys: AugmentedSubmittable<
        (
          keys:
            | DanceboxRuntimeSessionKeys
            | { nimbus?: any }
            | string
            | Uint8Array,
          proof: Bytes | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [DanceboxRuntimeSessionKeys, Bytes]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * Authenticates the current sudo key and sets the given AccountId (`new`)
       * as the new sudo key.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * ## Complexity
       *
       * - O(1).
       */
      setKey: AugmentedSubmittable<
        (
          updated:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress]
      >;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * ## Complexity
       *
       * - O(1).
       */
      sudo: AugmentedSubmittable<
        (
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Call]
      >;
      /**
       * Authenticates the sudo key and dispatches a function call with `Signed`
       * origin from a given account.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * ## Complexity
       *
       * - O(1).
       */
      sudoAs: AugmentedSubmittable<
        (
          who:
            | MultiAddress
            | { Id: any }
            | { Index: any }
            | { Raw: any }
            | { Address32: any }
            | { Address20: any }
            | string
            | Uint8Array,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [MultiAddress, Call]
      >;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root`
       * origin. This function does not check the weight of the call, and
       * instead allows the Sudo user to specify the weight of the call.
       *
       * The dispatch origin for this call must be _Signed_.
       *
       * ## Complexity
       *
       * - O(1).
       */
      sudoUncheckedWeight: AugmentedSubmittable<
        (
          call: Call | IMethod | string | Uint8Array,
          weight:
            | SpWeightsWeightV2Weight
            | { refTime?: any; proofSize?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Call, SpWeightsWeightV2Weight]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    system: {
      /**
       * Kill all storage items with a key that starts with the given prefix.
       *
       * **NOTE:** We rely on the Root origin to provide us the number of
       * subkeys under the prefix we are removing to accurately calculate the
       * weight of this function.
       */
      killPrefix: AugmentedSubmittable<
        (
          prefix: Bytes | string | Uint8Array,
          subkeys: u32 | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Bytes, u32]
      >;
      /** Kill some items from storage. */
      killStorage: AugmentedSubmittable<
        (
          keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Bytes>]
      >;
      /**
       * Make some on-chain remark.
       *
       * - `O(1)`
       */
      remark: AugmentedSubmittable<
        (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Make some on-chain remark and emit event. */
      remarkWithEvent: AugmentedSubmittable<
        (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Set the new runtime code. */
      setCode: AugmentedSubmittable<
        (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Set the new runtime code without doing any checks of the given `code`. */
      setCodeWithoutChecks: AugmentedSubmittable<
        (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [Bytes]
      >;
      /** Set the number of pages in the WebAssembly environment's heap. */
      setHeapPages: AugmentedSubmittable<
        (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
        [u64]
      >;
      /** Set some items of storage. */
      setStorage: AugmentedSubmittable<
        (
          items:
            | Vec<ITuple<[Bytes, Bytes]>>
            | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<ITuple<[Bytes, Bytes]>>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    timestamp: {
      /**
       * Set the current time.
       *
       * This call should be invoked exactly once per block. It will panic at
       * the finalization phase, if this call hasn't been invoked by that time.
       *
       * The timestamp should be greater than the previous one by the amount
       * specified by `MinimumPeriod`.
       *
       * The dispatch origin for this call must be `Inherent`.
       *
       * ## Complexity
       *
       * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
       * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of
       *   `DidUpdate::take` in `on_finalize`)
       * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
       */
      set: AugmentedSubmittable<
        (
          now: Compact<u64> | AnyNumber | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Compact<u64>]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    utility: {
      /**
       * Send a call through an indexed pseudonym of the sender.
       *
       * Filter from origin are passed along. The call will be dispatched with
       * an origin which use the same filter as the origin of this call.
       *
       * NOTE: If you need to ensure that any account-based filtering is not
       * honored (i.e. because you expect `proxy` to have been used prior in the
       * call stack and you do not want the call restrictions to apply to any
       * sub-accounts), then use `as_multi_threshold_1` in the Multisig pallet instead.
       *
       * NOTE: Prior to version *12, this was called `as_limited_sub`.
       *
       * The dispatch origin for this call must be _Signed_.
       */
      asDerivative: AugmentedSubmittable<
        (
          index: u16 | AnyNumber | Uint8Array,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [u16, Call]
      >;
      /**
       * Send a batch of dispatch calls.
       *
       * May be called from any origin except `None`.
       *
       * - `calls`: The calls to be dispatched from the same origin. The number of
       *   call must not exceed the constant: `batched_calls_limit` (available
       *   in constant metadata).
       *
       * If origin is root then the calls are dispatched without checking origin
       * filter. (This includes bypassing `frame_system::Config::BaseCallFilter`).
       *
       * ## Complexity
       *
       * - O(C) where C is the number of calls to be batched.
       *
       * This will return `Ok` in all circumstances. To determine the success of
       * the batch, an event is deposited. If a call failed and the batch was
       * interrupted, then the `BatchInterrupted` event is deposited, along with
       * the number of successful calls made and the error of the failed call.
       * If all were successful, then the `BatchCompleted` event is deposited.
       */
      batch: AugmentedSubmittable<
        (
          calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Call>]
      >;
      /**
       * Send a batch of dispatch calls and atomically execute them. The whole
       * transaction will rollback and fail if any of the calls failed.
       *
       * May be called from any origin except `None`.
       *
       * - `calls`: The calls to be dispatched from the same origin. The number of
       *   call must not exceed the constant: `batched_calls_limit` (available
       *   in constant metadata).
       *
       * If origin is root then the calls are dispatched without checking origin
       * filter. (This includes bypassing `frame_system::Config::BaseCallFilter`).
       *
       * ## Complexity
       *
       * - O(C) where C is the number of calls to be batched.
       */
      batchAll: AugmentedSubmittable<
        (
          calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Call>]
      >;
      /**
       * Dispatches a function call with a provided origin.
       *
       * The dispatch origin for this call must be _Root_.
       *
       * ## Complexity
       *
       * - O(1).
       */
      dispatchAs: AugmentedSubmittable<
        (
          asOrigin:
            | DanceboxRuntimeOriginCaller
            | { system: any }
            | { Void: any }
            | string
            | Uint8Array,
          call: Call | IMethod | string | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [DanceboxRuntimeOriginCaller, Call]
      >;
      /**
       * Send a batch of dispatch calls. Unlike `batch`, it allows errors and
       * won't interrupt.
       *
       * May be called from any origin except `None`.
       *
       * - `calls`: The calls to be dispatched from the same origin. The number of
       *   call must not exceed the constant: `batched_calls_limit` (available
       *   in constant metadata).
       *
       * If origin is root then the calls are dispatch without checking origin
       * filter. (This includes bypassing `frame_system::Config::BaseCallFilter`).
       *
       * ## Complexity
       *
       * - O(C) where C is the number of calls to be batched.
       */
      forceBatch: AugmentedSubmittable<
        (
          calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]
        ) => SubmittableExtrinsic<ApiType>,
        [Vec<Call>]
      >;
      /**
       * Dispatch a function call with a specified weight.
       *
       * This function does not check the weight of the call, and instead allows
       * the Root origin to specify the weight of the call.
       *
       * The dispatch origin for this call must be _Root_.
       */
      withWeight: AugmentedSubmittable<
        (
          call: Call | IMethod | string | Uint8Array,
          weight:
            | SpWeightsWeightV2Weight
            | { refTime?: any; proofSize?: any }
            | string
            | Uint8Array
        ) => SubmittableExtrinsic<ApiType>,
        [Call, SpWeightsWeightV2Weight]
      >;
      /** Generic tx */
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
  } // AugmentedSubmittables
} // declare module
