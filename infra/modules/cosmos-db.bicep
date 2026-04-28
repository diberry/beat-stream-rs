@description('Azure region for all resources.')
param location string

@description('Base name for resource naming.')
param environmentName string

@description('Tags to apply to all resources.')
param tags object = {}

@description('Principal ID of the Container App managed identity.')
param containerAppPrincipalId string

// Cosmos DB Built-in Data Contributor role
var cosmosDataContributorRoleId = '00000000-0000-0000-0000-000000000002'

var cosmosAccountName = '${environmentName}-cosmos'

resource cosmosAccount 'Microsoft.DocumentDB/databaseAccounts@2024-02-15-preview' = {
  name: cosmosAccountName
  location: location
  tags: tags
  kind: 'GlobalDocumentDB'
  properties: {
    databaseAccountOfferType: 'Standard'
    locations: [
      {
        locationName: location
        failoverPriority: 0
      }
    ]
    capabilities: [
      {
        name: 'EnableServerless'
      }
    ]
    consistencyPolicy: {
      defaultConsistencyLevel: 'Session'
    }
  }
}

resource database 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases@2024-02-15-preview' = {
  parent: cosmosAccount
  name: 'beatstream'
  properties: {
    resource: {
      id: 'beatstream'
    }
  }
}

resource roomsContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-02-15-preview' = {
  parent: database
  name: 'rooms'
  properties: {
    resource: {
      id: 'rooms'
      partitionKey: {
        paths: ['/id']
        kind: 'Hash'
      }
      defaultTtl: 86400
    }
  }
}

resource patternsContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-02-15-preview' = {
  parent: database
  name: 'patterns'
  properties: {
    resource: {
      id: 'patterns'
      partitionKey: {
        paths: ['/room_id']
        kind: 'Hash'
      }
    }
  }
}

resource cosmosDataContributorAssignment 'Microsoft.DocumentDB/databaseAccounts/sqlRoleAssignments@2024-02-15-preview' = {
  parent: cosmosAccount
  name: guid(cosmosAccount.id, containerAppPrincipalId, cosmosDataContributorRoleId)
  properties: {
    principalId: containerAppPrincipalId
    roleDefinitionId: '${cosmosAccount.id}/sqlRoleDefinitions/${cosmosDataContributorRoleId}'
    scope: cosmosAccount.id
  }
}

@description('Resource ID of the Cosmos DB account.')
output cosmosAccountId string = cosmosAccount.id

@description('Name of the Cosmos DB account.')
output cosmosAccountName string = cosmosAccount.name

@description('Endpoint of the Cosmos DB account.')
output cosmosEndpoint string = cosmosAccount.properties.documentEndpoint
