use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct Options {
    #[clap(long)]
    /// LDAP domain.
    pub ldap_domain: Option<String>,
}
