// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_voice_connector_termination_credentials::_delete_voice_connector_termination_credentials_output::DeleteVoiceConnectorTerminationCredentialsOutputBuilder;

pub use crate::operation::delete_voice_connector_termination_credentials::_delete_voice_connector_termination_credentials_input::DeleteVoiceConnectorTerminationCredentialsInputBuilder;

impl crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_voice_connector_termination_credentials();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteVoiceConnectorTerminationCredentials`.
///
/// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteVoiceConnectorTerminationCredentialsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsOutput,
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsError,
    > for DeleteVoiceConnectorTerminationCredentialsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsOutput,
            crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteVoiceConnectorTerminationCredentialsFluentBuilder {
    /// Creates a new `DeleteVoiceConnectorTerminationCredentialsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteVoiceConnectorTerminationCredentials as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsInputBuilder {
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
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentials::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentials::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsOutput,
        crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsError,
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
    ///
    /// Appends an item to `Usernames`.
    ///
    /// To override the contents of this collection use [`set_usernames`](Self::set_usernames).
    ///
    /// <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
    pub fn usernames(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.usernames(input.into());
        self
    }
    /// <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
    pub fn set_usernames(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_usernames(input);
        self
    }
    /// <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
    pub fn get_usernames(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_usernames()
    }
}