# Snapr

Snapr is a simple snap ([snapcraft.io](http://snapcraft.io)) store.

I like to use snapd in a project where a local and private store is needed.
There was a open source implementation earlier but it stopped working and
was removed and at the moment there are no way to replace the official
store.

With this I hope to implement some of the endpoints to search and install
snaps with working channels and automatic updates.

## Endpoints

## find

Implement `/snaps/search` endpoint, no need to support all options.

* [x] [ ] GET /snaps/search?confinement=..&fields=..&q=..
* [x] [ ] GET /snaps/search?confinement=..&fields=..&section=..&q=..

## info

Implement `/snaps/details/<name>`

* [x] [ ] GET /snaps/details/<name>?channel=...&fields=...

## install

* [ ][ ] ?
