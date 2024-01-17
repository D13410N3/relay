(function() {var implementors = {
"relay_base_schema":[["impl JsonSchema for <a class=\"enum\" href=\"relay_base_schema/events/enum.EventType.html\" title=\"enum relay_base_schema::events::EventType\">EventType</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_base_schema/spans/enum.SpanStatus.html\" title=\"enum relay_base_schema::spans::SpanStatus\">SpanStatus</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_base_schema/metrics/enum.MetricUnit.html\" title=\"enum relay_base_schema::metrics::MetricUnit\">MetricUnit</a>"]],
"relay_event_schema":[["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ContextInner.html\" title=\"struct relay_event_schema::protocol::ContextInner\">ContextInner</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.EventProcessingError.html\" title=\"struct relay_event_schema::protocol::EventProcessingError\">EventProcessingError</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Mechanism.html\" title=\"struct relay_event_schema::protocol::Mechanism\">Mechanism</a>"],["impl&lt;T&gt; JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.PairList.html\" title=\"struct relay_event_schema::protocol::PairList\">PairList</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: JsonSchema + <a class=\"trait\" href=\"relay_event_schema/protocol/trait.AsPair.html\" title=\"trait relay_event_schema::protocol::AsPair\">AsPair</a>,\n    &lt;T as <a class=\"trait\" href=\"relay_event_schema/protocol/trait.AsPair.html\" title=\"trait relay_event_schema::protocol::AsPair\">AsPair</a>&gt;::<a class=\"associatedtype\" href=\"relay_event_schema/protocol/trait.AsPair.html#associatedtype.Value\" title=\"type relay_event_schema::protocol::AsPair::Value\">Value</a>: JsonSchema,</span>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.SystemSdkInfo.html\" title=\"struct relay_event_schema::protocol::SystemSdkInfo\">SystemSdkInfo</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.NetworkReportRaw.html\" title=\"struct relay_event_schema::protocol::NetworkReportRaw\">NetworkReportRaw</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Stacktrace.html\" title=\"struct relay_event_schema::protocol::Stacktrace\">Stacktrace</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Cookies.html\" title=\"struct relay_event_schema::protocol::Cookies\">Cookies</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.IpAddr.html\" title=\"struct relay_event_schema::protocol::IpAddr\">IpAddr</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ProguardDebugImage.html\" title=\"struct relay_event_schema::protocol::ProguardDebugImage\">ProguardDebugImage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.RuntimeContext.html\" title=\"struct relay_event_schema::protocol::RuntimeContext\">RuntimeContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.BrowserContext.html\" title=\"struct relay_event_schema::protocol::BrowserContext\">BrowserContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.TraceId.html\" title=\"struct relay_event_schema::protocol::TraceId\">TraceId</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Request.html\" title=\"struct relay_event_schema::protocol::Request\">Request</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.NetworkReportPhases.html\" title=\"enum relay_event_schema::protocol::NetworkReportPhases\">NetworkReportPhases</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Event.html\" title=\"struct relay_event_schema::protocol::Event\">Event</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.SampleRate.html\" title=\"struct relay_event_schema::protocol::SampleRate\">SampleRate</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.RelayInfo.html\" title=\"struct relay_event_schema::protocol::RelayInfo\">RelayInfo</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Breadcrumb.html\" title=\"struct relay_event_schema::protocol::Breadcrumb\">Breadcrumb</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.OsContext.html\" title=\"struct relay_event_schema::protocol::OsContext\">OsContext</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.Context.html\" title=\"enum relay_event_schema::protocol::Context\">Context</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ReprocessingContext.html\" title=\"struct relay_event_schema::protocol::ReprocessingContext\">ReprocessingContext</a>"],["impl&lt;T&gt; JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Values.html\" title=\"struct relay_event_schema::protocol::Values\">Values</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: JsonSchema,</span>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.SpanId.html\" title=\"struct relay_event_schema::protocol::SpanId\">SpanId</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Contexts.html\" title=\"struct relay_event_schema::protocol::Contexts\">Contexts</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.DeviceContext.html\" title=\"struct relay_event_schema::protocol::DeviceContext\">DeviceContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Addr.html\" title=\"struct relay_event_schema::protocol::Addr\">Addr</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.RegVal.html\" title=\"struct relay_event_schema::protocol::RegVal\">RegVal</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Fingerprint.html\" title=\"struct relay_event_schema::protocol::Fingerprint\">Fingerprint</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.ThreadId.html\" title=\"enum relay_event_schema::protocol::ThreadId\">ThreadId</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Span.html\" title=\"struct relay_event_schema::protocol::Span\">Span</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.MetricsSummary.html\" title=\"struct relay_event_schema::protocol::MetricsSummary\">MetricsSummary</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ClientSdkPackage.html\" title=\"struct relay_event_schema::protocol::ClientSdkPackage\">ClientSdkPackage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.NativeDebugImage.html\" title=\"struct relay_event_schema::protocol::NativeDebugImage\">NativeDebugImage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.PosixSignal.html\" title=\"struct relay_event_schema::protocol::PosixSignal\">PosixSignal</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Query.html\" title=\"struct relay_event_schema::protocol::Query\">Query</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.Level.html\" title=\"enum relay_event_schema::protocol::Level\">Level</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.FrameVars.html\" title=\"struct relay_event_schema::protocol::FrameVars\">FrameVars</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.MachException.html\" title=\"struct relay_event_schema::protocol::MachException\">MachException</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.CodeId.html\" title=\"struct relay_event_schema::protocol::CodeId\">CodeId</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.TransactionNameChange.html\" title=\"struct relay_event_schema::protocol::TransactionNameChange\">TransactionNameChange</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.UserReportV2Context.html\" title=\"struct relay_event_schema::protocol::UserReportV2Context\">UserReportV2Context</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Route.html\" title=\"struct relay_event_schema::protocol::Route\">Route</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.SourceMapDebugImage.html\" title=\"struct relay_event_schema::protocol::SourceMapDebugImage\">SourceMapDebugImage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.HeaderName.html\" title=\"struct relay_event_schema::protocol::HeaderName\">HeaderName</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ResponseContext.html\" title=\"struct relay_event_schema::protocol::ResponseContext\">ResponseContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.JvmDebugImage.html\" title=\"struct relay_event_schema::protocol::JvmDebugImage\">JvmDebugImage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.OtelContext.html\" title=\"struct relay_event_schema::protocol::OtelContext\">OtelContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Headers.html\" title=\"struct relay_event_schema::protocol::Headers\">Headers</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.FrameData.html\" title=\"struct relay_event_schema::protocol::FrameData\">FrameData</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Metrics.html\" title=\"struct relay_event_schema::protocol::Metrics\">Metrics</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.LockReasonType.html\" title=\"enum relay_event_schema::protocol::LockReasonType\">LockReasonType</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Replay.html\" title=\"struct relay_event_schema::protocol::Replay\">Replay</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.MonitorContext.html\" title=\"struct relay_event_schema::protocol::MonitorContext\">MonitorContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.RawStacktrace.html\" title=\"struct relay_event_schema::protocol::RawStacktrace\">RawStacktrace</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.AppContext.html\" title=\"struct relay_event_schema::protocol::AppContext\">AppContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Thread.html\" title=\"struct relay_event_schema::protocol::Thread\">Thread</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ReplayContext.html\" title=\"struct relay_event_schema::protocol::ReplayContext\">ReplayContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ProfileContext.html\" title=\"struct relay_event_schema::protocol::ProfileContext\">ProfileContext</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.InstructionAddrAdjustment.html\" title=\"enum relay_event_schema::protocol::InstructionAddrAdjustment\">InstructionAddrAdjustment</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.TagEntry.html\" title=\"struct relay_event_schema::protocol::TagEntry\">TagEntry</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.DebugMeta.html\" title=\"struct relay_event_schema::protocol::DebugMeta\">DebugMeta</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.GpuContext.html\" title=\"struct relay_event_schema::protocol::GpuContext\">GpuContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.LogEntry.html\" title=\"struct relay_event_schema::protocol::LogEntry\">LogEntry</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.TransactionSource.html\" title=\"enum relay_event_schema::protocol::TransactionSource\">TransactionSource</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Exception.html\" title=\"struct relay_event_schema::protocol::Exception\">Exception</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Message.html\" title=\"struct relay_event_schema::protocol::Message\">Message</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.MetricSummary.html\" title=\"struct relay_event_schema::protocol::MetricSummary\">MetricSummary</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.TraceContext.html\" title=\"struct relay_event_schema::protocol::TraceContext\">TraceContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Frame.html\" title=\"struct relay_event_schema::protocol::Frame\">Frame</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.User.html\" title=\"struct relay_event_schema::protocol::User\">User</a>"],["impl JsonSchema for <a class=\"enum\" href=\"relay_event_schema/protocol/enum.DebugImage.html\" title=\"enum relay_event_schema::protocol::DebugImage\">DebugImage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ExtraValue.html\" title=\"struct relay_event_schema::protocol::ExtraValue\">ExtraValue</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Data.html\" title=\"struct relay_event_schema::protocol::Data\">Data</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.AppleDebugImage.html\" title=\"struct relay_event_schema::protocol::AppleDebugImage\">AppleDebugImage</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.JsonLenientString.html\" title=\"struct relay_event_schema::protocol::JsonLenientString\">JsonLenientString</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.DebugId.html\" title=\"struct relay_event_schema::protocol::DebugId\">DebugId</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.CError.html\" title=\"struct relay_event_schema::protocol::CError\">CError</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Geo.html\" title=\"struct relay_event_schema::protocol::Geo\">Geo</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Tags.html\" title=\"struct relay_event_schema::protocol::Tags\">Tags</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.NsError.html\" title=\"struct relay_event_schema::protocol::NsError\">NsError</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.NelContext.html\" title=\"struct relay_event_schema::protocol::NelContext\">NelContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.TransactionInfo.html\" title=\"struct relay_event_schema::protocol::TransactionInfo\">TransactionInfo</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.NativeImagePath.html\" title=\"struct relay_event_schema::protocol::NativeImagePath\">NativeImagePath</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.CloudResourceContext.html\" title=\"struct relay_event_schema::protocol::CloudResourceContext\">CloudResourceContext</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.EventId.html\" title=\"struct relay_event_schema::protocol::EventId\">EventId</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ClientSdkInfo.html\" title=\"struct relay_event_schema::protocol::ClientSdkInfo\">ClientSdkInfo</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.BodyRaw.html\" title=\"struct relay_event_schema::protocol::BodyRaw\">BodyRaw</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Timestamp.html\" title=\"struct relay_event_schema::protocol::Timestamp\">Timestamp</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.LockReason.html\" title=\"struct relay_event_schema::protocol::LockReason\">LockReason</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.LenientString.html\" title=\"struct relay_event_schema::protocol::LenientString\">LenientString</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.MechanismMeta.html\" title=\"struct relay_event_schema::protocol::MechanismMeta\">MechanismMeta</a>"],["impl JsonSchema for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.HeaderValue.html\" title=\"struct relay_event_schema::protocol::HeaderValue\">HeaderValue</a>"]],
"relay_protocol":[["impl JsonSchema for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl&lt;T&gt; JsonSchema for <a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: JsonSchema,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()