<div align="center">
  <h1>🌤️ akana</h1>

A casual MUD with a focus on crafting, minigames, and socializing.

[![Dependency Status](https://deps.rs/repo/github/its-danny/akana/status.svg)](https://deps.rs/repo/github/its-danny/akana)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-pink.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/github/license/its-danny/akana)](LICENSE)

</div>

## Status

Very, _very_, **very** early stages.

## Development

A few extra tools are necessary for development.

- [cargo-bump](https://github.com/wraithan/cargo-bump)
- [cocogitto](https://github.com/cocogitto/cocogitto)
- [diesel_cli](https://github.com/diesel-rs/diesel)
- [docker-compose](https://docs.docker.com/compose/)
- [koji](https://github.com/its-danny/koji)

### Get it running

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
