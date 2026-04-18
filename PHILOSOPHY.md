# Technical Philosophy

## 🧩 Architectural Mindset
The core of `vtuber-brain` is built on the belief that software should be:
- **Resilient:** Handling failures gracefully.
- **Scalable:** Growing with the data volume.
- **Maintainable:** Easy for new contributors to understand.

## 🛠️ Implementation Choices
We prioritize `Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector` for its unique strengths in Per-turn persona text_prompt to PersonaPlex via ConversationDirective. Skills plug in as typed gRPC tool calls. Memory is tiered: short-term per-conversation, long-term in Postgres with pgvector, character lore from vtuber-commons..
