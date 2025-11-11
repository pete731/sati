# Contributing to SATI

Thank you for your interest in SATI! This is a community-driven standard.

## Current Status

SATI is currently in **scaffolding phase** (November 2025). Full implementation begins Q1 2026.

## How to Contribute

### 1. Specification Feedback

Review [SPECIFICATION.md](./docs/SPECIFICATION.md) and open issues for:
- Technical concerns
- Security considerations
- Design improvements
- Use case gaps

### 2. Integration Proposals

Building an agent framework or protocol? We'd love to hear about:
- Integration requirements
- Missing features
- Compatibility concerns

### 3. Implementation Contributions

**Phase 1 (Q1 2026):** Core infrastructure
- Identity Registry ZK Compression integration
- Reputation Registry implementation
- Mandate Registry with full AP2 support
- SAS schema implementations

**Phase 2 (Q2 2026+):** Advanced features
- Multi-validator consensus
- Cross-chain bridges
- Additional validation methods

## Development Setup

```bash
# Prerequisites
- Rust 1.89.0 (locked via rust-toolchain.toml)
- Solana CLI 1.18+
- Anchor 0.32.1+
- Node.js 18+
- pnpm (not npm/yarn)

# Setup
git clone https://github.com/tenequm/sati.git
cd sati
pnpm install
anchor build
cd sdk && pnpm build
anchor test
```

## Code Style

- Follow Anchor best practices
- Use rustfmt for Rust code
- Use prettier for TypeScript
- Add tests for new features
- **Use @solana/kit (NOT @solana/web3.js)**
- **Always provide type conversion helpers (toAddress/toPublicKey)**

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### PR Guidelines

- **Clear description:** Explain what and why
- **Reference issues:** Link to relevant issues
- **Tests:** Add tests for new functionality
- **Documentation:** Update docs for API changes
- **Small commits:** Keep commits focused and atomic

## Coding Standards

### Rust (Anchor Programs)

```rust
// ✅ Good: Clear naming, proper error handling
pub fn register_agent(
    ctx: Context<RegisterAgent>,
    agent_id: String,
) -> Result<()> {
    let agent = &mut ctx.accounts.agent;

    // Validate input
    require!(agent_id.len() > 0, ErrorCode::InvalidAgentId);

    // Initialize account
    agent.agent_id_hash = hash(&agent_id);
    agent.owner = ctx.accounts.owner.key();

    Ok(())
}

// ❌ Bad: No validation, unclear naming
pub fn reg(ctx: Context<Reg>, id: String) -> Result<()> {
    ctx.accounts.a.h = hash(&id);
    Ok(())
}
```

### TypeScript (SDK)

```typescript
// ✅ Good: Type conversions, clear documentation
/**
 * Register a new agent identity
 *
 * @param agentId - Unique agent identifier string
 * @returns Agent PDA public key
 */
async registerAgent(agentId: string): Promise<anchor.web3.PublicKey> {
    const [agentPda] = this.deriveAgentPDA(agentId);

    await this.program.methods
        .registerAgent(agentId)
        .accounts({ /* ... */ })
        .rpc();

    return agentPda;
}

// ❌ Bad: No docs, missing type conversions
async reg(id) {
    return await this.p.methods.reg(id).rpc();
}
```

## Testing

```bash
# Run all tests
anchor test

# Run specific test
anchor test --skip-deploy -- --nocapture identity_registry::test_register_agent
```

## Documentation

- **Code comments:** Explain _why_, not _what_
- **Function docs:** Document all public APIs
- **Examples:** Add usage examples for new features
- **Specification:** Update spec for protocol changes

## Community

- **Questions:** Use [GitHub Discussions](https://github.com/tenequm/sati/discussions)
- **Bugs:** Open an issue with reproduction steps
- **Features:** Discuss in issue before implementing
- **Twitter:** [@opwizardx](https://twitter.com/opwizardx)

## License

By contributing, you agree that your contributions will be licensed under CC0 1.0 Universal (Public Domain).

## Questions?

- GitHub Discussions: [sati/discussions](https://github.com/tenequm/sati/discussions)
- Twitter: [@opwizardx](https://twitter.com/opwizardx)
