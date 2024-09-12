// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_sip_media_application_logging_configuration::_put_sip_media_application_logging_configuration_output::PutSipMediaApplicationLoggingConfigurationOutputBuilder;

pub use crate::operation::put_sip_media_application_logging_configuration::_put_sip_media_application_logging_configuration_input::PutSipMediaApplicationLoggingConfigurationInputBuilder;

impl crate::operation::put_sip_media_application_logging_configuration::builders::PutSipMediaApplicationLoggingConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_sip_media_application_logging_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutSipMediaApplicationLoggingConfiguration`.
///
/// <p>Updates the logging configuration for the specified SIP media application.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutSipMediaApplicationLoggingConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_sip_media_application_logging_configuration::builders::PutSipMediaApplicationLoggingConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationOutput,
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationError,
    > for PutSipMediaApplicationLoggingConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationOutput,
            crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutSipMediaApplicationLoggingConfigurationFluentBuilder {
    /// Creates a new `PutSipMediaApplicationLoggingConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutSipMediaApplicationLoggingConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::put_sip_media_application_logging_configuration::builders::PutSipMediaApplicationLoggingConfigurationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfiguration::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationOutput,
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The SIP media application ID.</p>
    pub fn sip_media_application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sip_media_application_id(input.into());
        self
    }
    /// <p>The SIP media application ID.</p>
    pub fn set_sip_media_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sip_media_application_id(input);
        self
    }
    /// <p>The SIP media application ID.</p>
    pub fn get_sip_media_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sip_media_application_id()
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn sip_media_application_logging_configuration(mut self, input: crate::types::SipMediaApplicationLoggingConfiguration) -> Self {
        self.inner = self.inner.sip_media_application_logging_configuration(input);
        self
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn set_sip_media_application_logging_configuration(
        mut self,
        input: ::std::option::Option<crate::types::SipMediaApplicationLoggingConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_sip_media_application_logging_configuration(input);
        self
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn get_sip_media_application_logging_configuration(&self) -> &::std::option::Option<crate::types::SipMediaApplicationLoggingConfiguration> {
        self.inner.get_sip_media_application_logging_configuration()
    }
}
