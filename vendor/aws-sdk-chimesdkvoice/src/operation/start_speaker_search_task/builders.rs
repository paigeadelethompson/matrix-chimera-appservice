// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_speaker_search_task::_start_speaker_search_task_output::StartSpeakerSearchTaskOutputBuilder;

pub use crate::operation::start_speaker_search_task::_start_speaker_search_task_input::StartSpeakerSearchTaskInputBuilder;

impl crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_speaker_search_task::StartSpeakerSearchTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_speaker_search_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartSpeakerSearchTask`.
///
/// <p>Starts a speaker search task.</p><important>
/// <p>Before starting any speaker search tasks, you must provide all notices and obtain all consents from the speaker as required under applicable privacy and biometrics laws, and as required under the <a href="https://aws.amazon.com/service-terms/">AWS service terms</a> for the Amazon Chime SDK.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartSpeakerSearchTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput,
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskError,
    > for StartSpeakerSearchTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput,
            crate::operation::start_speaker_search_task::StartSpeakerSearchTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartSpeakerSearchTaskFluentBuilder {
    /// Creates a new `StartSpeakerSearchTaskFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartSpeakerSearchTask as a reference.
    pub fn as_input(&self) -> &crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskInputBuilder {
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
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_speaker_search_task::StartSpeakerSearchTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_speaker_search_task::StartSpeakerSearchTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_speaker_search_task::StartSpeakerSearchTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskOutput,
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskError,
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
    /// <p>The transaction ID of the call being analyzed.</p>
    pub fn transaction_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transaction_id(input.into());
        self
    }
    /// <p>The transaction ID of the call being analyzed.</p>
    pub fn set_transaction_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transaction_id(input);
        self
    }
    /// <p>The transaction ID of the call being analyzed.</p>
    pub fn get_transaction_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transaction_id()
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub fn voice_profile_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.voice_profile_domain_id(input.into());
        self
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub fn set_voice_profile_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_voice_profile_domain_id(input);
        self
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub fn get_voice_profile_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_voice_profile_domain_id()
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn call_leg(mut self, input: crate::types::CallLegType) -> Self {
        self.inner = self.inner.call_leg(input);
        self
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn set_call_leg(mut self, input: ::std::option::Option<crate::types::CallLegType>) -> Self {
        self.inner = self.inner.set_call_leg(input);
        self
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn get_call_leg(&self) -> &::std::option::Option<crate::types::CallLegType> {
        self.inner.get_call_leg()
    }
}
