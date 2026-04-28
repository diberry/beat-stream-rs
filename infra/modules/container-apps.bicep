@description('Azure region for all resources.')
param location string

@description('Base name for resource naming.')
param environmentName string

@description('Tags to apply to all resources.')
param tags object = {}

@description('Resource ID of the Log Analytics workspace.')
param logAnalyticsId string

@description('Login server of the Container Registry.')
param acrLoginServer string

@description('URI of the Key Vault.')
param keyVaultUri string

@description('Endpoint of the Cosmos DB account.')
param cosmosEndpoint string

var containerAppsEnvName = '${environmentName}-env'
var containerAppName = 'beatstream'

resource containerAppsEnv 'Microsoft.App/managedEnvironments@2024-03-01' = {
  name: containerAppsEnvName
  location: location
  tags: tags
  properties: {
    appLogsConfiguration: {
      destination: 'log-analytics'
      logAnalyticsConfiguration: {
        customerId: reference(logAnalyticsId, '2023-09-01').customerId
        sharedKey: listKeys(logAnalyticsId, '2023-09-01').primarySharedKey
      }
    }
  }
}

resource containerApp 'Microsoft.App/containerApps@2024-03-01' = {
  name: containerAppName
  location: location
  tags: tags
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    managedEnvironmentId: containerAppsEnv.id
    configuration: {
      ingress: {
        external: true
        targetPort: 8080
        transport: 'auto'
        stickySessions: {
          affinity: 'sticky'
        }
      }
      registries: [
        {
          server: acrLoginServer
          identity: 'system'
        }
      ]
    }
    template: {
      containers: [
        {
          name: containerAppName
          image: '${acrLoginServer}/${containerAppName}:latest'
          resources: {
            cpu: json('0.5')
            memory: '1Gi'
          }
          env: [
            {
              name: 'COSMOS_ENDPOINT'
              value: cosmosEndpoint
            }
            {
              name: 'KEY_VAULT_URI'
              value: keyVaultUri
            }
            {
              name: 'RUST_LOG'
              value: 'info'
            }
          ]
        }
      ]
      scale: {
        minReplicas: 1
        maxReplicas: 5
        rules: [
          {
            name: 'http-scale'
            http: {
              metadata: {
                concurrentRequests: '50'
              }
            }
          }
        ]
      }
    }
  }
}

@description('Principal ID of the Container App managed identity.')
output containerAppPrincipalId string = containerApp.identity.principalId

@description('FQDN of the Container App.')
output containerAppFqdn string = containerApp.properties.configuration.ingress.fqdn

@description('Name of the Container App.')
output containerAppName string = containerApp.name

@description('Resource ID of the Container Apps Environment.')
output containerAppsEnvId string = containerAppsEnv.id
