//! Reactor used to initialize a node.
#![allow(unreachable_code)]

use casper_node_macros::reactor;

use crate::{protocol::Message, reactor::validator, types::NodeId, utils::WithDir};

reactor!(Initializer {
  type Config = WithDir<validator::Config>;

  components: {
    chainspec_loader = has_effects ChainspecLoader(cfg.dir(), effect_builder);
    storage = Storage(&cfg.map_ref(|cfg| cfg.storage.clone()));
    contract_runtime = ContractRuntime(cfg.map_ref(|cfg| cfg.storage.clone()),
&cfg.value().contract_runtime, registry);   }

  events: {}

  requests: {
    StorageRequest -> storage;
    ContractRuntimeRequest -> contract_runtime;

    // No network traffic during initialization, just discard.
    // TODO: Allow for "hard" discard, resulting in a crash?
    NetworkRequest<NodeId, Message> -> !;
  }

  announcements: {
    ChainspecLoaderAnnouncement -> [!];
  }
});

// TODO: Metrics
// TODO: is_stopped

impl Initializer {
    /// Returns whether the initialization process completed successfully or not.
    pub fn stopped_successfully(&self) -> bool {
        self.chainspec_loader.stopped_successfully()
    }
}
