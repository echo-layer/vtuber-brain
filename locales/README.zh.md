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

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | [日本語](./README.ja.md) | 简体中文 ]

vtuber-brain 是 Director/Performer 分离 (ADR-001) 中的 Director。接收 context (chat、观众信号、内部状态),完成推理与记忆检索,决定采用哪个 persona 与调用哪个 skill,再通过 gRPC 向 vtuber-voice emit ConversationDirective (text_prompt + voice_prompt)。skill (game/sing/policy/strategy) 以 typed tool 注册,brain 通过 tool-use 协议 dispatch。长期记忆存于 Postgres + pgvector;character lore 启动时从 vtuber-commons 加载;Mojo 承担热推理 kernel (RAG re-rank、意图分类);Python 负责 LLM (Ollama/vLLM) 服务。

## ✨ 特性 (Features)
- 🚀 **Director 循环 —— 接收 context → 选择 persona → 通过 gRPC ConversationDirective 向 vtuber-voice emit text_prompt 与 voice_prompt**
- 🛡️ **Skill 路由 —— 以来自 vtuber-contracts 的 typed gRPC contract 向 game / sing / policy / strategy dispatch tool call**
- 📊 **分层记忆 —— 短期 (会话内进程内)、长期 (Postgres + pgvector)、character lore (启动时从 vtuber-commons 加载)**

## 🛠️ 快速开始 (Quick Start)
```bash
# 安装 Rust toolchain (rustup)、Python 3.12+、带 pgvector 扩展的 Postgres 16+,运行 cargo build 与 pip install -r python/requirements.txt,.env 中设置 OLLAMA_HOST 或 VLLM_URL,然后 cargo run 在 8081 端口启动 brain
```

## 🗺️ 导航 (Navigation)
- 🏗️ **[架构 (Architecture)](../ARCHITECTURE.md)**
- 📅 **[路线图 (Roadmap)](../ROADMAP.md)**
- 🤝 **[贡献 (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[项目结构 (Structure)](../STRUCTURE.tree)**

## ⚖️ 许可证 (License)
[MIT](../LICENSE)
