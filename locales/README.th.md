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

[ [English](../README.md) | ภาษาไทย | [日本語](./README.ja.md) | [简体中文](./locales/README.zh.md) ]

vtuber-brain เป็นฝั่ง Director ของการแยก Director/Performer (ADR-001 ใน repo_plus.yml) รับ context (chat, audience signal, internal state) ทำ reasoning + memory retrieval เลือก persona + skill ที่จะเรียก แล้ว emit ConversationDirective (text_prompt + voice_prompt) ไป vtuber-voice ผ่าน gRPC — skill (game/sing/policy/strategy) ลงทะเบียนเป็น typed tool และ brain dispatch ผ่าน tool-use protocol memory อยู่บน Postgres + pgvector; character lore โหลดจาก vtuber-commons; Mojo ดูแล hot inference kernel (RAG re-rank, intent classification); Python serve LLM (Ollama/vLLM)

## ✨ ฟีเจอร์เด่น (Features)
- 🚀 **Director loop — รับ context → เลือก persona → emit text_prompt + voice_prompt ไปยัง vtuber-voice ผ่าน gRPC ConversationDirective**
- 🛡️ **Skill router — dispatch tool call ไปยัง game / sing / policy / strategy ผ่าน typed gRPC contract จาก vtuber-contracts**
- 📊 **Memory หลายชั้น — short-term (in-process ต่อบทสนทนา), long-term (Postgres + pgvector), character lore (โหลดจาก vtuber-commons ตอน startup)**

## 🛠️ เริ่มต้นใช้งาน (Quick Start)
```bash
# ติดตั้ง Rust toolchain (rustup), Python 3.12+, Postgres 16+ พร้อม pgvector extension จากนั้นรัน cargo build และ pip install -r python/requirements.txt ตั้ง OLLAMA_HOST หรือ VLLM_URL ใน .env แล้วรัน cargo run เปิด brain ที่ port 8081
```

## 🗺️ การนำทาง (Navigation)
- 🏗️ **[สถาปัตยกรรม (Architecture)](../ARCHITECTURE.md)**
- 📅 **[แผนงาน (Roadmap)](../ROADMAP.md)**
- 🤝 **[การร่วมพัฒนา (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[โครงสร้างโปรเจกต์ (Structure)](../STRUCTURE.tree)**

## ⚖️ ลิขสิทธิ์ (License)
[MIT](../LICENSE)
