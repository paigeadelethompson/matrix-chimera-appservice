// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `CreateChannelModerator`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateChannelModerator;
impl CreateChannelModerator {
    /// Creates a new `CreateChannelModerator`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::create_channel_moderator::CreateChannelModeratorInput,
    ) -> ::std::result::Result<
        crate::operation::create_channel_moderator::CreateChannelModeratorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_channel_moderator::CreateChannelModeratorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::create_channel_moderator::CreateChannelModeratorError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::create_channel_moderator::CreateChannelModeratorOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::create_channel_moderator::CreateChannelModeratorInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "chimesdkmessaging",
            "CreateChannelModerator",
            input,
            runtime_plugins,
            stop_point,
        )
        .await
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateChannelModerator {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateChannelModerator");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            CreateChannelModeratorRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            CreateChannelModeratorResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "CreateChannelModerator",
            "chimesdkmessaging",
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateChannelModerator")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(CreateChannelModeratorEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::create_channel_moderator::CreateChannelModeratorError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::create_channel_moderator::CreateChannelModeratorError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::create_channel_moderator::CreateChannelModeratorError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct CreateChannelModeratorResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for CreateChannelModeratorResponseDeserializer {
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
        let parse_result = if !success && status != 201 || force_error {
            crate::protocol_serde::shape_create_channel_moderator::de_create_channel_moderator_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_channel_moderator::de_create_channel_moderator_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct CreateChannelModeratorRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateChannelModeratorRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::create_channel_moderator::CreateChannelModeratorInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::create_channel_moderator::CreateChannelModeratorInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                let input_1 = &_input.channel_arn;
                let input_1 = input_1
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("channel_arn", "cannot be empty or unset"))?;
                let channel_arn = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                if channel_arn.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                        "channel_arn",
                        "cannot be empty or unset",
                    ));
                }
                ::std::write!(output, "/channels/{ChannelArn}/moderators", ChannelArn = channel_arn).expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_channel_moderator::CreateChannelModeratorInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                let builder = crate::protocol_serde::shape_create_channel_moderator::ser_create_channel_moderator_headers(input, builder)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/json");
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_create_channel_moderator::ser_create_channel_moderator_input(&input)?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct CreateChannelModeratorEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateChannelModeratorEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "CreateChannelModeratorEndpointParamsInterceptor"
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
            .downcast_ref::<CreateChannelModeratorInput>()
            .ok_or("failed to downcast to CreateChannelModeratorInput")?;

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

/// Error type for the `CreateChannelModeratorError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateChannelModeratorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>The client is permanently forbidden from making the request.</p>
    ForbiddenException(crate::types::error::ForbiddenException),
    /// <p>The request exceeds the resource limit.</p>
    ResourceLimitExceededException(crate::types::error::ResourceLimitExceededException),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailureException(crate::types::error::ServiceFailureException),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailableException(crate::types::error::ServiceUnavailableException),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClientException(crate::types::error::ThrottledClientException),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClientException(crate::types::error::UnauthorizedClientException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateChannelModeratorError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl CreateChannelModeratorError {
    /// Creates the `CreateChannelModeratorError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `CreateChannelModeratorError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::BadRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ConflictException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ForbiddenException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceLimitExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ServiceFailureException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ServiceUnavailableException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ThrottledClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::UnauthorizedClientException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(self, Self::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(self, Self::ConflictException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(self, Self::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::ResourceLimitExceededException`.
    pub fn is_resource_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ResourceLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::ServiceFailureException`.
    pub fn is_service_failure_exception(&self) -> bool {
        matches!(self, Self::ServiceFailureException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(self, Self::ServiceUnavailableException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::ThrottledClientException`.
    pub fn is_throttled_client_exception(&self) -> bool {
        matches!(self, Self::ThrottledClientException(_))
    }
    /// Returns `true` if the error kind is `CreateChannelModeratorError::UnauthorizedClientException`.
    pub fn is_unauthorized_client_exception(&self) -> bool {
        matches!(self, Self::UnauthorizedClientException(_))
    }
}
impl ::std::error::Error for CreateChannelModeratorError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::BadRequestException(_inner) => ::std::option::Option::Some(_inner),
            Self::ConflictException(_inner) => ::std::option::Option::Some(_inner),
            Self::ForbiddenException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceLimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceFailureException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceUnavailableException(_inner) => ::std::option::Option::Some(_inner),
            Self::ThrottledClientException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnauthorizedClientException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for CreateChannelModeratorError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::BadRequestException(_inner) => _inner.fmt(f),
            Self::ConflictException(_inner) => _inner.fmt(f),
            Self::ForbiddenException(_inner) => _inner.fmt(f),
            Self::ResourceLimitExceededException(_inner) => _inner.fmt(f),
            Self::ServiceFailureException(_inner) => _inner.fmt(f),
            Self::ServiceUnavailableException(_inner) => _inner.fmt(f),
            Self::ThrottledClientException(_inner) => _inner.fmt(f),
            Self::UnauthorizedClientException(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateChannelModeratorError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateChannelModeratorError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::BadRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ConflictException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ForbiddenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceLimitExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ServiceFailureException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ThrottledClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::UnauthorizedClientException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for CreateChannelModeratorError {
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
impl ::aws_types::request_id::RequestId for crate::operation::create_channel_moderator::CreateChannelModeratorError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::create_channel_moderator::_create_channel_moderator_output::CreateChannelModeratorOutput;

pub use crate::operation::create_channel_moderator::_create_channel_moderator_input::CreateChannelModeratorInput;

mod _create_channel_moderator_input;

mod _create_channel_moderator_output;

/// Builders
pub mod builders;
