// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_voice_profile_domain::_update_voice_profile_domain_output::UpdateVoiceProfileDomainOutputBuilder;

pub use crate::operation::update_voice_profile_domain::_update_voice_profile_domain_input::UpdateVoiceProfileDomainInputBuilder;

impl crate::operation::update_voice_profile_domain::builders::UpdateVoiceProfileDomainInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_voice_profile_domain();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateVoiceProfileDomain`.
///
/// <p>Updates the settings for the specified voice profile domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateVoiceProfileDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_voice_profile_domain::builders::UpdateVoiceProfileDomainInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainOutput,
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainError,
    > for UpdateVoiceProfileDomainFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainOutput,
            crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateVoiceProfileDomainFluentBuilder {
    /// Creates a new `UpdateVoiceProfileDomainFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateVoiceProfileDomain as a reference.
    pub fn as_input(&self) -> &crate::operation::update_voice_profile_domain::builders::UpdateVoiceProfileDomainInputBuilder {
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
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomain::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomain::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainOutput,
        crate::operation::update_voice_profile_domain::UpdateVoiceProfileDomainError,
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
    /// <p>The domain ID.</p>
    pub fn voice_profile_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.voice_profile_domain_id(input.into());
        self
    }
    /// <p>The domain ID.</p>
    pub fn set_voice_profile_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_voice_profile_domain_id(input);
        self
    }
    /// <p>The domain ID.</p>
    pub fn get_voice_profile_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_voice_profile_domain_id()
    }
    /// <p>The name of the voice profile domain.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the voice profile domain.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the voice profile domain.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The description of the voice profile domain.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the voice profile domain.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the voice profile domain.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
}