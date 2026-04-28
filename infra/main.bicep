targetScope = 'resourceGroup'

@description('Azure region for all resources.')
param location string

@description('Name of the environment (used for resource naming).')
param environmentName string

@description('Tags to apply to all resources.')
param tags object = {}

// --- Monitoring ---
module monitoring 'modules/monitoring.bicep' = {
  name: 'monitoring'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
  }
}

// --- ACR (deployed early so login server is available) ---
module acr 'modules/acr.bicep' = {
  name: 'acr'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    containerAppPrincipalId: containerApps.outputs.containerAppPrincipalId
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
    acrLoginServer: '${replace(replace(environmentName, '-', ''), '_', '')}.azurecr.io'
    keyVaultUri: keyVault.outputs.keyVaultUri
    cosmosEndpoint: cosmos.outputs.cosmosEndpoint
  }
}

// --- Cosmos DB ---
module cosmos 'modules/cosmos-db.bicep' = {
  name: 'cosmos'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    containerAppPrincipalId: containerApps.outputs.containerAppPrincipalId
  }
}

// --- Key Vault ---
module keyVault 'modules/key-vault.bicep' = {
  name: 'keyVault'
  params: {
    location: location
    environmentName: environmentName
    tags: tags
    containerAppPrincipalId: containerApps.outputs.containerAppPrincipalId
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
