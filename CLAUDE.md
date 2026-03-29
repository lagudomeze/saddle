# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Saddle** is a Rust-based agent harness system that combines a CLI/TUI interface with LLM integration, local memory (SQLite), and a WASM plugin runtime. It uses a `harness/features.json` file to track feature development progress.

## Build & Test Commands

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo test <test>  # Run a specific test (e.g., cargo test test_load_features)
cargo run          # Run the application (defaults to TUI mode)
cargo run -- --help # Show CLI help
```

## Architecture

### Entry Point Flow

`main.rs` → parses CLI → loads Config → initializes logging → delegates to `Cli::run()`

### Module Organization (via `src/lib.rs`)

- **cli** - Clap-based commands: `Run` (TUI mode), `Init`, `Status`
- **config** - `Settings` struct in `settings.rs`, `ConfigLoader` in `loader.rs` reads `~/.saddle/config.toml`
- **harness** - Manages `harness/features.json` (FeatureManager), `harness/progress.md` (ProgressTracker), `harness/handoff.md` (HandoffGenerator)
- **llm** - LLM integration via `rig-core` using the Responses API (`rig::providers::openai::responses_api`)
- **agent** - Wraps rig agents with preset system prompts in `agent::presets` (assistant, code_assistant, researcher, critic)
- **memory** - SQLite-backed via `rusqlite`; `MemoryStore` has `insert()` and `search()` (vector search not yet implemented)
- **plugins** - WASM runtime stub via `wasmtime`; `PluginRuntime::load_plugin()` and `list_plugins()` are placeholders
- **tui** - `ratatui`-based app with theme support (nord, dracula, monokai via `Theme` struct)

### Error Handling

All errors use `SaddleError` enum (in `src/utils.rs`) with domain-specific variants. The `exn` crate provides the `SaddleResult<T> = exn::Result<T, SaddleError>` type alias. Errors are propagated via `.or_raise()` which converts them to `SaddleError`.

### Configuration

`~/.saddle/config.toml` (auto-created with defaults if missing). Key sections: `app`, `llm` (model, api_base_url, api_key_env), `memory` (db_path, vec_dim), `plugins`, `logging`.

### Harness System

Features in `harness/features.json` track development progress with dependencies and tags. Status is either `pending` or `completed`.
