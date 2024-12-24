use std::{
    error::Error,
    fmt::Debug,
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use alvidir::{
    command::Command,
    document::{lazy::LazyDocument, DocumentRepository},
    id::Identify,
    schema::{delete::Delete, save::Save, Schema},
};
use anyhow::Result;
use clap::{Args, Subcommand};

/// A file-system document.
#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
}

impl Identify for Document {
    type Id = PathBuf;

    fn id(&self) -> &Self::Id {
        &self.path
    }
}

#[derive(Args)]
struct DocumentSaveArgs {
    /// The content of the node.
    content: Option<String>,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum DocumentSubCommand {
    /// Delete a document.
    Delete,
    /// List all documents.
    #[command(alias("ls"))]
    List,
    /// Save a document.
    Save(DocumentSaveArgs),
}

#[derive(Args)]
pub struct DocumentCommand {
    /// The id of the document.
    id: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    subcommand: DocumentSubCommand,
}

pub struct DocumentCli<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
{
    pub schema: Arc<Schema<LazyDocument<DocumentRepo>>>,
    pub document_repo: Arc<DocumentRepo>,
}

impl<DocumentRepo> DocumentCli<DocumentRepo>
where
    DocumentRepo: 'static + DocumentRepository<Document = Document>,
    DocumentRepo::Document: Debug,
    <DocumentRepo::Document as Identify>::Id: Ord + Clone + FromStr + Debug,
    <<DocumentRepo::Document as Identify>::Id as FromStr>::Err: 'static + Error + Sync + Send,
{
    pub fn execute(&self, command: DocumentCommand) -> Result<()> {
        let document_id = || {
            command
                .id
                .map(|id| <DocumentRepo::Document as Identify>::Id::from_str(&id))
                .transpose()
                .map_err(anyhow::Error::new)?
                .ok_or(anyhow::Error::msg("node id must be set"))
        };

        match command.subcommand {
            DocumentSubCommand::Delete => Delete::new(document_id()?).execute(&self.schema)?,
            DocumentSubCommand::List => {
                let mut stdout = io::stdout().lock();
                self.schema
                    .read()
                    .into_iter()
                    .for_each(|node| writeln!(stdout, "{:?}", node.id()).unwrap());
            }
            DocumentSubCommand::Save(args) => {
                let document_id = document_id()?;
                let document = Document {
                    path: document_id.clone(),
                    bytes: args.content.map(|s| s.into_bytes()).unwrap_or_default(),
                };

                Save::new(LazyDocument::new(self.document_repo.clone(), document))
                    .execute(&self.schema)?;
            }
        };

        Ok(())
    }
}
