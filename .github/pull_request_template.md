## Summary

<!-- 1-3 bullet points describing what this PR does and why -->

## Type

- [ ] Bug fix
- [ ] Feature
- [ ] Chore (refactor, deps, CI, docs)
- [ ] Breaking change

## Component

- [ ] Commitment schemes (`src/`)
- [ ] Bare-metal / `no_std` support
- [ ] Upstream sync (`arkworks-rs/poly-commit`)
- [ ] Docs (`README.md`)
- [ ] CI / infra

## Related issues

<!-- Link issues: Closes #123, Relates to #456 -->

## Validation

<!-- Commands run and their results (host tests, clippy, no_std build for the MCU targets where relevant) -->

## Checklist

- [ ] Title follows format: `type(scope): description` (e.g. `fix(no-std): gate float maths on libm`)
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] Tests pass: `cargo test`
- [ ] `no_std` build succeeds, if bare-metal support changed
- [ ] New tests added for changed behaviour
- [ ] No secrets, planning notes, or generated build artifacts committed
- [ ] Breaking changes documented (if any)
