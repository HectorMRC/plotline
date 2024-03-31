use crate::{display::DisplayTable, Error, Result};
use clap::{Args, Subcommand};
use plotline::{
    entity::{application::EntityRepository, Entity},
    event::application::EventRepository,
    experience::{
        application::{ExperienceApplication, ExperienceRepository, PluginFactory},
        Experience, Profile,
    },
    id::{Id, Identifiable},
};
use prettytable::row;

// const KEY_VALUE_SEPARATOR: char = '=';

#[derive(Args)]
struct ProfileSaveArgs {
    /// A key-value pair expressed as key=value.
    #[clap(short, long, alias = "value")]
    values: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum ProfileCommand {
    /// Save a profile.
    Save(ProfileSaveArgs),
    /// List all profiles.
    #[clap(alias = "ls")]
    List,
    /// Remove a profile.
    #[clap(alias = "rm")]
    Remove,
}

#[derive(Args)]
struct ProfileArgs {
    /// The id of the profile's entity.
    entity: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<ProfileCommand>,
}

#[derive(Args)]
struct ExperienceSaveArgs {
    /// The id of the entity involved in the experience.
    #[arg(long, short = 'n')]
    entity: Option<String>,
    /// The id of the event causing the experience.
    #[arg(long, short)]
    event: Option<String>,
    /// Mark the experience as terminal.
    #[clap(short, long)]
    terminal: bool,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum ExperienceSubCommand {
    /// Save an experience.
    Save(ExperienceSaveArgs),
    /// List all experiences.
    #[command(alias("ls"))]
    List,
    /// Manage profiles.
    Profile(ProfileArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ExperienceCommand {
    /// The id of the experience.
    experience: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<ExperienceSubCommand>,
}

pub struct ExperienceCli<ExperienceRepo, EntityRepo, EventRepo, PluginFcty> {
    pub experience_app: ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>,
}

impl<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
    ExperienceCli<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    EntityRepo: 'static + EntityRepository + Sync + Send,
    EventRepo: 'static + EventRepository + Sync + Send,
    ExperienceRepo: 'static + ExperienceRepository<Intv = EventRepo::Intv> + Sync + Send,
    PluginFcty: 'static + PluginFactory<Intv = EventRepo::Intv> + Sync + Send,
{
    /// Given an [ExperienceCommand], executes the corresponding logic.
    pub async fn execute(&self, experience_cmd: ExperienceCommand) -> Result {
        let experience_id = experience_cmd
            .experience
            .map(TryInto::try_into)
            .transpose()?;

        if let Some(command) = experience_cmd.command {
            return self.execute_subcommand(command, experience_id).await;
        }

        Ok(())
    }

    async fn execute_subcommand(
        &self,
        subcommand: ExperienceSubCommand,
        experience: Option<Id<Experience<EventRepo::Intv>>>,
    ) -> Result {
        match subcommand {
            ExperienceSubCommand::Save(args) => {
                let experience_id = experience.unwrap_or_default();
                self.experience_app
                    .save_experience(experience_id)
                    .with_entity(args.entity.map(TryInto::try_into).transpose()?)
                    .with_event(args.event.map(TryInto::try_into).transpose()?)
                    .with_profiles(args.terminal.then_some(Vec::default()))
                    .execute()
                    .await?;

                println!("{}", experience_id);
            }
            ExperienceSubCommand::List => {
                let experiences = self.experience_app.filter_experiences().execute().await?;
                DisplayTable::new(&experiences).show(|table, experiences| {
                    table.add_row(row!["ID", "ENTITY ID", "EVENT ID"]);
                    experiences.iter().for_each(|experience| {
                        table.add_row(row![
                            &experience.id,
                            &experience.entity.id(),
                            &experience.event.id()
                        ]);
                    });
                });
            }
            ExperienceSubCommand::Profile(args) => {
                self.execute_profile_command(
                    experience.ok_or(Error::MissingArgument("experience id"))?,
                    args.entity.map(TryInto::try_into).transpose()?,
                    args.command,
                )
                .await?
            }
        }

        Ok(())
    }

    async fn execute_profile_command(
        &self,
        experience: Id<Experience<EventRepo::Intv>>,
        entity: Option<Id<Entity>>,
        command: Option<ProfileCommand>,
    ) -> Result {
        let Some(command) = command else {
            return self.list_profiles(experience, entity).await;
        };

        match command {
            ProfileCommand::List => self.list_profiles(experience, entity),
            ProfileCommand::Save(_args) => {
                todo!()
            }
            ProfileCommand::Remove => {
                todo!()
            }
        }
        .await
    }

    async fn list_profiles(
        &self,
        experience: Id<Experience<EventRepo::Intv>>,
        entity: Option<Id<Entity>>,
    ) -> Result {
        let experience = self
            .experience_app
            .find_experience(experience)
            .execute()
            .await?;
        let profile: Option<&Profile> = entity.and_then(|entity| {
            experience
                .profiles()
                .iter()
                .find(|profile| profile.id() == entity)
        });

        if let Some(profile) = profile {
            DisplayTable::new(profile).show(|table, profile| {
                table.add_row(row!["ENTITY", profile.id()]);
                table.add_empty_row();

                profile.values().for_each(|(key, value)| {
                    table.add_row(row![key, value]);
                });
            });
        } else {
            DisplayTable::new(&experience.profiles()).show(|table, profiles| {
                table.add_row(row!["ID"]);
                profiles.iter().for_each(|profile| {
                    table.add_row(row![profile.id()]);
                });
            });
        };

        Ok(())
    }
}
