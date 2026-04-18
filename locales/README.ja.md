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

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | 日本語 | [简体中文](./README.zh.md) ]

vtuber-brain は Director/Performer 分離 (ADR-001) の Director 側。context (chat、audience シグナル、内部状態) を受けて reasoning と memory 検索を行い、どの persona と skill を呼ぶか決定し、ConversationDirective (text_prompt + voice_prompt) を gRPC 経由で vtuber-voice に emit する。skill (game/sing/policy/strategy) は typed tool として登録され、brain が tool-use プロトコルで dispatch する。長期メモリは Postgres + pgvector、character lore は vtuber-commons から startup 時ロード。Mojo がホット推論カーネル (RAG re-rank、intent 分類) を担当し、Python が LLM (Ollama/vLLM) をサーブする。

## ✨ 特徴 (Features)
- 🚀 **Director ループ — context 受信 → persona 選択 → ConversationDirective で text_prompt + voice_prompt を vtuber-voice に emit**
- 🛡️ **Skill ルーター — typed gRPC contract で game / sing / policy / strategy に tool call を dispatch**
- 📊 **階層化メモリ — short-term (会話内 in-process)、long-term (Postgres + pgvector)、character lore (startup 時 vtuber-commons からロード)**

## 🛠️ クイックスタート (Quick Start)
```bash
# Rust toolchain (rustup)、Python 3.12+、pgvector 拡張付き Postgres 16+ をインストールし、cargo build と pip install -r python/requirements.txt を実行。.env に OLLAMA_HOST または VLLM_URL を設定後、cargo run で brain をポート 8081 で起動
```

## 🗺️ ナวิゲーション (Navigation)
- 🏗️ **[アーキテクチャ (Architecture)](../ARCHITECTURE.md)**
- 📅 **[ロードマップ (Roadmap)](../ROADMAP.md)**
- 🤝 **[貢献する (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[プロジェクト構造 (Structure)](../STRUCTURE.tree)**

## ⚖️ ライセンス (License)
[MIT](../LICENSE)
