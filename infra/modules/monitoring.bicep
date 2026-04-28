@description('Azure region for all resources.')
param location string

@description('Base name for resource naming.')
param environmentName string

@description('Tags to apply to all resources.')
param tags object = {}

var logAnalyticsName = '${environmentName}-logs'

resource logAnalytics 'Microsoft.OperationalInsights/workspaces@2023-09-01' = {
  name: logAnalyticsName
  location: location
  tags: tags
  properties: {
    sku: {
      name: 'PerGB2018'
    }
    retentionInDays: 30
  }
}

// TODO: Add Application Insights post-MVP
// resource appInsights 'Microsoft.Insights/components@2020-02-02' = {
//   name: '${environmentName}-appinsights'
//   location: location
//   tags: tags
//   kind: 'web'
//   properties: {
//     Application_Type: 'web'
//     WorkspaceResourceId: logAnalytics.id
//   }
// }

@description('Resource ID of the Log Analytics workspace.')
output logAnalyticsId string = logAnalytics.id

@description('Name of the Log Analytics workspace.')
output logAnalyticsName string = logAnalytics.name
