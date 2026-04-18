# Security Policy

## 🛡️ Commitment
We take the security of `vtuber-brain` seriously. This project aims for a **Medium** security standard.

## 📢 Reporting a Vulnerability
Please do not report security vulnerabilities through public GitHub issues. Instead, send a detailed report to:
**mr.bt1590@gmail.com**

### What to include:
- A description of the vulnerability.
- Steps to reproduce (PoC).
- Potential impact.

## 🔐 Security Protocols
This service handles user conversation context and LLM prompts — not the public edge (vtuber-api owns auth) but does receive sensitive context. Conversation history MUST encrypt at rest in Postgres. Prompt-injection defense routes through vtuber-policy before any tool call dispatch — never let raw user text reach a tool invocation without policy classification first.

- **Dependency Management:** Regularly scan for vulnerable packages.
- **CI/CD Security:** Mandatory automated security scans are integrated into `.github/workflows/security.yml`.
- **Disclosure:** We follow a responsible disclosure timeline.
