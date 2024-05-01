export * from './accounts.service';
import { AccountsService } from './accounts.service';
export * from './apps.service';
import { AppsService } from './apps.service';
export * from './authTokens.service';
import { AuthTokensService } from './authTokens.service';
export * from './channels.service';
import { ChannelsService } from './channels.service';
export * from './customDomains.service';
import { CustomDomainsService } from './customDomains.service';
export * from './deviceCodes.service';
import { DeviceCodesService } from './deviceCodes.service';
export * from './keyValuePairs.service';
import { KeyValuePairsService } from './keyValuePairs.service';
export * from './keyValueStores.service';
import { KeyValueStoresService } from './keyValueStores.service';
export * from './oci.service';
import { OciService } from './oci.service';
export * from './payments.service';
import { PaymentsService } from './payments.service';
export * from './personalAccessTokens.service';
import { PersonalAccessTokensService } from './personalAccessTokens.service';
export * from './revisions.service';
import { RevisionsService } from './revisions.service';
export * from './sqlDatabases.service';
import { SqlDatabasesService } from './sqlDatabases.service';
export * from './variablePairs.service';
import { VariablePairsService } from './variablePairs.service';
export const APIS = [AccountsService, AppsService, AuthTokensService, ChannelsService, CustomDomainsService, DeviceCodesService, KeyValuePairsService, KeyValueStoresService, OciService, PaymentsService, PersonalAccessTokensService, RevisionsService, SqlDatabasesService, VariablePairsService];
