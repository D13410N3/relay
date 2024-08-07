searchState.loadedDescShard("relay_metrics", 0, "Metric protocol, aggregation and processing for Sentry.\nBit (<code>&quot;bit&quot;</code>), corresponding to 1/8 of a byte.\nAn aggregation of metric values.\nRelay internal metadata for a metric bucket.\nThe aggregated value of a metric bucket.\nA view into a metrics bucket. Sometimes also called a …\nA view into the datapoints of a <code>BucketValue</code>.\nA view into a slice of metric buckets.\nIterator slicing a <code>BucketsView</code> into smaller views …\nIterator yielding all items contained in a <code>BucketsView</code>.\nByte (<code>&quot;byte&quot;</code>).\nCounts instances of an event.\nCounts instances of an event (<code>MetricType::Counter</code>).\nA counter metric.\nType used for Counter metric\nUser-defined metrics directly sent by SDKs and …\nUser-defined units without built-in conversion or default.\nCustom user-defined units without builtin conversion.\nDay (<code>&quot;day&quot;</code>), 86,400 seconds.\nBuilds a statistical distribution over values reported.\nBuilds a statistical distribution over values reported (…\nA distribution metric.\nType of distribution entries\nA distribution of values within a <code>Bucket</code>.\nA time duration, defaulting to <code>&quot;millisecond&quot;</code>.\nTime duration units used in <code>MetricUnit::Duration</code>.\nSmallest positive normal value.\nExabyte (<code>&quot;exabyte&quot;</code>), 10^18 bytes.\nExbibyte (<code>&quot;exbibyte&quot;</code>), 2^60 bytes.\nA finite 64-bit floating point type.\nFractions such as percentages, defaulting to <code>&quot;ratio&quot;</code>.\nUnits of fraction used in <code>MetricUnit::Fraction</code>.\nStores absolute snapshots of values.\nStores absolute snapshots of values.\nA gauage metric.\nType used for Gauge entries\nA snapshot of values within a <code>Bucket</code>.\nGibibyte (<code>&quot;gibibyte&quot;</code>), 2^30 bytes.\nGigabyte (<code>&quot;gigabyte&quot;</code>), 10^9 bytes.\nHour (<code>&quot;hour&quot;</code>), 3600 seconds.\nSize of information derived from bytes, defaulting to …\nSize of information derived from bytes, used in …\nKibibyte (<code>&quot;kibibyte&quot;</code>), 2^10 bytes.\nKilobyte (<code>&quot;kilobyte&quot;</code>), 10^3 bytes.\nLargest finite value.\nSmallest finite value.\nMebibyte (<code>&quot;mebibyte&quot;</code>), 2^20 bytes.\nMegabyte (<code>&quot;megabyte&quot;</code>), 10^6 bytes.\nOptimized string represenation of a metric name.\nThe namespace of a metric.\nA unique identifier for metrics including typing and …\nThe type of a <code>MetricResourceIdentifier</code>, determining its …\nThe unit of measurement of a metric value.\nMicrosecond (<code>&quot;microsecond&quot;</code>), 10^-6 seconds.\nMillisecond (<code>&quot;millisecond&quot;</code>), 10^-3 seconds.\nMinute (<code>&quot;minute&quot;</code>), 60 seconds.\nNanosecond (<code>&quot;nanosecond&quot;</code>), 10^-9 seconds.\nUntyped value without a unit (<code>&quot;&quot;</code>).\nIterator over parsed metrics returned from …\nError type returned when parsing <code>FiniteF64</code> fails.\nAn error returned when metrics or MRIs cannot be parsed.\nAn error parsing a <code>MetricUnit</code> or one of its variants.\nPebibyte (<code>&quot;pebibyte&quot;</code>), 2^50 bytes.\nRatio expressed as a fraction of <code>100</code>. <code>100%</code> equals a ratio …\nPetabyte (<code>&quot;petabyte&quot;</code>), 10^15 bytes.\nMetrics extracted from profile functions.\nFloating point fraction of <code>1</code>.\nFull second (<code>&quot;second&quot;</code>).\nMetrics extracted from sessions.\nCounts the number of unique reported values.\nCounts the number of unique reported values.\nA set metric.\nType used for set elements in Set metric\nA set of unique values.\nA view into the datapoints of a set metric.\nMetrics extracted from spans.\nMetric stats.\nTebibyte (<code>&quot;tebibyte&quot;</code>), 2^40 bytes.\nTerabyte (<code>&quot;terabyte&quot;</code>), 10^12 bytes.\nMetrics extracted from transaction events.\nError type returned when conversion to <code>FiniteF64</code> fails.\nUnescaper’s <code>Error</code>.\nA unix timestamp (full seconds elapsed since 1970-01-01 …\nAn unknown and unsupported metric.\nWeek (<code>&quot;week&quot;</code>), 604,800 seconds.\nComputes the absolute value of self.\nCore functionality of metrics aggregation.\nReturns all namespaces/variants of this enum.\nReturns the timestamp as chrono datetime.\nReturns the number of seconds since the UNIX epoch start.\nReturns the same bucket view as a bucket view over a slice.\nReturn the shortcode for this metric type.\nReturns the string representation for this metric type.\nReturns the string representation for this metric unit.\nReturns the string representation for this duration unit.\nReturns the string representation for this information …\nReturns the string representation for this fraction unit.\nReturns the string representation of this unit.\nReturns the average of all values reported in this bucket.\nIterator which slices the source view into segments with …\nCOGS related metric utilities.\nEstimates the number of bytes needed to encode the bucket …\nThe number of times this bucket was updated with a new …\nReturns a bucket value representing a counter with the …\nCreates a <code>DistributionValue</code> containing the given arguments.\nCreates a <code>DistributionValue</code> containing the given arguments.\nReturns a bucket value representing a distribution with a …\nEstimates the number of bytes needed to serialize the …\nIs <code>true</code> if this metric was extracted from a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a unix timestamp from the given chrono <code>DateTime</code>.\nConverts the given <code>Instant</code> into a UNIX timestamp.\nCreates a unix timestamp from the given number of seconds.\nCreates a unix timestamp from the given system time.\nReturns a bucket value representing a gauge with a single …\nReturns <code>true</code> if metric stats are enabled for this …\nInserts a new value into the gauge.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts the MRI into an owned version with a static …\nWhether the metadata does not contain more information …\nReturns <code>true</code> if this bucket contains no values.\nReturns whether the view contains any buckets.\nReturns <code>true</code> if this bucket view contains no values.\nReturns <code>true</code> if this set contains no values.\nReturns <code>true</code> if the metric_unit is <code>None</code>.\nIterator over all buckets in the view.\nIterator over all datapoints contained in this set metric.\nThe last value reported in the bucket.\nReturns the number of raw data points in this value.\nReturns the amount of partial or full buckets in the view.\nNumber of raw datapoints in this view.\nAmount of datapoints contained within the set view.\nReturns the maximum of two numbers.\nThe maximum value reported in the bucket.\nMerges two gauge snapshots.\nMerges the given <code>bucket_value</code> into <code>self</code>.\nMerges another metadata object into the current one.\nHow many times the bucket was merged.\nFunctionality for aggregating and storing of metrics …\nReturns the metadata for this bucket.\nRelay internal metadata for a metric bucket.\nReturns the minimum of two numbers.\nThe minimum value reported in the bucket.\nName of the bucket.\nThe display name of the metric in the allowed character …\nThe name of the metric in MRI (metric resource identifier) …\nExtracts the namespace from a well formed MRI.\nThe namespace for this metric.\nCreates a fresh metadata instance.\nCreates a finite float if the value is finite.\nCreates a new buckets view containing all data from the …\nCreates a new bucket view of a bucket.\nCreates a finite float without checking whether the value …\nReturns the current timestamp.\nParses and validates an MRI.\nParses a <code>CustomUnit</code> from a string.\nParses a single metric aggregate from the raw protocol.\nParses a set of metric aggregates from the raw protocol.\nParses an MRI from a string and a separate type.\nReceived timestamp of the first metric in this bucket.\nRemoves the value of the specified tag.\nAdds two numbers, saturating at the maximum and minimum …\nDivides two numbers, saturating at the maximum and minimum …\nMultiplies two numbers, saturating at the maximum and …\nAdds two numbers, saturating at the maximum and minimum …\nSelects a sub-view of the current view.\nReturns a bucket value representing a set with a single …\nReturns a bucket value representing a set with a single …\nReturns a bucket value representing a set with a single …\nCreates a gauge snapshot from a single value.\nCalculates a split for this bucket if its estimated …\nThe sum of all values reported in the bucket.\nReturns the value of the specified tag if it exists.\nReturns the value of the specified tag if it exists.\nName of the bucket.\nA list of tags adding dimensions to the metric for …\nTimestamp of the bucket.\nThe start time of the bucket’s time window.\nReturns the plain <code>f64</code>.\nExtracts the namespace from a well formed MRI.\nExtracts the type from a well formed MRI.\nReturns the type of this value.\nType of the value of the bucket view.\nThe type of a metric, determining its aggregation and …\nThe verbatim unit name of the metric value.\nValue of the bucket view.\nThe type and aggregated values of this bucket.\nWidth of the bucket.\nThe length of the time window in seconds.\nAny error that may occur during aggregation.\nA collector of <code>Bucket</code> submissions.\nParameters used by the <code>Aggregator</code>.\nShifts the flush time by an offset based on the bucket key …\nConfiguration value for <code>AggregatorConfig::flush_batching</code>.\nA metric bucket had invalid characters in the metric name.\nA metric bucket had a too long string (metric name or a …\nA metric bucket’s timestamp was out of the configured …\nInternal error: Attempted to merge two metric buckets of …\nDo not apply shift.\nShifts the flush time by an offset based on the partition …\nShifts the flush time by an offset based on the <code>ProjectKey</code>.\nA metric bucket is too large for the per-project bytes …\nA metric bucket is too large for the global bytes limit.\nA metric bucket had an unknown namespace in the metric …\nReturns the number of buckets in the aggregator.\nDetermines the wall clock time interval for buckets in …\nThe batching mode for the flushing of the aggregator.\nThe number of logical partitions that can receive flushed …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe initial delay in seconds to wait before flushing a …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts this aggregator into a vector of <code>Bucket</code>.\nThe length the name of a metric is allowed to be.\nMaximum amount of bytes used for metrics aggregation per …\nThe time in seconds that a timestamp may be in the future.\nThe age in seconds of the oldest allowed bucket timestamp.\nThe length the tag key is allowed to be.\nThe length the tag value is allowed to be.\nMaximum amount of bytes used for metrics aggregation.\nMerge a bucket into this aggregator.\nMerge a bucket into this aggregator.\nReturns the name of the aggregator.\nLike <code>Self::new</code>, but with a provided name.\nCreate a new aggregator.\nPop and return the partitions with buckets that are …\nEstimates the number of bytes needed to encode the tags.\nReturns the valid range for metrics timestamps.\nReturns <code>true</code> if the cost trackers value is larger than the …\nCOGS estimator based on the bucket count.\nCOGS estimator based on the estimated size of each bucket …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA metadata item.\nA code location.\nA location metadata pointing to the code location where …\nA metrics meta aggregator.\nA metric metadata item.\nRedis metric meta\nA Unix timestamp that is truncated to the start of the day.\nUnknown item.\nThe absolute file path.\nAdds a new meta item to the aggregator.\nReturns the underlying unix timestamp, truncated to the …\nRemove all contained state related to a project.\nSource code of the current line (<code>lineno</code>).\nThe relative file path.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe containing function name.\nRetrieves all currently relevant metric meta for a project.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe line number.\nThe contained metadata mapped by MRI.\nThe containing module name or path.\nCreates a new Redis metrics meta store.\nCreates a new metrics meta aggregator.\nCreates a new <code>StartOfDayUnixTimestamp</code> from a timestamp by …\nSource code of the lines after <code>lineno</code>.\nSource code leading up to <code>lineno</code>.\nStores metric metadata in Redis.\nTimestamp scope for the contained metadata.")