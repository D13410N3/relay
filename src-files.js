var srcIndex = JSON.parse('{\
"document_metrics":["",[],["main.rs"]],\
"document_pii":["",[],["item_collector.rs","main.rs","pii_finder.rs"]],\
"generate_schema":["",[],["main.rs"]],\
"process_event":["",[],["main.rs"]],\
"relay":["",[],["cli.rs","cliapp.rs","main.rs","setup.rs","utils.rs"]],\
"relay_auth":["",[],["lib.rs"]],\
"relay_aws_extension":["",[],["aws_extension.rs","lib.rs"]],\
"relay_base_schema":["",[["metrics",[],["mod.rs","mri.rs","units.rs"]]],["data_category.rs","events.rs","lib.rs","project.rs","spans.rs"]],\
"relay_cabi":["",[],["auth.rs","codeowners.rs","constants.rs","core.rs","ffi.rs","lib.rs","processing.rs"]],\
"relay_cardinality":["",[],["cache.rs","config.rs","error.rs","lib.rs","limiter.rs","redis.rs","statsd.rs","window.rs"]],\
"relay_common":["",[],["glob.rs","glob2.rs","glob3.rs","lib.rs","macros.rs","time.rs"]],\
"relay_config":["",[],["byte_size.rs","config.rs","lib.rs","upstream.rs"]],\
"relay_crash":["",[],["lib.rs"]],\
"relay_dashboard":["",[],["main.rs","stats.rs","utils.rs"]],\
"relay_dynamic_config":["",[],["defaults.rs","error_boundary.rs","feature.rs","global.rs","lib.rs","metrics.rs","project.rs","utils.rs"]],\
"relay_event_derive":["",[],["lib.rs"]],\
"relay_event_normalization":["",[["normalize",[["span",[["description",[["sql",[],["mod.rs","parser.rs"]]],["mod.rs","resource.rs"]]],["attributes.rs","mod.rs","tag_extraction.rs"]]],["breakdowns.rs","contexts.rs","mod.rs","nel.rs","request.rs","user_agent.rs","utils.rs"]],["transactions",[],["mod.rs","processor.rs","rules.rs"]]],["clock_drift.rs","event.rs","event_error.rs","geo.rs","legacy.rs","lib.rs","logentry.rs","mechanism.rs","regexes.rs","remove_other.rs","replay.rs","schema.rs","stacktrace.rs","statsd.rs","timestamp.rs","trimming.rs","validation.rs"]],\
"relay_event_schema":["",[["processor",[],["attrs.rs","chunks.rs","funcs.rs","impls.rs","mod.rs","traits.rs"]],["protocol",[["contexts",[],["app.rs","browser.rs","cloud_resource.rs","device.rs","gpu.rs","mod.rs","monitor.rs","nel.rs","os.rs","otel.rs","profile.rs","replay.rs","reprocessing.rs","response.rs","runtime.rs","trace.rs","user_report_v2.rs"]]],["base.rs","breadcrumb.rs","breakdowns.rs","client_report.rs","clientsdk.rs","constants.rs","debugmeta.rs","device_class.rs","event.rs","exception.rs","fingerprint.rs","logentry.rs","measurements.rs","mechanism.rs","metrics.rs","metrics_summary.rs","mod.rs","nel.rs","relay_info.rs","replay.rs","request.rs","schema.rs","security_report.rs","session.rs","span.rs","stacktrace.rs","tags.rs","templateinfo.rs","thread.rs","transaction.rs","types.rs","user.rs","user_report.rs","utils.rs"]]],["lib.rs"]],\
"relay_ffi":["",[],["lib.rs"]],\
"relay_ffi_macros":["",[],["lib.rs"]],\
"relay_filter":["",[],["browser_extensions.rs","client_ips.rs","common.rs","config.rs","csp.rs","error_messages.rs","generic.rs","legacy_browsers.rs","lib.rs","localhost.rs","releases.rs","transaction_name.rs","web_crawlers.rs"]],\
"relay_jsonschema_derive":["",[],["lib.rs"]],\
"relay_kafka":["",[["producer",[],["mod.rs","schemas.rs","utils.rs"]]],["config.rs","lib.rs","statsd.rs"]],\
"relay_log":["",[],["dashboard.rs","lib.rs","setup.rs","test.rs","utils.rs"]],\
"relay_metrics":["",[["meta",[],["aggregator.rs","mod.rs","protocol.rs","redis.rs"]]],["aggregator.rs","aggregatorservice.rs","bucket.rs","finite.rs","lib.rs","protocol.rs","router.rs","statsd.rs","view.rs"]],\
"relay_monitors":["",[],["lib.rs"]],\
"relay_pii":["",[],["attachments.rs","builtin.rs","compiledconfig.rs","config.rs","convert.rs","generate_selectors.rs","legacy.rs","lib.rs","minidumps.rs","processor.rs","redactions.rs","regexes.rs","selector.rs","utils.rs"]],\
"relay_profiling":["",[],["android.rs","error.rs","extract_from_transaction.rs","lib.rs","measurements.rs","native_debug_image.rs","outcomes.rs","sample.rs","transaction_metadata.rs","utils.rs"]],\
"relay_protocol":["",[],["annotated.rs","condition.rs","impls.rs","lib.rs","macros.rs","meta.rs","size.rs","traits.rs","value.rs"]],\
"relay_protocol_derive":["",[],["lib.rs"]],\
"relay_quotas":["",[],["global.rs","lib.rs","quota.rs","rate_limit.rs","redis.rs"]],\
"relay_redis":["",[],["config.rs","lib.rs","real.rs"]],\
"relay_replays":["",[],["lib.rs","recording.rs","transform.rs"]],\
"relay_sampling":["",[],["config.rs","dsc.rs","evaluation.rs","lib.rs","redis_sampling.rs"]],\
"relay_server":["",[["endpoints",[],["attachments.rs","batch_metrics.rs","batch_outcomes.rs","common.rs","dashboard.rs","envelope.rs","events.rs","forward.rs","health_check.rs","logs.rs","minidump.rs","mod.rs","monitor.rs","nel.rs","project_configs.rs","public_keys.rs","security_report.rs","spans.rs","statics.rs","stats.rs","store.rs","unreal.rs"]],["extractors",[],["content_type.rs","forwarded_for.rs","mime.rs","mod.rs","request_meta.rs","signed_json.rs","start_time.rs"]],["metrics_extraction",[["sessions",[],["mod.rs","types.rs"]],["transactions",[],["mod.rs","types.rs"]]],["event.rs","generic.rs","mod.rs"]],["middlewares",[],["cors.rs","decompression.rs","handle_panic.rs","metrics.rs","mod.rs","normalize_path.rs","trace.rs"]],["services",[["processor",[["span",[],["processing.rs"]]],["attachment.rs","dynamic_sampling.rs","event.rs","profile.rs","replay.rs","report.rs","session.rs","span.rs","unreal.rs"]],["spooler",[],["mod.rs","sql.rs"]]],["global_config.rs","health_check.rs","mod.rs","outcome.rs","outcome_aggregator.rs","processor.rs","project.rs","project_cache.rs","project_local.rs","project_redis.rs","project_upstream.rs","relays.rs","server.rs","store.rs","test_store.rs","upstream.rs"]],["utils",[],["api.rs","buffer.rs","dynamic_sampling.rs","garbage.rs","managed_envelope.rs","metrics_rate_limits.rs","mod.rs","multipart.rs","native.rs","param_parser.rs","rate_limits.rs","retry.rs","semaphore.rs","sizes.rs","sleep_handle.rs","statsd.rs","unreal.rs"]]],["constants.rs","envelope.rs","http.rs","lib.rs","service.rs","statsd.rs"]],\
"relay_spans":["",[],["lib.rs","otel_to_sentry_tags.rs","span.rs","status_codes.rs","trace.rs","utils.rs"]],\
"relay_statsd":["",[],["lib.rs"]],\
"relay_system":["",[],["controller.rs","lib.rs","service.rs","statsd.rs"]],\
"relay_test":["",[],["lib.rs"]],\
"relay_ua":["",[],["lib.rs"]],\
"scrub_minidump":["",[],["main.rs"]]\
}');
createSrcSidebar();
