# Change Log

All notable changes to the "telemetry-kit" extension will be documented in this file.

## [0.1.0] - 2025-01-23

### Added
- Initial release of telemetry-kit VS Code extension
- Smart project analysis for telemetry opportunities
- Inline code suggestions and quick fixes
- 17+ code snippets for common telemetry patterns
- Integration with telemetry-kit CLI analyze command
- Configurable auto-analysis on file save
- Priority-based suggestion system (High/Medium/Low)
- Context-aware telemetry insertion
- Support for Rust projects with Cargo.toml
- Privacy-first snippets (DO_NOT_TRACK, GDPR compliance)
- Web framework integration snippets (Axum)
- CLI application tracking patterns
- Library crate telemetry patterns
- Comprehensive documentation and examples

### Features
- **Analysis Command**: Scan entire project for instrumentation opportunities
- **Quick Fixes**: One-click code actions to add telemetry
- **Snippets**: Complete library of telemetry patterns
  - Initialization (basic, with consent, with sync)
  - Event tracking (commands, features, errors)
  - Performance monitoring (timing, duration)
  - Privacy controls (DO_NOT_TRACK, strict mode)
  - Framework integration (Axum, CLI patterns)
- **Configuration**: Customizable severity levels and CLI path
- **Context Menu**: Right-click to add telemetry at cursor

### Technical
- TypeScript-based extension
- Integration with telemetry-kit CLI via JSON output
- VS Code Diagnostics API for inline suggestions
- Code Action Provider for quick fixes
- Snippet provider for code completion
- Configuration through VS Code settings

## [Unreleased]

### Planned Features
- Inline hints with CodeLens
- Hover information for telemetry patterns
- Auto-fix all suggestions command
- Telemetry dashboard view
- Integration with Language Server Protocol
- Support for more frameworks (Actix, Rocket)
- Template generation for new projects
- Batch apply suggestions
- Custom snippet creation
