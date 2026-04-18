# Project Strategy

## 🎯 Strategic Intent
Our goal is to build `vtuber-brain` as a leader in the Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector ecosystem by focusing on:
**Per-turn persona text_prompt to PersonaPlex via ConversationDirective. Skills plug in as typed gRPC tool calls. Memory is tiered: short-term per-conversation, long-term in Postgres with pgvector, character lore from vtuber-commons.**

## 🗺️ Execution Pillars
1. **Rapid Prototyping:** Iterating quickly while maintaining core architectural integrity.
2. **Community Feedback:** Using user insights to drive the roadmap.
3. **Automation First:** Every repetitive task should be a script or a workflow.

## 📈 Success Metrics
- **Performance:** Achievement of benchmarks defined in `ARCHITECTURE.md`.
- **Stability:** Passing all tests in `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.
- **Adoption:** Clear documentation and easy onboarding per `README.md`.
