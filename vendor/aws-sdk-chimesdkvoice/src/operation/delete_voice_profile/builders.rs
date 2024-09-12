// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_voice_profile::_delete_voice_profile_output::DeleteVoiceProfileOutputBuilder;

pub use crate::operation::delete_voice_profile::_delete_voice_profile_input::DeleteVoiceProfileInputBuilder;

impl crate::operation::delete_voice_profile::builders::DeleteVoiceProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_voice_profile::DeleteVoiceProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_voice_profile::DeleteVoiceProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_voice_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteVoiceProfile`.
///
/// <p>Deletes a voice profile, including its voice print and enrollment data. WARNING: This action is not reversible.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteVoiceProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_voice_profile::builders::DeleteVoiceProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_voice_profile::DeleteVoiceProfileOutput,
        crate::operation::delete_voice_profile::DeleteVoiceProfileError,
    > for DeleteVoiceProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_voice_profile::DeleteVoiceProfileOutput,
            crate::operation::delete_voice_profile::DeleteVoiceProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteVoiceProfileFluentBuilder {
    /// Creates a new `DeleteVoiceProfileFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteVoiceProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_voice_profile::builders::DeleteVoiceProfileInputBuilder {
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
        crate::operation::delete_voice_profile::DeleteVoiceProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_voice_profile::DeleteVoiceProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_voice_profile::DeleteVoiceProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_voice_profile::DeleteVoiceProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_voice_profile::DeleteVoiceProfileOutput,
        crate::operation::delete_voice_profile::DeleteVoiceProfileError,
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
    /// <p>The voice profile ID.</p>
    pub fn voice_profile_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.voice_profile_id(input.into());
        self
    }
    /// <p>The voice profile ID.</p>
    pub fn set_voice_profile_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_voice_profile_id(input);
        self
    }
    /// <p>The voice profile ID.</p>
    pub fn get_voice_profile_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_voice_profile_id()
    }
}
