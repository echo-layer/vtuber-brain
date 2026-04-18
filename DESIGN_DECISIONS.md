# Design Decisions (ADR)

## 💡 Philosophy
This project uses Architectural Decision Records (ADR) to track significant design choices.

## 📝 Decision Log

### ADR-001: Initial Scaffolding
- **Status:** Accepted
- **Context:** Bootstrapped using MLOps Meta-Repo.
- **Decision:** Use Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector for the core implementation to balance performance and safety.
- **Consequences:** Provides a solid foundation for vtuber-brain is the Director half of the Director/Performer split (ADR-001 in repo_plus.yml). It receives context (chat, audience signal, internal state), runs reasoning and memory retrieval, decides which persona to wear and which skill to invoke, then emits a ConversationDirective containing text_prompt and voice_prompt to vtuber-voice via gRPC. Skills (game/sing/policy/strategy) register as typed tools and brain dispatches via tool-use protocol. Long-term memory lives in Postgres with pgvector; character lore loads from vtuber-commons at startup. Mojo handles hot inference kernels (RAG re-rank, intent classification); Python serves the underlying LLM (Ollama or vLLM)..

---
*Add new decisions above this line using the standard ADR format.*
