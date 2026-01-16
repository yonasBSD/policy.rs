# 🎨 policy.rs

> A small, auditable, terminating, deterministic micro-policy engine.

![Licenses](https://github.com/yonasBSD/policy.rs/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/policy.rs/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/policy.rs/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/policy.rs/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/policy.rs/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/policy.rs/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/policy.rs/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_policy.rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_policy.rs)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_policy.rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_policy.rs)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_policy.rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_policy.rs)
<!--[![codecov](https://codecov.io/gh/yonasBSD/policy.rs/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/policy.rs)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/policy.rs/pkgs/container/policy.rs)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/policy.rs/example.svg)](https://hub.docker.com/r/policy.rs/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/policy.rs/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/policy.rs)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/policy.rs/status.svg)](https://deps.rs/repo/github/yonasBSD/policy.rs)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/policy.rs.svg)](https://github.com/yonasBSD/policy.rs/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/policy.rs.svg)](https://github.com/yonasBSD/policy.rs/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

## Security Model

policy.rs (based on [Gate0](https://github.com/Qarait/gate0)) is designed for high-assurance environments where policy evaluation must be deterministic and resource-bounded. See [SECURITY.md](SECURITY.md) for the full threat model, system invariants, and mechanical guarantees.

## Architecture

policy.rs uses a linear, **Deny-Overrides** evaluation strategy. Each rule consists of a **Target** (fast-path match) and an optional **Condition** (deep logic).

```mermaid
graph TD
    REQ([Request]) ==> POL
    
    subgraph POL [Policy: Ordered Rules]
        direction TB
        R1[Rule 1: Deny]
        R2[Rule 2: Allow]
        R3[Rule 3: Allow]
    end

    POL ==> MATCH{Match?}
    
    subgraph EVAL [Evaluation Logic]
        direction LR
        MATCH -- "Target + Condition" --> DECIDE
        DECIDE{Effect?}
        DECIDE -- "Deny" --> D_WIN[Deny Wins]
        DECIDE -- "Allow" --> A_PEND[Allow Pending]
    end

    D_WIN ==> FIN([Final Decision])
    A_PEND -- "Next Rule" --> MATCH
    MATCH -- "No more rules" --> NO_M
    NO_M[No Match] ==> D_DEF[Default Deny]
    D_DEF ==> FIN
```

## Verification

The correctness and safety of policy.rs are mechanically verified:

- **Unit Tests**: Full coverage of core logic and edge cases.
- **Property-Based Testing**: Hundreds of adversarial scenarios generated via `proptest`.
- **Undefined Behavior Check**: Verified panic-free and UB-free using `cargo miri`.
- **Bounded Evaluation**: Worst-case inputs are tested to ensure termination.

```bash
cargo test
cargo +nightly miri test --lib
```

## Example

```rust
use policy_rs::policy::{Policy, ReasonCode, Request, Rule, Target};

let policy = Policy::builder()
    .rule(Rule::allow(Target::any(), ReasonCode(1)))
    .build()?;

let decision = policy.evaluate(&Request::new("alice", "read", "doc"))?;
assert!(decision.is_allow());
```

## Examples

The `examples/` directory contains illustrative scenarios demonstrating common policy.rs usage patterns:

- **SaaS API**: Standard RBAC/Multi-tenancy logic.
- **Zero Trust Network**: Attribute-Based Access Control (ABAC) with MFA and location checks.
- **Complex Overrides**: Demonstrating Deny-Overrides conflict resolution.

Run them with:
```bash
cargo run --example simple
cargo run --example simple-with-logger
```

## License

MIT
