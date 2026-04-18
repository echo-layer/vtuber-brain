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

[ [English](../README.md) | ภาษาไทย | [日本語](./README.ja.md) | [简体中文](./locales/README.zh.md) ]

> [AI: TRANSLATE the professional tagline/description into THAI here]
vtuber-brain is the Director half of the Director/Performer split (ADR-001 in repo_plus.yml). It receives context (chat, audience signal, internal state), runs reasoning and memory retrieval, decides which persona to wear and which skill to invoke, then emits a ConversationDirective containing text_prompt and voice_prompt to vtuber-voice via gRPC. Skills (game/sing/policy/strategy) register as typed tools and brain dispatches via tool-use protocol. Long-term memory lives in Postgres with pgvector; character lore loads from vtuber-commons at startup. Mojo handles hot inference kernels (RAG re-rank, intent classification); Python serves the underlying LLM (Ollama or vLLM).

## ✨ ฟีเจอร์เด่น (Features)
> [AI: TRANSLATE all 3 Features into THAI here]

## 🛠️ เริ่มต้นใช้งาน (Quick Start)
> [AI: TRANSLATE getting_started_instructions into THAI here]

## 🗺️ การนำทาง (Navigation)
- 🏗️ **[สถาปัตยกรรม (Architecture)](../ARCHITECTURE.md)**
- 📅 **[แผนงาน (Roadmap)](../ROADMAP.md)**
- 🤝 **[การร่วมพัฒนา (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[โครงสร้างโปรเจกต์ (Structure)](../STRUCTURE.tree)**

## ⚖️ ลิขสิทธิ์ (License)
[MIT](../LICENSE)
