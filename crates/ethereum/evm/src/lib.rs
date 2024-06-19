//! EVM config for vanilla ethereum.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use reth_evm::{ConfigureEvm, ConfigureEvmEnv};
use reth_revm::{Database, EvmBuilder};

pub mod execute;

/// Ethereum DAO hardfork state change data.
pub mod dao_fork;

/// [EIP-6110](https://eips.ethereum.org/EIPS/eip-6110) handling.
pub mod eip6110;

/// Ethereum-related EVM configuration.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct EthEvmConfig;

impl ConfigureEvmEnv for EthEvmConfig {}

impl ConfigureEvm for EthEvmConfig {
    type DefaultExternalContext<'a> = ();

    fn evm<'a, DB: Database + 'a>(
        &self,
        db: DB,
    ) -> reth_revm::Evm<'a, Self::DefaultExternalContext<'a>, DB> {
        EvmBuilder::default().with_db(db).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_primitives::{
        revm_primitives::{BlockEnv, CfgEnv, SpecId},
        ChainSpec, Header, U256,
    };
    use revm_primitives::CfgEnvWithHandlerCfg;

    #[test]
    #[ignore]
    fn test_fill_cfg_and_block_env() {
        let mut cfg_env = CfgEnvWithHandlerCfg::new_with_spec_id(CfgEnv::default(), SpecId::LATEST);
        let mut block_env = BlockEnv::default();
        let header = Header::default();
        let chain_spec = ChainSpec::default();
        let total_difficulty = U256::ZERO;

        EthEvmConfig::fill_cfg_and_block_env(
            &mut cfg_env,
            &mut block_env,
            &chain_spec,
            &header,
            total_difficulty,
        );

        assert_eq!(cfg_env.chain_id, chain_spec.chain().id());
    }
}
