@description('Name of the User-Assigned Managed Identity.')
param name string

@description('Azure region.')
param location string

@description('Tags to apply.')
param tags object = {}

resource identity 'Microsoft.ManagedIdentity/userAssignedIdentities@2023-01-31' = {
  name: name
  location: location
  tags: tags
}

@description('Resource ID of the managed identity.')
output identityId string = identity.id

@description('Principal ID of the managed identity.')
output principalId string = identity.properties.principalId

@description('Client ID of the managed identity.')
output clientId string = identity.properties.clientId
