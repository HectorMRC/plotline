use super::{
    service::{CreateTimeline},
    Error,
};
use crate::cli::CliResult;
use clap::{Args, Subcommand};
use std::{
    io::{stderr, stdout, Write},
    sync::mpsc,
};

#[derive(Args)]
struct TimelineCreateArgs {
    /// The name of the timeline.
    #[arg(num_args(1..))]
    name: String,
    /// The uuid string of the timeline.
    #[arg(short, long)]
    id: Option<String>,
}

#[derive(Args)]
struct MomentCreateArgs {
    /// The uuid of the timeline where to create the moment.
    timeline: Option<String>,
    /// The uuid string of the moment.
    #[arg(short, long)]
    id: Option<String>,
}
