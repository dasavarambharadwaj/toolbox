## Deferred from: code review of spec-1-2-unified-multi-call-binary-entry-point-mode-dispatcher.md (2026-10-04)

- source_spec: `_bmad-output/implementation-artifacts/spec-1-2-unified-multi-call-binary-entry-point-mode-dispatcher.md`
  summary: Duplicate root integration tests in tests/e2e_binary.rs and tests/e2e_boundaries.rs
  evidence: Root tests/ directory mirrors crates/tb/tests/ and is part of initial workspace scaffolding.
- source_spec: `_bmad-output/implementation-artifacts/spec-1-2-unified-multi-call-binary-entry-point-mode-dispatcher.md`
  summary: CLI subcommands not yet modeled in CliArgs for symlink help parsing
  evidence: Subcommands (image, pdf, archive, dev) are scheduled for their respective epics (Epic 2+).
