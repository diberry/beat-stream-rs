# ADR-0000: Use Bicep for Infrastructure-as-Code

## Status

Accepted

## Context

beat-stream-rs is deployed exclusively on Azure. The team is a single developer
who needs a lightweight, low-ceremony IaC tool. There are no multi-cloud
requirements now or on the roadmap. The key decision is between Bicep and
Terraform for managing Azure resources.

Bicep is Azure-native and compiles directly to ARM templates. It requires no
state file, no backend storage account, and ships with the Azure CLI. Terraform
is cloud-agnostic but adds operational overhead: a state file that must be stored
remotely, a lock file, and a separate binary to install and update.

## Decision

Use **Azure Bicep** as the sole infrastructure-as-code tool.

## Consequences

### Positive

- **No state file management** — Bicep is declarative over ARM; Azure itself is
  the source of truth. No need to provision a storage account for remote state.
- **Direct ARM integration** — Bicep compiles 1:1 to ARM templates. New Azure
  resource types and API versions are available on day one.
- **Minimal tooling** — Bicep CLI ships with Azure CLI. No additional binary to
  install, no version-lock file to maintain.
- **First-class VS Code support** — The Bicep extension provides intellisense,
  validation, and visualization out of the box.
- **Azure Developer CLI (azd) integration** — `azd up` natively understands
  Bicep templates for streamlined dev-loop deployments.

### Negative

- **Azure-only** — If the project ever needs multi-cloud deployment, Bicep
  cannot target AWS or GCP resources.
- **Smaller community** — Terraform has a larger ecosystem of modules and
  community examples.
- **No built-in drift detection** — Unlike Terraform's `plan`, Bicep relies on
  ARM's what-if operation which is less mature.

### Neutral

- CI/CD pipelines use `az deployment group create` or `azd up` — both are
  well-documented for GitHub Actions with OIDC authentication.
