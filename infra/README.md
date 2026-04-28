# beat-stream-rs Infrastructure

Bicep-based infrastructure-as-code for the beat-stream-rs project on Azure.

## Architecture

| Resource | SKU / Tier | Purpose |
|---|---|---|
| Container Apps Environment | Consumption | Hosts the Rust backend |
| Container App (`beatstream`) | 0.5 vCPU / 1 GiB | WebSocket + REST API |
| Cosmos DB | Serverless | Room & pattern storage |
| Key Vault | Standard (RBAC) | Secret management |
| Container Registry | Basic | Docker image storage |
| Log Analytics | PerGB2018 | Container log streaming |

## Prerequisites

- [Azure CLI](https://learn.microsoft.com/cli/azure/install-azure-cli) v2.60+
- [Bicep CLI](https://learn.microsoft.com/azure/azure-resource-manager/bicep/install) v0.25+ (bundled with Azure CLI)
- An Azure subscription with Contributor access
- A resource group created in advance

## Deploy

```bash
# Login
az login

# Create a resource group (if needed)
az group create --name beatstream-dev-rg --location eastus2

# Deploy infrastructure
az deployment group create \
  --resource-group beatstream-dev-rg \
  --template-file infra/main.bicep \
  --parameters infra/main.bicepparam
```

## OIDC Setup for GitHub Actions

To deploy from GitHub Actions using OpenID Connect (OIDC):

1. Create an Entra ID app registration:
   ```bash
   az ad app create --display-name "beat-stream-rs-github"
   ```

2. Add a federated credential for the repo:
   ```bash
   az ad app federated-credential create \
     --id <APP_OBJECT_ID> \
     --parameters '{
       "name": "github-main",
       "issuer": "https://token.actions.githubusercontent.com",
       "subject": "repo:diberry/beat-stream-rs:ref:refs/heads/main",
       "audiences": ["api://AzureADTokenExchange"]
     }'
   ```

3. Create a service principal and grant Contributor on the resource group:
   ```bash
   az ad sp create --id <APP_ID>
   az role assignment create \
     --assignee <APP_ID> \
     --role Contributor \
     --scope /subscriptions/<SUB_ID>/resourceGroups/beatstream-dev-rg
   ```

4. Set the following GitHub Actions secrets:
   - `AZURE_CLIENT_ID` — Application (client) ID
   - `AZURE_TENANT_ID` — Directory (tenant) ID
   - `AZURE_SUBSCRIPTION_ID` — Subscription ID

## Cost Estimate (MVP — Dev/Test)

| Resource | Estimated Monthly Cost |
|---|---|
| Container Apps (Consumption) | ~$0 (free grant covers light usage) |
| Cosmos DB (Serverless) | ~$0–5 (pay-per-request) |
| Key Vault | ~$0.03/10K operations |
| Container Registry (Basic) | ~$5/month |
| Log Analytics | ~$2.76/GB ingested |
| **Total (light dev use)** | **~$5–10/month** |
