@description('Azure region for all resources.')
param location string

@description('Base name for resource naming.')
param environmentName string

@description('Tags to apply to all resources.')
param tags object = {}

@description('Principal ID of the Container App managed identity.')
param containerAppPrincipalId string

// AcrPull role
var acrPullRoleId = '7f951dda-4ed3-4680-a7ca-43fe172d538d'

var acrName = replace(replace(environmentName, '-', ''), '_', '')

resource acr 'Microsoft.ContainerRegistry/registries@2023-07-01' = {
  name: length(acrName) > 50 ? substring(acrName, 0, 50) : acrName
  location: location
  tags: tags
  sku: {
    name: 'Basic'
  }
  properties: {
    adminUserEnabled: false
  }
}

resource acrPullRole 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(acr.id, containerAppPrincipalId, acrPullRoleId)
  scope: acr
  properties: {
    principalId: containerAppPrincipalId
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', acrPullRoleId)
    principalType: 'ServicePrincipal'
  }
}

@description('Resource ID of the Container Registry.')
output acrId string = acr.id

@description('Name of the Container Registry.')
output acrName string = acr.name

@description('Login server for the Container Registry.')
output acrLoginServer string = acr.properties.loginServer
