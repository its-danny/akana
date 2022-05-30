<div align="center">
  <h1>🌤️ akana</h1>

A casual MUD with a focus on crafting, minigames, and socializing.

[![Bevy](https://img.shields.io/badge/bevy-0.7.0-lightgrey)](https://bevyengine.org/)
[![Dependency Status](https://deps.rs/repo/github/its-danny/akana/status.svg)](https://deps.rs/repo/github/its-danny/akana)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-pink.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/github/license/its-danny/akana)](LICENSE)

</div>

## Status

Very, _very_, **very** early stages. I'm still pretty new to Rust & Bevy, so this code
is probably a disaster. I've tried to keep things organized via [plugins](https://bevyengine.org/learn/book/getting-started/plugins/)
but it's a work in progress.

## Development

A few things will need to be installed first:

- [rust](https://rustup.rs/)
- [diesel_cli](https://github.com/diesel-rs/diesel) for generating migrations
- [docker-compose](https://docs.docker.com/compose/) to handle PostgreSQL
- [koji](https://github.com/its-danny/koji) to keep commits nice and tidy

### Get it running

Start all the necessary services.

```bash
$ cp .env.example .env # The defaults should work fine
$ docker-compose up    # Starts PostgreSQL
$ cargo run -p api     # Starts the API
$ cargo run -p server  # Starts the game server
```

Run migrations.

```bash
$ export $(xargs < .env)
$ diesel migration run --migration-dir database/migrations
```

Join the game!

```bash
$ telnet localhost 4000
```

## Contributing

As this is more of a passion project than anything, I'm not looking
for feature requests. Bug fixes, performance improvements, code quality refactors,
etc are all welcome, though.
