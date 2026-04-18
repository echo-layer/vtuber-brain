<div align="center">

# vtuber-brain

**Director for the vtuber-* program — long-term memory, skill router, tool use, and per-turn persona prompt generation. Receives context, chooses persona, emits text_prompt and voice_prompt to vtuber-voice (PersonaPlex Performer) via ConversationDirective.**

[![CI](https://github.com/echo-layer/vtuber-brain/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-brain/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-brain/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-brain/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

<!-- Language Badges: Synchronize with English README -->
[AI: Generate the same individual language badges here]

<!-- LOD Badges: Synchronize with English README -->
[AI: Generate the same LOD badges here]

</div>

---

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | 日本語 | [简体中文](./README.zh.md) ]

> [AI: TRANSLATE the professional tagline/description into JAPANESE here]
vtuber-brain is the Director half of the Director/Performer split (ADR-001 in repo_plus.yml). It receives context (chat, audience signal, internal state), runs reasoning and memory retrieval, decides which persona to wear and which skill to invoke, then emits a ConversationDirective containing text_prompt and voice_prompt to vtuber-voice via gRPC. Skills (game/sing/policy/strategy) register as typed tools and brain dispatches via tool-use protocol. Long-term memory lives in Postgres with pgvector; character lore loads from vtuber-commons at startup. Mojo handles hot inference kernels (RAG re-rank, intent classification); Python serves the underlying LLM (Ollama or vLLM).

## ✨ 特徴 (Features)
> [AI: TRANSLATE all 3 Features into JAPANESE here]

## 🛠️ クイックスタート (Quick Start)
> [AI: TRANSLATE getting_started_instructions into JAPANESE here]

## 🗺️ ナวิゲーション (Navigation)
- 🏗️ **[アーキテクチャ (Architecture)](../ARCHITECTURE.md)**
- 📅 **[ロードマップ (Roadmap)](../ROADMAP.md)**
- 🤝 **[貢献する (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[プロジェクト構造 (Structure)](../STRUCTURE.tree)**

## ⚖️ ライセンス (License)
[MIT](../LICENSE)
