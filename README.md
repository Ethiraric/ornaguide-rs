# A client-side API for orna.guide (or so it started)
This repository started by aiming at being a client-side library for the public
API of orna.guide. Its path has since long changed, although it still is
closely related to orna.guide.

The client API is still incomplete, but different features that have been added
may be noteworthy.

This repository could make use of a restructuring due to its tortuous history.

# Brief client architecture overview
The game entities queryable from the API are located in their respective
directories (`src/{pets,skills,items,monsters}`). In each of these directories
are an `{entity}`, an `admin` and a `raw` modules. The `{entity}` module is the
main module containing parsed data from the guide. The enums or structures in
that module are the easiest one to work with when building on this library. The
`raw` module contains the raw data structures deserialized from the jsons. They
contain lots of `Option`s that have been removed in the `{entity}` module.
Finally, the `admin` module contains structures that are used when retrieving
data from the admin view. The admin view is not publicly available, and this
module can only be used by the Research Team.

```
TODO(ethiraric, 13/07/2022): Code examples
```

# Additional features
## Admin utilities
The `AdminGuide` trait (and `OrnaAdminGuide` implementation) allow the user to
interact with the Django admin view from the guide. They act as a web client,
retrieving HTML pages, parsing them and returning the data it contains, and
POSTing forms in a format Django recognizes.

Though not all entities are available, the main ones are:
  * Items (and types, categories, useable classes, ...)
  * Monsters (and their statuses, elemental resistances, families, spawns, ...)
  * Skills (and their element, type, ...)

## Codex parsing
The `Codex` trait (and `OrnaAdminGuide` implementation) allow the user to
retrieve data from playorna's codex. There again, we retrieve HTML pages and
parse them. We, of course, only have read access to the codex.

Event entities, which are still on the codex but are not searchable, can still
be retrieved.

## The `ethi` example
This started as just a `main.rs`, but it quickly grew out of hand. The code
here should be a separate application in a different crate. It is by no means
an _example_, but rather my workspace.

In it is a tool to help the guide team find discrepancies between the guide and
the codex. Of course, we expect the codex to be the source of truth, but there
are some TODOs left in the code for some details Northen Forge has overlooked.
The program is able to quickfix most of them.

Added to that is the ability to export (and import) that data from json files.

Finally, the tool also caches any request that is made to both the guide and
the codex. This allows setting up a webserver locally to serve those pages and
both not spam the guide and codex, and iterating faster when reloading all
items.

The `justfile` at the root of the repository has commands to back up both the
JSON and the HTML files.
