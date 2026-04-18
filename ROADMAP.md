# Project Roadmap

## 📅 Timeline
Q2 2026 v0.1 (director loop + Ollama bridge), Q3 2026 v0.2 (skill router + memory tier), Q4 2026 v0.5 (Mojo kernels + RAG re-rank), Q1 2027 v1.0 (semver-locked stable)

## 🏁 Milestones
v0.1 director loop + Ollama bridge, v0.2 skill router + memory, v0.5 Mojo kernels + RAG re-rank, v1.0 stable semver-locked

## 🚀 Future Vision
The central reasoning, memory, and tool-use core that every vtuber persona depends on for character consistency across hours of live streaming, with a plug-and-play skill ecosystem.

### Phase 1: Foundation
- [ ] Implement core Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector engine.
- [ ] Set up basic CI/CD in `.github/workflows/ci.yml`.

### Phase 2: Scale
- [ ] Optimize Tool use over hardcoded skills (skill = registered tool with typed contract from vtuber-contracts) implementations.
- [ ] Expand connector support.

### Phase 3: Excellence
- [ ] Full security audit per [SECURITY.md](SECURITY.md).
- [ ] Finalize production release.
