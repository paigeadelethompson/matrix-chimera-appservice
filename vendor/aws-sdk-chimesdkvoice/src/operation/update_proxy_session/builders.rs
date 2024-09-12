// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_proxy_session::_update_proxy_session_output::UpdateProxySessionOutputBuilder;

pub use crate::operation::update_proxy_session::_update_proxy_session_input::UpdateProxySessionInputBuilder;

impl crate::operation::update_proxy_session::builders::UpdateProxySessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_proxy_session::UpdateProxySessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_proxy_session::UpdateProxySessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_proxy_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateProxySession`.
///
/// <p>Updates the specified proxy session details, such as voice or SMS capabilities.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateProxySessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_proxy_session::builders::UpdateProxySessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_proxy_session::UpdateProxySessionOutput,
        crate::operation::update_proxy_session::UpdateProxySessionError,
    > for UpdateProxySessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_proxy_session::UpdateProxySessionOutput,
            crate::operation::update_proxy_session::UpdateProxySessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateProxySessionFluentBuilder {
    /// Creates a new `UpdateProxySessionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateProxySession as a reference.
    pub fn as_input(&self) -> &crate::operation::update_proxy_session::builders::UpdateProxySessionInputBuilder {
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
        crate::operation::update_proxy_session::UpdateProxySessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_proxy_session::UpdateProxySessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_proxy_session::UpdateProxySession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_proxy_session::UpdateProxySession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_proxy_session::UpdateProxySessionOutput,
        crate::operation::update_proxy_session::UpdateProxySessionError,
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
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.voice_connector_id(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_voice_connector_id(input);
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn get_voice_connector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_voice_connector_id()
    }
    /// <p>The proxy session ID.</p>
    pub fn proxy_session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.proxy_session_id(input.into());
        self
    }
    /// <p>The proxy session ID.</p>
    pub fn set_proxy_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_proxy_session_id(input);
        self
    }
    /// <p>The proxy session ID.</p>
    pub fn get_proxy_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_proxy_session_id()
    }
    ///
    /// Appends an item to `Capabilities`.
    ///
    /// To override the contents of this collection use [`set_capabilities`](Self::set_capabilities).
    ///
    /// <p>The proxy session capabilities.</p>
    pub fn capabilities(mut self, input: crate::types::Capability) -> Self {
        self.inner = self.inner.capabilities(input);
        self
    }
    /// <p>The proxy session capabilities.</p>
    pub fn set_capabilities(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Capability>>) -> Self {
        self.inner = self.inner.set_capabilities(input);
        self
    }
    /// <p>The proxy session capabilities.</p>
    pub fn get_capabilities(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Capability>> {
        self.inner.get_capabilities()
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn expiry_minutes(mut self, input: i32) -> Self {
        self.inner = self.inner.expiry_minutes(input);
        self
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn set_expiry_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_expiry_minutes(input);
        self
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn get_expiry_minutes(&self) -> &::std::option::Option<i32> {
        self.inner.get_expiry_minutes()
    }
}
