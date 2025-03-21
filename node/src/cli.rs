// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    crate::{chain_spec::RawGenesisConfig, service::Sealing},
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    sc_cli::{CliConfiguration, NodeKeyParams, SharedParams},
    sc_network::config::MultiaddrWithPeerId,
    sp_runtime::traits::Get,
    std::{collections::BTreeMap, path::PathBuf},
    tp_container_chain_genesis_data::json::properties_to_map,
};

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
    /// Build a chain specification.
    BuildSpec(BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Remove the whole chain.
    PurgeChain(cumulus_client_cli::PurgeChainCmd),

    /// Export the genesis state of the parachain.
    ExportGenesisState(ExportGenesisStateCommand),

    /// Export the genesis wasm of the parachain.
    ExportGenesisWasm(ExportGenesisWasmCommand),

    /// Sub-commands concerned with benchmarking.
    /// The pallet benchmarking moved to the `pallet` sub-command.
    #[command(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// Try some testing command against a specified runtime state.
    #[cfg(feature = "try-runtime")]
    TryRuntime(try_runtime_cli::TryRuntimeCmd),

    /// Errors since the binary was not build with `--features try-runtime`.
    #[cfg(not(feature = "try-runtime"))]
    TryRuntime,

    /// Key management cli utilities
    #[command(subcommand)]
    Key(KeyCmd),
}

/// The `build-spec` command used to build a specification.
#[derive(Debug, Clone, clap::Parser)]
pub struct BuildSpecCmd {
    #[clap(flatten)]
    pub base: sc_cli::BuildSpecCmd,

    /// Id of the parachain this spec is for. Note that this overrides the `--chain` param.
    #[arg(long)]
    pub parachain_id: Option<u32>,

    /// List of container chain chain spec paths to add to genesis.
    #[arg(long)]
    pub add_container_chain: Vec<String>,
}

impl CliConfiguration for BuildSpecCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.base.shared_params
    }

    fn node_key_params(&self) -> Option<&NodeKeyParams> {
        Some(&self.base.node_key_params)
    }
}

/// Command for exporting the genesis state of the parachain
#[derive(Debug, clap::Parser)]
pub struct ExportGenesisStateCommand {
    /// Output file name or stdout if unspecified.
    pub output: Option<PathBuf>,

    /// Id of the parachain this state is for.
    #[arg(long)]
    pub parachain_id: Option<u32>,

    /// Write output in binary. Default is to write in hex.
    #[arg(short, long)]
    pub raw: bool,

    /// The name of the chain for that the genesis state should be exported.
    #[arg(long)]
    pub chain: Option<String>,
}

/// Command for exporting the genesis wasm file.
#[derive(Debug, clap::Parser)]
pub struct ExportGenesisWasmCommand {
    /// Output file name or stdout if unspecified.
    pub output: Option<PathBuf>,

    /// Write output in binary. Default is to write in hex.
    #[arg(short, long)]
    pub raw: bool,

    /// The name of the chain for that the genesis wasm file should be exported.
    #[arg(long)]
    pub chain: Option<String>,
}

#[derive(Debug, clap::Parser)]
#[group(skip)]
pub struct RunCmd {
    #[clap(flatten)]
    pub base: cumulus_client_cli::RunCmd,

    /// Enable the development service to run without a backing relay chain
    #[arg(long)]
    pub dev_service: bool,

    /// When blocks should be sealed in the dev service.
    ///
    /// Options are "instant", "manual", or timer interval in milliseconds
    #[arg(long, default_value = "instant")]
    pub sealing: Sealing,

    /// Id of the parachain this collator collates for.
    #[arg(long)]
    pub parachain_id: Option<u32>,
}

impl std::ops::Deref for RunCmd {
    type Target = cumulus_client_cli::RunCmd;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum KeyCmd {
    #[command(flatten)]
    BaseCli(sc_cli::KeySubcommand),
}

impl KeyCmd {
    /// run the key subcommands
    pub fn run<C: sc_cli::SubstrateCli>(&self, cli: &C) -> Result<(), sc_cli::Error> {
        match self {
            KeyCmd::BaseCli(cmd) => cmd.run(cli),
        }
    }
}

#[derive(Debug, clap::Parser)]
#[command(
    propagate_version = true,
    args_conflicts_with_subcommands = true,
    subcommand_negates_reqs = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: RunCmd,

    /// Disable automatic hardware benchmarks.
    ///
    /// By default these benchmarks are automatically ran at startup and measure
    /// the CPU speed, the memory bandwidth and the disk speed.
    ///
    /// The results are then printed out in the logs, and also sent as part of
    /// telemetry, if telemetry is enabled.
    #[arg(long)]
    pub no_hardware_benchmarks: bool,

    /// Optional parachain id that should be used to build chain spec.
    #[arg(long)]
    pub para_id: Option<u32>,

    /// Relay chain arguments, optionally followed by "--" and orchestrator chain arguments
    #[arg(raw = true)]
    extra_args: Vec<String>,
}

impl Cli {
    pub fn relaychain_args(&self) -> &[String] {
        let (relay_chain_args, _) = self.split_extra_args_at_first_dashdash();

        relay_chain_args
    }

    pub fn container_chain_args(&self) -> &[String] {
        let (_, container_chain_args) = self.split_extra_args_at_first_dashdash();

        container_chain_args
    }

    fn split_extra_args_at_first_dashdash(&self) -> (&[String], &[String]) {
        let index_of_dashdash = self.extra_args.iter().position(|x| *x == "--");

        if let Some(i) = index_of_dashdash {
            let (container_chain_args, extra_extra) = self.extra_args.split_at(i);
            (&extra_extra[1..], container_chain_args)
        } else {
            // Only relay chain args
            (&self.extra_args, &[])
        }
    }
}

#[derive(Debug)]
pub struct RelayChainCli {
    /// The actual relay chain cli object.
    pub base: polkadot_cli::RunCmd,

    /// Optional chain id that should be passed to the relay chain.
    pub chain_id: Option<String>,

    /// The base path that should be used by the relay chain.
    pub base_path: PathBuf,
}

impl RelayChainCli {
    /// Parse the relay chain CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        relay_chain_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
        let chain_id = extension.map(|e| e.relay_chain.clone());
        let base_path = para_config.base_path.path().join("polkadot");

        Self {
            base_path,
            chain_id,
            base: clap::Parser::parse_from(relay_chain_args),
        }
    }
}

/// The `run` command used to run a container chain node.
#[derive(Debug, clap::Parser, Clone)]
#[group(skip)]
pub struct ContainerChainRunCmd {
    /// The cumulus RunCmd inherits from sc_cli's
    #[command(flatten)]
    pub base: sc_cli::RunCmd,

    /// Run node as collator.
    ///
    /// Note that this is the same as running with `--validator`.
    #[arg(long, conflicts_with = "validator")]
    pub collator: bool,

    /// Optional container chain para id that should be used to build chain spec.
    #[arg(long)]
    pub para_id: Option<u32>,
}

#[derive(Debug)]
pub struct ContainerChainCli {
    /// The actual container chain cli object.
    pub base: ContainerChainRunCmd,

    /// The base path that should be used by the container chain.
    pub base_path: PathBuf,

    /// The ChainSpecs that this struct can initialize. This starts empty and gets filled
    /// by calling preload_chain_spec_file.
    pub preloaded_chain_spec: Option<Box<dyn sc_chain_spec::ChainSpec>>,
}

impl Clone for ContainerChainCli {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            base_path: self.base_path.clone(),
            preloaded_chain_spec: self.preloaded_chain_spec.as_ref().map(|x| x.cloned_box()),
        }
    }
}

impl ContainerChainCli {
    /// Parse the container chain CLI parameters using the para chain `Configuration`.
    pub fn new<'a>(
        para_config: &sc_service::Configuration,
        container_chain_args: impl Iterator<Item = &'a String>,
    ) -> Self {
        let base_path = para_config.base_path.path().join("containers");

        Self {
            base_path,
            base: clap::Parser::parse_from(container_chain_args),
            preloaded_chain_spec: None,
        }
    }

    pub fn chain_spec_from_genesis_data<MaxLengthTokenSymbol: Get<u32>>(
        para_id: u32,
        genesis_data: ContainerChainGenesisData<MaxLengthTokenSymbol>,
        chain_type: sc_chain_spec::ChainType,
        relay_chain: String,
        boot_nodes: Vec<String>,
    ) -> Result<crate::chain_spec::RawChainSpec, String> {
        let name = String::from_utf8(genesis_data.name).map_err(|_e| "Invalid name".to_string())?;
        let id: String =
            String::from_utf8(genesis_data.id).map_err(|_e| "Invalid id".to_string())?;
        let storage_raw: BTreeMap<_, _> =
            genesis_data.storage.into_iter().map(|x| x.into()).collect();
        let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
            .into_iter()
            .map(|x| {
                x.parse::<MultiaddrWithPeerId>()
                    .map_err(|e| format!("{}", e))
            })
            .collect::<Result<_, _>>()?;
        let telemetry_endpoints = None;
        let protocol_id = Some(format!("container-chain-{}", para_id));
        let fork_id = genesis_data
            .fork_id
            .map(|fork_id| String::from_utf8(fork_id).map_err(|_e| "Invalid fork_id".to_string()))
            .transpose()?;
        let properties = Some(
            properties_to_map(&genesis_data.properties)
                .map_err(|e| format!("Invalid properties: {}", e))?,
        );
        let extensions = crate::chain_spec::Extensions {
            relay_chain,
            para_id,
        };
        let chain_spec = crate::chain_spec::RawChainSpec::from_genesis(
            &name,
            &id,
            chain_type,
            move || RawGenesisConfig {
                storage_raw: storage_raw.clone(),
            },
            boot_nodes,
            telemetry_endpoints,
            protocol_id.as_deref(),
            fork_id.as_deref(),
            properties,
            // TODO: what to do with extensions? We are hardcoding the relay_chain and the para_id, any
            // other extensions are being ignored
            extensions,
        );

        Ok(chain_spec)
    }

    pub fn preload_chain_spec_from_genesis_data<MaxLengthTokenSymbol: Get<u32>>(
        &mut self,
        para_id: u32,
        genesis_data: ContainerChainGenesisData<MaxLengthTokenSymbol>,
        chain_type: sc_chain_spec::ChainType,
        relay_chain: String,
        boot_nodes: Vec<String>,
    ) -> Result<(), String> {
        let chain_spec = Self::chain_spec_from_genesis_data(
            para_id,
            genesis_data,
            chain_type,
            relay_chain,
            boot_nodes,
        )?;
        self.preloaded_chain_spec = Some(Box::new(chain_spec));

        Ok(())
    }
}
