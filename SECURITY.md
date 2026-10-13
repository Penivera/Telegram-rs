# Security Policy

The `telegram-rs` project takes the security of our library, dependencies, and users seriously. This document outlines our vulnerability handling and reporting policies.

## Supported Versions

We provide security updates and patches for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability

If you discover a potential security vulnerability in `telegram-rs`, please do **not** open a public issue or discuss it in public chat groups.

Instead, please report the vulnerability using one of the following methods:

1. **GitHub Security Advisories (Preferred)**:
   Navigate to the [Security Advisories](https://github.com/Penivera/Telegram-rs/security/advisories) tab of this repository and click **"New draft advisory"** to open a private advisory.

2. **Direct Maintainer Contact**:
   If private advisories are unavailable, contact the repository maintainers privately via their GitHub profile contact info.

### What to Include in Your Report

To help us triage and resolve the issue quickly, please include:

- A descriptive summary of the vulnerability.
- Steps to reproduce the issue, including minimal code samples or a Proof of Concept (PoC).
- Affected version(s) and feature flags (e.g. `mini-app`, `wallet`, `webhook`).
- Your assessment of the potential severity and real-world impact.
- Any suggested mitigations or patches if available.

## Response & Disclosure Process

1. **Acknowledgment**: We aim to acknowledge receipt of security reports within **48 hours**.
2. **Investigation & Confirmation**: We will investigate and confirm whether the reported vulnerability is valid and keep you updated on progress.
3. **Patch Development**: We will prepare and test a fix in a private branch or advisory fork.
4. **Coordinated Disclosure**: Once a fix is verified, a patched release will be published to [crates.io](https://crates.io/crates/telegram-rs-2) alongside a public security advisory giving credit to the reporter (unless you request to remain anonymous).

We kindly ask reporters to follow responsible disclosure guidelines and refrain from disclosing vulnerability details publicly until a fix has been released.
