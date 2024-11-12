searchState.loadedDescShard("relay_config", 0, "Configuration for the Relay CLI and server.\nRelay reports readiness regardless of the authentication …\nEmit outcomes as client reports\nEmit outcomes as outcomes\nAuthentication options.\n(default) Relay is ready when authenticated and connected …\nParsing JSON failed.\nRaised if an upstream could not be parsed as URL.\nParsing YAML failed.\nA format using the Brotli algorithm.\nRepresents a size in bytes.\nThe error returned when trying to parse a <code>SpecificSize</code>, …\nThis Relay is run as a canary instance where experiments …\nEvents are held in memory for inspection only.\nCardinality Limiter configuration options.\nConnect to a Redis Cluster.\nConnect to a Redis Cluster.\nCOGS configuration.\nConfig struct.\nIndicates config related errors.\nIndicates config related errors.\nFailed to open the file.\nFailed to save a file.\nThe relay credentials\nThis Relay is run as a default instance.\nRuns normalization, excluding steps that break future …\nCompression using a zlib structure with deflate encoding.\nDetermines how to emit outcomes. For compatibility …\nThe provided string is empty, i.e. “”.\nRaised if the DNS lookup succeeded but an empty result was …\nPersistent buffering configuration for incoming envelopes.\nVersion of the envelope buffering mechanism.\nRun full normalization.\nGeoIp database configuration options.\nA format using the Lempel-Ziv coding (LZ77), with a 32-bit …\nSettings to control Relay’s health checks.\nHttp content encoding for both incoming and outgoing web …\nIdentity function without no compression.\nIndividual configurations for each pool.\nUse an individual pool for each use case.\nThe multiple in the string is invalid, e.g. “100 invalid…\nThe value is invalid, see <code>SpecificSize::new</code>.\nInvalid config value\nRaised if the DNS lookup for an upstream host failed.\nProject configurations are managed by the upstream.\nMinimal version of a config for dumping out.\nThe value is missing the multiple of bytes, e.g. “100”.\nThe provided string is missing a value, e.g. “B”.\nConnect to multiple Redis instances for multiple writes.\nConnect to multiple Redis instances for multiple writes.\nRaised if no host was provided.\nRaised if a path was added to a URL.\nDo not emit any outcomes\nConfiguration for normalization in this Relay.\nConfiguration for the level of normalization this Relay …\nConfiguration values for the outcome aggregator\nOutcome generation specific configuration values.\nStructure used to hold information about configuration …\nError returned when parsing an invalid <code>RelayMode</code>.\nAdditional configuration options for a redis client.\nControls Sentry-internal event processing.\nThe user attempted to run Relay with processing enabled, …\nThis relay acts as a proxy for all requests and events.\nControls responses from the readiness health check …\nRedis configuration.\nReference to the <code>RedisConfig</code> with the final …\nConfigurations for the various Redis pools used by Relay.\nHelper struct bundling connections and options for the …\nRelay specific configuration values.\nInformation on a downstream Relay.\nThe instance type of Relay.\nThe operation mode of a relay.\nControls traffic steering.\nConnect to a single Redis instance.\nConnect to a single Redis instance.\nStruct that can serialize a string to a single Redis …\nPersistent buffering configuration.\nThis relay is configured statically in the file system.\nAll pools should be configured the same way.\nUse one pool for everything.\nRaised if an unknown or unsupported scheme is encountered.\nThe upstream target is a type that holds all the …\nIndicates failures in the upstream error api.\nRaised if a URL cannot be parsed into an upstream …\nUse the spooler service, which only buffers envelopes for …\nUse the envelope buffer, through which all envelopes pass …\nReturns <code>true</code> if unknown items should be accepted and …\nMetrics aggregator configuration.\nConfigures the outcome aggregator.\nReturns aggregator config for a given metrics namespace.\nReturns true of outcomes are emitted via http, kafka, or …\nOverride configuration with values coming from other …\nReturn the value in bytes.\nChunk size of attachments in bytes.\nMaximum chunk size of attachments for Kafka.\nThe maximum time interval (in milliseconds) that an …\nThe maximum number of outcomes that are batched before …\nDefines the width of the buckets into which outcomes are …\nCreate a byte size from bytes.\nReturns the expiry timeout for cached misses before trying …\nCache vacuum interval in seconds for the in memory cache.\nCache vacuum interval for the cardinality limiter in …\nMaximum amount of COGS measurements buffered in memory.\nResource ID to use for Relay COGS measurements.\nChecks if the config is already initialized.\nSets the connection timeout used by the pool, in seconds.\nReturns the number of cores to use for thread pools.\nReturn the current credentials\nConstructs a disabled processing configuration.\nReturns configuration for the default metrics aggregator.\nReturns the duration in which downstream relays are …\nReturns whether this Relay should emit client outcomes\nControls wheather client reported outcomes should be …\nReturns whether this Relay should emit outcomes.\nControls whether outcomes will be emitted when processing …\nTrue if the Relay should do processing. Defaults to <code>false</code>.\nReturns the maximum number of buffered envelopes\nDefines how often all buckets are flushed, in seconds.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGiven a DSN this returns an upstream descriptor that …\nCreates a config from a JSON value.\nLoads a config from a given config folder.\nGenerates new random credentials.\nThe path to the GeoIp database required for event …\nGeoIp DB file source.\nReturns a URL relative to the upstream.\nReturns the interval in seconds in which fresh global …\nReturns <code>true</code> if the config is ready to use.\nMaximum memory watermark in bytes.\nMaximum memory watermark as a percentage of maximum system …\nHealth check probe timeout.\nInterval to refresh internal health checks.\nReturns the host as a string.\nThe host the relay should bind to (network interface).\nThe host the relay should bind to (network interface).\nReturns the interval at which Realy should try to …\nReturns the connection timeout for all upstream HTTP …\nContent encoding of upstream requests.\nReturns whether metrics should be sent globally through a …\nReturns the custom HTTP “Host” header.\nReturns the failed upstream request retry interval.\nThe maximum time of experiencing uninterrupted network …\nTime of continued project request failures before Relay …\nTime Relay waits before retrying an upstream request.\nReturns the default timeout for all upstream HTTP requests.\nThe globally unique ID of the relay.\nThe globally unique ID of the relay.\nSets the idle timeout used by the pool, in seconds.\nCreate a byte size from bytes, inferring the most …\nThe instance type of this relay.\nThe instance type of this relay.\nMarks an internal relay that has privileged access to more …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns a version of the upstream descriptor that is …\nReturns <code>true</code> if the <code>RelayInstance</code> is of type …\nConfiguration name and list of Kafka configuration …\nKafka producer configurations.\nthe kafka bootstrap.servers configuration string\nWhether to validate the topics against Kafka.\nWhether to validate the supplied topics by calling Kafka’…\nReturns the server keep-alive timeout in seconds.\nCreate a byte size from 1024-based kibibytes.\nReturns the error kind of the error.\nLevel of normalization for Relay to apply to incoming data.\nReturns the listen address.\nReturns the interval in seconds in which local project …\nThe log level of this relay.\nReturns logging configuration.\nReturns the maximum payload size for chunks\nReturns the maximum payload size for file uploads and …\nReturns the maximum payload size for general API requests.\nReturns the maximum size of each attachment.\nReturns the maximum combined size of attachments or …\nReturns the maximum payload size of a monitor check-in in …\nReturns the maximum combined size of client reports in …\nReturns the maximum number of active queries\nReturns the maximum number of active requests\nMaximum number of connections managed by the pool.\nReturns the maximum size of an envelope payload in bytes.\nReturns the maximum size of an event payload in bytes.\nSets the maximum lifetime of connections in the pool, in …\nMaximum memory watermark in bytes.\nMaximum memory watermark as a percentage of maximum system …\nReturns the maximum payload size of metric buckets in …\nReturns the maximum payload size for a profile\nMaximum rate limit to report to clients in seconds.\nMaximum rate limit to report to clients.\nReturns the maximum payload size for a compressed replay.\nReturns the maximum message size for an uncompressed …\nReturns the maximum payload size for an uncompressed …\nMaximum future timestamp of ingested data.\nMaximum future timestamp of ingested events.\nReturns the maximum number of sessions per envelope.\nMaximum age of ingested sessions. Older sessions will be …\nMaximum age of ingested sessions. Older sessions will be …\nReturns the maximum payload size of a span in bytes.\nReturns the maximum payload size of a statsd metric in …\nCreate a byte size from 1024-based mebibytes.\nRefresh frequency for polling new memory stats.\nThe refresh frequency of memory stats which are used to …\nWhether metric stats are collected and emitted.\nReturns the default tags for statsd metrics.\nReturns the name of the hostname tag that should be …\nMaximum metrics batch size in bytes.\nReturns the interval for periodic metrics emitted from …\nReturn the prefix for statsd metrics.\nReturns the global sample rate for all metrics.\nMinimum amount of idle connections kept alive in the pool.\nThe operation mode of this relay.\nThe operation mode of this relay.\nReturns the value for the <code>content-encoding</code> HTTP header.\nCreates a new RelayInfo\nManually constructs an upstream descriptor.\nLevel of normalization for Relay to apply to incoming data.\nReturns the width of the buckets into which outcomes are …\nReturns the maximum interval that an outcome may be batched\nReturns the maximum number of outcomes that are batched …\nThe originating source of the outcome\nOutcome source\nReturns <code>true</code> when project IDs should be overriden rather …\nAlways override project IDs from the URL and DSN with the …\nParses a <code>HttpEncoding</code> from its <code>content-encoding</code> header …\nReturns the filename of the config file.\nCreates an <code>AggregatorConfig</code> that is compatible with every …\nReturns the upstream port\nThe port to bind for the unencrypted relay HTTP server.\nThe port to bind for the unencrypted relay HTTP server.\nHealth check probe timeout in milliseconds.\n“true” if processing is enabled “false” otherwise\nTrue if the Relay should do processing.\nReturns the expiry timeout for cached projects.\nGet filename for static project config.\nReturns the grace period for project caches.\nDefault prefix to use when looking up project configs in …\nPrefix to use when looking up project configs in Redis. …\nReturns the public key if set.\nThe public key of the relay\nThe public key of the relay\nThe public key that this Relay uses to authenticate and …\nReturns the duration in which batchable project config …\nReturns the maximum size of a project config query.\nThe maximum number of seconds a query is allowed to take …\nSets the read timeout out on the connection, in seconds.\nControls responses from the readiness health check …\nRedis servers to connect to for project configs, …\nRedis hosts to connect to for storing state for rate …\nthe redis server url\nInterval to refresh internal health checks.\nRegenerates the relay credentials.\nThe relay part of the config.\nReturns the expiry timeout for cached relay infos (public …\nReturns the relay ID.\nReturns the instance type of relay.\nReturns the relay mode.\nSet new credentials.\nReturns <code>true</code> if the full project state should be requested …\nReturns <code>true</code> if Relay requires authentication for …\nSaves the config in the given config folder as config.yml\nReturns the upstream’s connection scheme.\nReturns configuration for non-default metrics aggregator.\nAdditional kafka producer configurations.\nReturns the secret key if set.\nThe secret key of the relay\nThe secret key of the relay\nReturns logging configuration.\nThe maximum number of seconds to wait for pending …\nshutdown timeout\nCreates a new Redis config for a single Redis instance …\nReturns the socket address of the upstream.\nDefines the source string registered in the outcomes …\nReturns the refresh frequency for disk usage monitoring as …\nNumber of encoded envelope bytes that need to be …\nReturns the time after which we drop envelopes as a …\nMaximum number of connections to create to buffer file.\nThe maximum size of the buffer, in bytes.\nThe maximum size of the memory buffer, in bytes.\nMinimum number of connections to create to buffer file.\nReturns the path of the buffer file if the …\nUnspool interval in milliseconds.\nReturns the maximum number of envelopes that can be put in …\nReturns the relative memory usage up to which the disk …\nReturns <code>true</code> if version 2 of the spooling mechanism is …\nReturn the statically configured Relays.\nStatically authenticated downstream relays.\nReturns the socket addresses for statsd.\nTCP listen backlog to configure on Relay’s listening …\nReturns the password for the identity bundle\nPassword for the PKCS12 archive.\nReturns the path to the identity bundle\nThe path to the identity (DER-encoded PKCS12) to use for …\nReturns the TLS listen address.\nOptional port to bind for the encrypted relay HTTPS server.\nSerializes this configuration to JSON.\nDumps out a YAML string of the values.\nKafka topic names.\nAll unused but configured topic assignments.\nThe upstream relay or sentry instance.\nThe upstream relay or sentry instance.\nReturns the upstream target as descriptor.\nAlternate upstream provided through a Sentry DSN. Key and …\nSets the write timeout on the connection, in seconds.\nRedis nodes urls of the cluster.\nConfigurations for the Redis instances.\nOptions of the Redis config.\nReference to the Redis nodes urls of the cluster.\nConfigurations for the Redis instances.\nOptions of the Redis config.\nOptions of the Redis config.\nReference to the Redis node url.\nConfiguration for the <code>cardinality</code> pool.\nConfiguration for the <code>project_configs</code> pool.\nConfiguration for the <code>quotas</code> pool.\nConfiguration for the <code>cardinality</code> pool.\nConfiguration for the <code>project_configs</code> pool.\nConfiguration for the <code>quotas</code> pool.\nParameters used for metric aggregation.\nMatches if all conditions are true.\nCondition that needs to be met for a metric or bucket to …\nChecks for equality on a specific field.\nDefines a field and a field value to compare to when a …\nField that allows comparison to a metric or bucket’s …\nInverts the condition.\nMatches if any condition is true.\nContains an <code>AggregatorServiceConfig</code> for a specific scope.\nThe config used by the internal aggregator.\nCondition that needs to be met for a metric or bucket to …\nThe configuration of the secondary aggregator.\nThe flushing interval in milliseconds that determines how …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if the condition matches the given namespace.\nThe approximate maximum number of bytes submitted within …\nName of the aggregator, used to tag statsd metrics.\nInner rules to combine.\nInner rules to combine.\nInner rule to negate.")