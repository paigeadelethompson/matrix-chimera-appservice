// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `TransactGetItems`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TransactGetItems;
impl TransactGetItems {
    /// Creates a new `TransactGetItems`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::transact_get_items::TransactGetItemsInput,
    ) -> ::std::result::Result<
        crate::operation::transact_get_items::TransactGetItemsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::transact_get_items::TransactGetItemsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::transact_get_items::TransactGetItemsError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::transact_get_items::TransactGetItemsOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::transact_get_items::TransactGetItemsInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("dynamodb", "TransactGetItems", input, runtime_plugins, stop_point).await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
        runtime_plugins = runtime_plugins.with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![
            ::aws_runtime::auth::sigv4::SCHEME_ID,
        ]));
        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for TransactGetItems {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("TransactGetItems");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            TransactGetItemsRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            TransactGetItemsResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "TransactGetItems",
            "dynamodb",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("TransactGetItems")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(TransactGetItemsEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::transact_get_items::TransactGetItemsError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::transact_get_items::TransactGetItemsError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::transact_get_items::TransactGetItemsError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct TransactGetItemsResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for TransactGetItemsResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_transact_get_items::de_transact_get_items_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_transact_get_items::de_transact_get_items_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct TransactGetItemsRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for TransactGetItemsRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::transact_get_items::TransactGetItemsInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::transact_get_items::TransactGetItemsInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::transact_get_items::TransactGetItemsInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-amz-json-1.0");
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "DynamoDB_20120810.TransactGetItems",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_transact_get_items::ser_transact_get_items_input(&input)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct TransactGetItemsEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for TransactGetItemsEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "TransactGetItemsEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<TransactGetItemsInput>()
            .ok_or("failed to downcast to TransactGetItemsInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

// The get_* functions below are generated from JMESPath expressions in the
// operationContextParams trait. They target the operation's input shape.

/// Error type for the `TransactGetItemsError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum TransactGetItemsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    #[allow(missing_docs)] // documentation missing in model
    InvalidEndpointException(crate::types::error::InvalidEndpointException),
    /// <p>Your request rate is too high. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceededException(crate::types::error::ProvisionedThroughputExceededException),
    /// <p>Throughput exceeds the current throughput quota for your account. Please contact <a href="https://aws.amazon.com/support">Amazon Web Services Support</a> to request a quota increase.</p>
    RequestLimitExceeded(crate::types::error::RequestLimitExceeded),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The entire transaction request was canceled.</p>
    /// <p>DynamoDB cancels a <code>TransactWriteItems</code> request under the following circumstances:</p>
    /// <ul>
    /// <li>
    /// <p>A condition in one of the condition expressions is not met.</p></li>
    /// <li>
    /// <p>A table in the <code>TransactWriteItems</code> request is in a different account or region.</p></li>
    /// <li>
    /// <p>More than one action in the <code>TransactWriteItems</code> operation targets the same item.</p></li>
    /// <li>
    /// <p>There is insufficient provisioned capacity for the transaction to be completed.</p></li>
    /// <li>
    /// <p>An item size becomes too large (larger than 400 KB), or a local secondary index (LSI) becomes too large, or a similar validation error occurs because of changes made by the transaction.</p></li>
    /// <li>
    /// <p>There is a user error, such as an invalid data format.</p></li>
    /// <li>
    /// <p>There is an ongoing <code>TransactWriteItems</code> operation that conflicts with a concurrent <code>TransactWriteItems</code> request. In this case the <code>TransactWriteItems</code> operation fails with a <code>TransactionCanceledException</code>.</p></li>
    /// </ul>
    /// <p>DynamoDB cancels a <code>TransactGetItems</code> request under the following circumstances:</p>
    /// <ul>
    /// <li>
    /// <p>There is an ongoing <code>TransactGetItems</code> operation that conflicts with a concurrent <code>PutItem</code>, <code>UpdateItem</code>, <code>DeleteItem</code> or <code>TransactWriteItems</code> request. In this case the <code>TransactGetItems</code> operation fails with a <code>TransactionCanceledException</code>.</p></li>
    /// <li>
    /// <p>A table in the <code>TransactGetItems</code> request is in a different account or region.</p></li>
    /// <li>
    /// <p>There is insufficient provisioned capacity for the transaction to be completed.</p></li>
    /// <li>
    /// <p>There is a user error, such as an invalid data format.</p></li>
    /// </ul><note>
    /// <p>If using Java, DynamoDB lists the cancellation reasons on the <code>CancellationReasons</code> property. This property is not set for other languages. Transaction cancellation reasons are ordered in the order of requested items, if an item has no error it will have <code>None</code> code and <code>Null</code> message.</p>
    /// </note>
    /// <p>Cancellation reason codes and possible error messages:</p>
    /// <ul>
    /// <li>
    /// <p>No Errors:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>None</code></p></li>
    /// <li>
    /// <p>Message: <code>null</code></p></li>
    /// </ul></li>
    /// <li>
    /// <p>Conditional Check Failed:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>ConditionalCheckFailed</code></p></li>
    /// <li>
    /// <p>Message: The conditional request failed.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Item Collection Size Limit Exceeded:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>ItemCollectionSizeLimitExceeded</code></p></li>
    /// <li>
    /// <p>Message: Collection size exceeded.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Transaction Conflict:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>TransactionConflict</code></p></li>
    /// <li>
    /// <p>Message: Transaction is ongoing for the item.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>Provisioned Throughput Exceeded:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>ProvisionedThroughputExceeded</code></p></li>
    /// <li>
    /// <p>Messages:</p>
    /// <ul>
    /// <li>
    /// <p>The level of configured provisioned throughput for the table was exceeded. Consider increasing your provisioning level with the UpdateTable API.</p><note>
    /// <p>This Message is received when provisioned throughput is exceeded is on a provisioned DynamoDB table.</p>
    /// </note></li>
    /// <li>
    /// <p>The level of configured provisioned throughput for one or more global secondary indexes of the table was exceeded. Consider increasing your provisioning level for the under-provisioned global secondary indexes with the UpdateTable API.</p><note>
    /// <p>This message is returned when provisioned throughput is exceeded is on a provisioned GSI.</p>
    /// </note></li>
    /// </ul></li>
    /// </ul></li>
    /// <li>
    /// <p>Throttling Error:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>ThrottlingError</code></p></li>
    /// <li>
    /// <p>Messages:</p>
    /// <ul>
    /// <li>
    /// <p>Throughput exceeds the current capacity of your table or index. DynamoDB is automatically scaling your table or index so please try again shortly. If exceptions persist, check if you have a hot key: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/bp-partition-key-design.html.</p><note>
    /// <p>This message is returned when writes get throttled on an On-Demand table as DynamoDB is automatically scaling the table.</p>
    /// </note></li>
    /// <li>
    /// <p>Throughput exceeds the current capacity for one or more global secondary indexes. DynamoDB is automatically scaling your index so please try again shortly.</p><note>
    /// <p>This message is returned when writes get throttled on an On-Demand GSI as DynamoDB is automatically scaling the GSI.</p>
    /// </note></li>
    /// </ul></li>
    /// </ul></li>
    /// <li>
    /// <p>Validation Error:</p>
    /// <ul>
    /// <li>
    /// <p>Code: <code>ValidationError</code></p></li>
    /// <li>
    /// <p>Messages:</p>
    /// <ul>
    /// <li>
    /// <p>One or more parameter values were invalid.</p></li>
    /// <li>
    /// <p>The update expression attempted to update the secondary index key beyond allowed size limits.</p></li>
    /// <li>
    /// <p>The update expression attempted to update the secondary index key to unsupported type.</p></li>
    /// <li>
    /// <p>An operand in the update expression has an incorrect data type.</p></li>
    /// <li>
    /// <p>Item size to update has exceeded the maximum allowed size.</p></li>
    /// <li>
    /// <p>Number overflow. Attempting to store a number with magnitude larger than supported range.</p></li>
    /// <li>
    /// <p>Type mismatch for attribute to update.</p></li>
    /// <li>
    /// <p>Nesting Levels have exceeded supported limits.</p></li>
    /// <li>
    /// <p>The document path provided in the update expression is invalid for update.</p></li>
    /// <li>
    /// <p>The provided expression refers to an attribute that does not exist in the item.</p></li>
    /// </ul></li>
    /// </ul></li>
    /// </ul>
    TransactionCanceledException(crate::types::error::TransactionCanceledException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-TransactGetItemsError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl TransactGetItemsError {
    /// Creates the `TransactGetItemsError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `TransactGetItemsError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServerError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidEndpointException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ProvisionedThroughputExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::RequestLimitExceeded(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TransactionCanceledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `TransactGetItemsError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `TransactGetItemsError::InvalidEndpointException`.
    pub fn is_invalid_endpoint_exception(&self) -> bool {
        matches!(self, Self::InvalidEndpointException(_))
    }
    /// Returns `true` if the error kind is `TransactGetItemsError::ProvisionedThroughputExceededException`.
    pub fn is_provisioned_throughput_exceeded_exception(&self) -> bool {
        matches!(self, Self::ProvisionedThroughputExceededException(_))
    }
    /// Returns `true` if the error kind is `TransactGetItemsError::RequestLimitExceeded`.
    pub fn is_request_limit_exceeded(&self) -> bool {
        matches!(self, Self::RequestLimitExceeded(_))
    }
    /// Returns `true` if the error kind is `TransactGetItemsError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `TransactGetItemsError::TransactionCanceledException`.
    pub fn is_transaction_canceled_exception(&self) -> bool {
        matches!(self, Self::TransactionCanceledException(_))
    }
}
impl ::std::error::Error for TransactGetItemsError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidEndpointException(_inner) => ::std::option::Option::Some(_inner),
            Self::ProvisionedThroughputExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::RequestLimitExceeded(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::TransactionCanceledException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for TransactGetItemsError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::InvalidEndpointException(_inner) => _inner.fmt(f),
            Self::ProvisionedThroughputExceededException(_inner) => _inner.fmt(f),
            Self::RequestLimitExceeded(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::TransactionCanceledException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for TransactGetItemsError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for TransactGetItemsError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServerError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidEndpointException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ProvisionedThroughputExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::RequestLimitExceeded(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TransactionCanceledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for TransactGetItemsError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::operation::transact_get_items::TransactGetItemsError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::transact_get_items::_transact_get_items_output::TransactGetItemsOutput;

pub use crate::operation::transact_get_items::_transact_get_items_input::TransactGetItemsInput;

mod _transact_get_items_input;

mod _transact_get_items_output;

/// Builders
pub mod builders;