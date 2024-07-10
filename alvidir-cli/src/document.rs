use std::{fmt::Display, str::FromStr, sync::Arc, time::SystemTime};

use alvidir::{graph::{application::GraphApplication, Node}, id::Identify};
use clap::{Args, Subcommand};
use tokio::{fs::File, process::Command};
use tracing::{error, info};

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum DocumentSubCommand {
    /// Open all documents.
    Open,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct DocumentCommand {
    /// The id of the document.
    document: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<DocumentSubCommand>,
}

pub struct DocumentCli<T: Identify> {
    pub graph_app: Arc<GraphApplication<T>>,
}

impl<T> DocumentCli<T>
where 
    T: Identify + Node,
    T::Id: Display + FromStr + Clone,
    <T::Id as FromStr>::Err: 'static + std::error::Error + Sync + Send,
{
    pub async fn execute(&self, cli: DocumentCommand) -> anyhow::Result<()> {
        let doc_name = cli.document.map(|s| T::Id::from_str(&s)).transpose()?;
        if let Some(command) = cli.command {
            return self.execute_subcommand(command, doc_name).await;
        }

        Ok(())
    }

    async fn execute_subcommand(
        &self,
        subcommand: DocumentSubCommand,
        doc_name: Option<T::Id>,
    ) -> anyhow::Result<()> {
        match subcommand {
            DocumentSubCommand::Open => {
                let Some(doc_name) = doc_name else {
                    return Err(
                        clap::Error::new(clap::error::ErrorKind::MissingRequiredArgument).into(),
                    );
                };
                
                let opened_at = SystemTime::now();

                let mut cmd = Command::new("vim")
                    .arg(doc_name.to_string())
                    .stdout(std::io::stdout())
                    .spawn()?;

                let exit_status = cmd.wait().await?;
                if !exit_status.success() {
                    error!(status = exit_status.to_string(), "running document editor");
                    return Err(anyhow::Error::msg("aborted transaction"));
                }

                let file = File::open(doc_name.to_string()).await?;
                
                let created_at = file.metadata().await?.created()?;
                if created_at > opened_at {
                    info!(path = doc_name.to_string(), "document created");
                    // self.graph_app.graph = self.graph_app.graph.with_node();
                }

                let updated_at = file.metadata().await?.modified()?;
                if updated_at < opened_at {
                    info!(path = doc_name.to_string(), "document modified");
                    return self.graph_app.check(doc_name).await.map_err(Into::into);
                }

                info!(path = doc_name.to_string(), "document not modified");
            }
        };

        Ok(())
    }
}
