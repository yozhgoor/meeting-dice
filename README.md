# chair-chooser

[![actions status][actions-badge]][actions-url]
[![crate version][crates-version-badge]][crates-url]
[![dependencies status][deps-badge]][deps-url]
![licenses][licenses-badge]

[actions-badge]: https://github.com/yozhgoor/chair-chooser/workflows/main/badge.svg
[actions-url]: https://github.com/yozhgoor/chair-chooser/actions
[crates-version-badge]: https://img.shields.io/crates/v/chair-chooser
[crates-url]: https://crates.io/crates/cargo-temp
[deps-badge]: https://deps.rs/crate/chair-chooser/0.1.0/status.svg
[deps-url]: https://deps.rs/crate/chair-chooser
[licenses-badge]: https://img.shields.io/crates/l/chair-chooser

A CLI tool that helps you to choose in a team who's gonna be the next meeting chair.

## Installation

`cargo install chair-chooser`

## Usage

## List the members of the team

```
chair-chooser --list
```

This will print:

* The members of the team.
* Member(s) that will not participate to the meeting.
* An eventual last meeting chair.

## Specify who was the last meeting chair

```
chair-chooser --last-chair <name>
```

This command will return an error if the `name` doesn't exists in the members list.

## Add member(s) to the team

```
chair-chooser --add-members <names>
```

If a name already exists in the members list, it will not be added.
Note that this is checked on lowercase values so `John == john`.

## Remove member(s) of the team.

```
chair-chooser --remove-members <names>
```

## Select a meeting chair

Since all the options that we mentioned before can be used without actually choosing a new meeting
chair, you need to pass `--run` to randomly select the new meeting chair.

```
chair-chooser --run
```

If you want to list the members before the run, you can use:

```
chair-chooser --list --run
```

After printing the selection, the program will ask if you confirm the selection and store it for the
next run.

### Remove temporarily member(s) of the team

If a member of the team can't be present for this meeting, you can remove him from this run like
this:

```
chair-chooser --hide-members <names> --run
```
