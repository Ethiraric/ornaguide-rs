# A set of tools for manipulating Orna data
This repository started by aiming at being a client-side library for the public
API of orna.guide. Its path has since long changed, although it still is
closely related to orna.guide.

The client API has since been removed, as a new one contained within this
repository should replace it.

This repository could make use of a restructuring due to its tortuous history.

# Brief overview
This repository is a workspace comprising 3 projects:
  - `ornaguide-rs`: This is the base library for other projects. It contains
    utilities allowing the manipulation of data from both orna.guide and
    playorna.com. 
  - `ethi`: This started as an example for the `ornaguide-rs` library. It
    contains utilities which help the Research Team keep orna.guide up-to-date.
    This binary makes heavy use of the admin interface of orna.guide, and is
    aimed at the Research Team members.
  - `api`: A public API for the guide data. The public API of orna.guide is
    incomplete and harder to maintain. This project aims at replacing it while
    offering more features. It can be set up from JSON extracts from the guide,
    which means that an admin account on orna.guide is not necessary.

# `ornaguide-rs`
## Admin view
`src/guide.rs` contains the `AdminGuide` trait. This trait defines methods used
to both query and edit the guide. The structures used in this trait are located
in (`src/{pets,skills,items,monsters}`). In each of these directories, an
`admin` module contains structures that are used when retrieving data from the
admin view. The admin view is not publicly available, and this module can only
be used by the Research Team.

Other entities can be found in `src/guide/static.rs`, such as Monster Families,
Elements, Status effects, ...

The `AdminGuide` trait (and `OrnaAdminGuide` implementation) allow the user to
interact with the Django admin view from the guide. They act as a web client,
retrieving HTML pages, parsing them and returning the data it contains, and
POSTing forms in a format Django recognizes.

Though not all entities are available, the main ones are:
  * Items (and types, categories, useable classes, ...)
  * Monsters (and their statuses, elemental resistances, families, spawns, ...)
  * Skills (and their element, type, ...)

```
TODO(ethiraric, 13/07/2022): Code examples
```

## Codex parsing
The `Codex` trait (and `OrnaAdminGuide` implementation) allow the user to
retrieve data from playorna's codex. There again, we retrieve HTML pages and
parse them. We, of course, only have read access to the codex.

Event entities, which are still on the codex but are not searchable, can still
be retrieved.

## Offline saves
The `Http` struct saves the body of each `GET` request that is not looping to
`localhost`. This allows for backups and fast querying, setting up a webserver
locally to serve those pages and both not spam the guide and codex.

The `justfile` at the root of the repository has commands to back up the HTML
files.

## JSON exports / imports
The data we fetch from the guide and the codex can be exported to JSON. This
helps finding changes and is a fast and cheap way to keep historical data.

Additionally, JSONs can be imported on the API, removing the need of an admin
account on the guide to access the data.

The `justfile` at the root of the repository has commands to back up the JSON
files.

# `ethi`
This started as just a `main.rs`, but it quickly grew out of hand, and
eventually was turned into its own project.

In it is a tool to help the guide team find discrepancies between the guide and
the codex. Of course, we expect the codex to be the source of truth, but there
are some TODOs left in the code for some details Northen Forge has overlooked.
The program is able to quickfix most of them.

# `api`
This is what (I hope) will become the new API for Orna data. It features
powerful filtering abilities.

Some entities are yet to be added to it (Elements, Families, Spawns, ...) and
its documentation is unfortunately yet to be written.
