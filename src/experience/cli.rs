use clap::{Args, Subcommand};

#[derive(Args)]
struct ExperienceSaveArgs {
    /// The id of the entities resulting from the experience.
    #[clap(short, long, value_delimiter = ',')]
    after: Vec<String>,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum ExperienceSubCommand {
    /// Save an experience.
    Save(ExperienceSaveArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ExperienceCommand {
    /// The id of the experience.
    #[clap(value_parser, num_args = 2, value_delimiter = ' ')]
    experience: Option<Vec<String>>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<ExperienceSubCommand>,
}
