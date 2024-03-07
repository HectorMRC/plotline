use crate::{Error, Result};
use clap::{Args, Subcommand};
use plotline::{
    entity::{application::EntityRepository, Entity},
    event::application::EventRepository,
    experience::{
        application::{ConstraintFactory, ExperienceApplication, ExperienceRepository},
        Experience, Profile,
    },
    id::{Id, Identifiable},
};
use prettytable::Table;
use std::fmt::Display;

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
    #[arg(long, short)]
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

pub struct ExperienceCli<ExperienceRepo, EntityRepo, EventRepo, CnstFactory> {
    pub experience_app: ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>,
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    ExperienceCli<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: 'static + ExperienceRepository<Interval = EventRepo::Interval> + Sync + Send,
    EntityRepo: 'static + EntityRepository + Sync + Send,
    EventRepo: 'static + EventRepository + Sync + Send,
    CnstFactory: 'static + ConstraintFactory<EventRepo::Interval> + Sync + Send,
{
    /// Given an [ExperienceCommand], executes the corresponding logic.
    pub fn execute(&self, experience_cmd: ExperienceCommand) -> Result {
        let experience_id = experience_cmd
            .experience
            .map(TryInto::try_into)
            .transpose()?;

        if let Some(command) = experience_cmd.command {
            return self.execute_subcommand(command, experience_id);
        }

        Ok(())
    }

    fn execute_subcommand(
        &self,
        subcommand: ExperienceSubCommand,
        experience: Option<Id<Experience<EventRepo::Interval>>>,
    ) -> Result {
        match subcommand {
            ExperienceSubCommand::Save(args) => {
                let experience_id = experience.unwrap_or_default();
                self.experience_app
                    .save_experience(experience_id)
                    .with_entity(args.entity.map(TryInto::try_into).transpose()?)
                    .with_event(args.event.map(TryInto::try_into).transpose()?)
                    .with_profiles(args.terminal.then_some(Vec::default()));

                println!("{}", experience_id);
            }
            ExperienceSubCommand::List => {
                let experiences = self.experience_app.filter_experiences().execute()?;
                print!("{}", ManyExperiencesFmt::new(&experiences));
            }
            ExperienceSubCommand::Profile(args) => self.execute_profile_command(
                experience.ok_or(Error::MissingArgument("experience id"))?,
                args.entity.map(TryInto::try_into).transpose()?,
                args.command,
            )?,
        }

        Ok(())
    }

    fn execute_profile_command(
        &self,
        experience: Id<Experience<EventRepo::Interval>>,
        entity: Option<Id<Entity>>,
        command: Option<ProfileCommand>,
    ) -> Result {
        let Some(command) = command else {
            return self.execute_profile_command(experience, entity, Some(ProfileCommand::List));
        };

        match command {
            ProfileCommand::List => {
                let experience = self.experience_app.find_experience(experience).execute()?;
                let profile = entity.and_then(|entity| {
                    experience
                        .profiles()
                        .iter()
                        .find(|profile| profile.id() == entity)
                });

                if let Some(profile) = profile {
                    print!("{}", SingleProfileFmt::new(profile));
                } else {
                    print!("{}", ManyProfilesFmt::new(experience.profiles()));
                };
            }
            ProfileCommand::Save(_args) => {
                todo!()
            }
            ProfileCommand::Remove => {
                todo!()
            }
        }

        Ok(())
    }
}

struct ManyExperiencesFmt<'a, Intv> {
    experiences: &'a [Experience<Intv>],
}

impl<'a, Intv> Display for ManyExperiencesFmt<'a, Intv> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["ID", "ENTITY ID", "EVENT ID"]);
        self.experiences.iter().for_each(|experience| {
            table.add_row(row![&experience.id, &experience.entity, &experience.event]);
        });

        table.fmt(f)
    }
}

impl<'a, Intv> ManyExperiencesFmt<'a, Intv> {
    pub fn new(experiences: &'a [Experience<Intv>]) -> Self {
        Self { experiences }
    }
}

struct SingleProfileFmt<'a> {
    profile: &'a Profile,
}

impl<'a> Display for SingleProfileFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["ENTITY", self.profile.id()]);
        table.add_empty_row();

        self.profile.values().for_each(|(key, value)| {
            table.add_row(row![key, value]);
        });

        table.fmt(f)
    }
}

impl<'a> SingleProfileFmt<'a> {
    pub fn new(profile: &'a Profile) -> Self {
        Self { profile }
    }
}

struct ManyProfilesFmt<'a> {
    profiles: &'a [Profile],
}

impl<'a> Display for ManyProfilesFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["ID"]);
        self.profiles.iter().for_each(|profile| {
            table.add_row(row![profile.id()]);
        });

        table.fmt(f)
    }
}

impl<'a> ManyProfilesFmt<'a> {
    pub fn new(profiles: &'a [Profile]) -> Self {
        Self { profiles }
    }
}
