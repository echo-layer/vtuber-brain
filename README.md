<div align="center">

# vtuber-brain

**Director for the vtuber-* program — long-term memory, skill router, tool use, and per-turn persona prompt generation. Receives context, chooses persona, emits text_prompt and voice_prompt to vtuber-voice (PersonaPlex Performer) via ConversationDirective.**

[![CI](https://github.com/echo-layer/vtuber-brain/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-brain/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-brain/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-brain/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

![Rust LOD](https://img.shields.io/badge/Rust_LOD-0-dea584.svg) ![Mojo LOD](https://img.shields.io/badge/Mojo_LOD-0-CC0000.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-0-3776AB.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-0-3776AB.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-0-brightgreen.svg)

[![Rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=white)](./) [![Mojo](https://img.shields.io/badge/Mojo-CC0000?logo=mojo&logoColor=white)](./) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](./) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](./) [![Axum](https://img.shields.io/badge/Axum-dea584?logo=rust&logoColor=white)](./)

</div>

---

[ English | [ภาษาไทย](./locales/README.th.md) | [日本語](./locales/README.ja.md) | [简体中文](./locales/README.zh.md) ]

vtuber-brain is the Director half of the Director/Performer split (ADR-001 in repo_plus.yml). It receives context (chat, audience signal, internal state), runs reasoning and memory retrieval, decides which persona to wear and which skill to invoke, then emits a ConversationDirective containing text_prompt and voice_prompt to vtuber-voice via gRPC. Skills (game/sing/policy/strategy) register as typed tools and brain dispatches via tool-use protocol. Long-term memory lives in Postgres with pgvector; character lore loads from vtuber-commons at startup. Mojo handles hot inference kernels (RAG re-rank, intent classification); Python serves the underlying LLM (Ollama or vLLM).

## ✨ Features

- 🚀 **Feature 1** — Director loop: receive context → choose persona → emit text_prompt and voice_prompt to vtuber-voice via gRPC ConversationDirective
- 🛡️ **Feature 2** — Skill router: dispatch tool calls to game/sing/policy/strategy services via typed gRPC contracts from vtuber-contracts
- 📊 **Feature 3** — Tiered memory: short-term (per-conversation in-process), long-term (Postgres with pgvector), character lore (loaded from vtuber-commons at startup)

## 🛠️ Quick Start

```bash
# Install Rust toolchain (rustup), Python 3.12+, and Postgres 16+ with the pgvector extension, then run cargo build and pip install -r python/requirements.txt. Set OLLAMA_HOST or VLLM_URL in .env to point at your LLM server, then cargo run to start the brain on port 8081.
```

## 🗺️ Navigation

- 🏗️ **[Architecture](ARCHITECTURE.md)** — Core design and components.
- 📅 **[Roadmap](ROADMAP.md)** — Project timeline and milestones.
- 🤝 **[Contributing](CONTRIBUTING.md)** — How to join and help.
- 🌳 **[Project Structure](STRUCTURE.tree)** — Full file map.

## ⚖️ License

[MIT](LICENSE)
