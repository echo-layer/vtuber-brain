# Project Vision

## 🌟 Mission Statement
**The central reasoning, memory, and tool-use core that every vtuber persona depends on for character consistency across hours of live streaming, with a plug-and-play skill ecosystem.**

## 🎯 Primary Objectives
- **Objective 1:** Central reasoning, memory, and tool use core for character consistency across hours of streaming
- **Objective 2:** Plug-and-play skill ecosystem — adding a new capability equals registering a typed tool with the brain

## 🔭 Long-term Impact
`vtuber-brain` aims to solve the following problem:
vtuber-brain is the Director half of the Director/Performer split (ADR-001 in repo_plus.yml). It receives context (chat, audience signal, internal state), runs reasoning and memory retrieval, decides which persona to wear and which skill to invoke, then emits a ConversationDirective containing text_prompt and voice_prompt to vtuber-voice via gRPC. Skills (game/sing/policy/strategy) register as typed tools and brain dispatches via tool-use protocol. Long-term memory lives in Postgres with pgvector; character lore loads from vtuber-commons at startup. Mojo handles hot inference kernels (RAG re-rank, intent classification); Python serves the underlying LLM (Ollama or vLLM).

By leveraging `Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector`, we ensure that our solution is not only functional but also future-proof and high-performing.
