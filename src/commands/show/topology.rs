// src/commands/show/topology.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TopologyCommands {
    /// Show physical topology
    Physical,
    /// Show site topology
    Sites,
    /// Show L3 topology by type (OSPF or IS-IS)
    L3 {
        /// Topology type (e.g. OSPF, IS-IS)
        topology_type: String,
    },
    /// Show VLAN names
    Vlans,
}
