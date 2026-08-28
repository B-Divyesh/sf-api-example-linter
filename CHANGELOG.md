# Changelog

## Unreleased

- Recognize normal equals-form and compact curl JSON data arguments without executing shell text.
- Avoid a redundant `NO_EXAMPLES` diagnostic when a fenced example is malformed.
- Ship Azure Static Web Apps response-policy configuration and an executable visible install command.

## 0.1.0 — 2026-08-27

- Extract JSON and curl request bodies from Markdown.
- Read request and response examples embedded in OpenAPI 3.x documents.
- Validate examples against named schemas or operations.
- Emit terminal, JSON, and GitHub Actions diagnostics.
- Run optional allowlisted mock-server checks without executing shell input.
