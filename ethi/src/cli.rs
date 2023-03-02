use clap::{Parser, Subcommand};

pub mod backups {
    /// Commands to manipulate backups.
    #[derive(clap::Subcommand, Debug)]
    pub enum Command {
        /// Merge all backups into one merge file.
        Merge,
        /// Remove duplicates in backup files.
        Prune,
    }
}

pub mod codex {
    /// Commands to manipulate the codex.
    #[derive(clap::Subcommand, Debug)]
    pub enum Command {
        /// Check whether bugs found on the codex has been fixed..
        Bugs,
        /// Fetch missing codex entry.
        Missing,
    }
}

pub mod json {
    /// Commands to manipulate the json output of `ethi`.
    #[derive(clap::Subcommand, Debug)]
    pub enum Command {
        /// Fetch all entries we can find the `codex_uri` of in the guide database.
        FetchAllMatchesFromGuide,
        /// Fetch missing codex entry.
        Refresh(RefreshCmd),
    }

    /// Intermediate structure to allow for an `Option`.
    /// Makes `json refresh` a valid command.
    #[derive(clap::Args, Debug)]
    pub struct RefreshCmd {
        /// Subcommand, if any.
        #[command(subcommand)]
        pub c: Option<Refresh>,
    }

    /// Commands to (partially) refresh the json output.
    #[derive(clap::Subcommand, Debug)]
    pub enum Refresh {
        /// Refresh guide jsons.
        Guide(RefreshGuideCmd),
        /// Refresh codex jsons.
        Codex(RefreshCodexCmd),
    }

    /// Intermediate structure to allow for an `Option`.
    /// Makes `json refresh guide` a valid command.
    #[derive(clap::Args, Debug)]
    pub struct RefreshGuideCmd {
        /// Subcommand, if any.
        #[command(subcommand)]
        pub c: Option<RefreshGuide>,
    }

    /// Commands to (partially) refresh the guide json output.
    #[derive(clap::Subcommand, Debug)]
    pub enum RefreshGuide {
        /// Refresh only items.
        Items,
        /// Refresh only monsters.
        Monsters,
        /// Refresh only pets.
        Pets,
        /// Refresh only skills.
        Skills,
        /// Refresh only static resources.
        Static,
    }

    /// Intermediate structure to allow for an `Option`.
    /// Makes `json refresh codex` a valid command.
    #[derive(clap::Args, Debug)]
    pub struct RefreshCodexCmd {
        /// Subcommand, if any.
        #[command(subcommand)]
        pub c: Option<RefreshCodex>,
    }

    /// Commands to (partially) refresh the codex json output.
    #[derive(clap::Subcommand, Debug)]
    pub enum RefreshCodex {
        /// Refresh only bosses.
        Bosses,
        /// Refresh only followers.
        Followers,
        /// Refresh only items.
        Items,
        /// Refresh only monsters.
        Monsters,
        /// Refresh only raids.
        Raids,
        /// Refresh only skills.
        Skills,
    }
}

pub mod match_ {
    /// Commands to match the guide data vs the codex data.
    #[derive(clap::Args, Debug)]
    pub struct Command {
        /// Whether to fix the mismatches when possible.
        #[arg(short, long, default_value_t = false)]
        pub fix: bool,
        /// Subcommand, if any.
        #[command(subcommand)]
        pub c: Option<Subcommand>,
    }

    /// Commands to (partially) match.
    #[derive(clap::Subcommand, Debug)]
    pub enum Subcommand {
        /// Match only items.
        Items,
        /// Match only monsters.
        Monsters,
        /// Match only pets.
        Pets,
        /// Match only skills.
        Skills,
        /// Match only status effects.
        StatusEffects,
    }
}

pub mod merge {
    /// Commands to manipulate merges.
    #[derive(clap::Subcommand, Debug)]
    pub enum Command {
        /// Match a merge
        Match(MatchCmd),
    }

    /// Commands to (partially) match the latest merge data with the codex.
    #[derive(clap::Args, Debug)]
    pub struct MatchCmd {
        /// Whether to fix the mismatches when possible.
        #[arg(short, long, default_value_t = false)]
        pub fix: bool,
        /// Subcommand, if any.
        #[command(subcommand)]
        pub c: Option<Match>,
    }

    #[derive(clap::Subcommand, Debug)]
    pub enum Match {
        // Match only items.
        Items,
        // Match only monsters.
        Monsters,
        // Match only pets.
        Pets,
        // Match only skills.
        Skills,
        // Match only status effects.
        StatusEffects,
    }
}

pub mod translation {
    /// Commands to manipulate translations.
    #[derive(clap::Subcommand, Debug)]
    pub enum Command {
        /// Fetch missing translations.
        Missing,
        /// Fetch missing translations.
        Fetch(FetchCmd),
    }

    /// Command to fetch data in a specific locale.
    #[derive(clap::Args, Debug)]
    pub struct FetchCmd {
        /// The locale in which to query.
        pub locale: String,
    }
}

/// Base enum for subcommands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Subcommand to manipulate backups.
    #[command(subcommand)]
    Backups(backups::Command),
    /// Subcommand to manipulate the codex.
    #[command(subcommand)]
    Codex(codex::Command),
    /// Subcommand to manipulate the json output.
    #[command(subcommand)]
    Json(json::Command),
    /// Subcommand to match the guide data vs the codex data.
    Match(match_::Command),
    /// Subcommand to manipulate merges.
    #[command(subcommand)]
    Merge(merge::Command),
    /// Subcommand to manipulate translations.
    #[command(subcommand)]
    Translation(translation::Command),
}

/// Program arguments.
#[derive(Parser, Debug)]
#[command(
    author = "ethiraric",
    version = "0.0.1",
    about = "A program to help programatically manipulate the Orna guide and codex",
    long_about = None
)]
pub struct Cli {
    /// Subcommand, if any.
    #[command(subcommand)]
    pub command: Option<Command>,
}
