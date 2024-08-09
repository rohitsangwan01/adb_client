use clap::Parser;

use crate::models::RebootTypeCommand;

#[derive(Parser, Debug)]
pub enum LocalCommand {
    /// List available server features.
    HostFeatures,
    /// Push a file on device
    Push { filename: String, path: String },
    /// Pull a file from device
    Pull { path: String, filename: String },
    /// List a directory on device
    List { path: String },
    /// Stat a file specified on device
    Stat { path: String },
    /// Spawn an interactive shell or run a list of commands on the device
    Shell { command: Vec<String> },
    /// Reboot the device
    Reboot {
        #[clap(subcommand)]
        sub_command: RebootTypeCommand,
    },
    /// Dump framebuffer of device
    Framebuffer { path: String },
}
