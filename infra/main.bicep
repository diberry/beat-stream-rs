targetScope = 'resourceGroup'

@description('Azure region for all resources.')
param location string

@description('Name of the environment (used for resource naming).')
param environmentName string

@description('Tags to apply to all resources.')
param tags object = {}

@description('Contact email for budget alerts.')
param contactEmail string

// --- User-Assigned Managed Identity (deploys FIRST — breaks circular dependency) ---
module identity 'modules/identity.bicep' = {
  name: 'identity'
  params: {
    name: '${environmentName}-id'
    location: location
    tags: tags
  }
}

// --- Monitoring ---
module monitoring 'modules/monitoring.bicep' = {
  name: 'monitoring'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
  }
}

// --- ACR ---
module acr 'modules/acr.bicep' = {
  name: 'acr'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    principalId: identity.outputs.principalId
  }
}

// --- Cosmos DB ---
module cosmos 'modules/cosmos-db.bicep' = {
  name: 'cosmos'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    principalId: identity.outputs.principalId
  }
}

// --- Key Vault ---
module keyVault 'modules/key-vault.bicep' = {
  name: 'keyVault'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    principalId: identity.outputs.principalId
  }
}

// --- Container Apps ---
module containerApps 'modules/container-apps.bicep' = {
  name: 'containerApps'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    logAnalyticsId: monitoring.outputs.logAnalyticsId
    acrLoginServer: acr.outputs.acrLoginServer
    keyVaultUri: keyVault.outputs.keyVaultUri
    cosmosEndpoint: cosmos.outputs.cosmosEndpoint
    identityId: identity.outputs.identityId
    identityClientId: identity.outputs.clientId
  }
}

// --- Budget Alert ($20/month) ---
module budget 'modules/budget.bicep' = {
  name: 'budget'
  params: {
    contactEmail: contactEmail
  }
}

// --- Outputs ---
@description('FQDN of the deployed Container App.')
output containerAppFqdn string = containerApps.outputs.containerAppFqdn

@description('Cosmos DB endpoint.')
output cosmosEndpoint string = cosmos.outputs.cosmosEndpoint

@description('Key Vault URI.')
output keyVaultUri string = keyVault.outputs.keyVaultUri

@description('ACR login server.')
output acrLoginServer string = acr.outputs.acrLoginServer
