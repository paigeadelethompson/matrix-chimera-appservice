// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_voice_profiles::_list_voice_profiles_output::ListVoiceProfilesOutputBuilder;

pub use crate::operation::list_voice_profiles::_list_voice_profiles_input::ListVoiceProfilesInputBuilder;

impl crate::operation::list_voice_profiles::builders::ListVoiceProfilesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_voice_profiles::ListVoiceProfilesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_voice_profiles::ListVoiceProfilesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_voice_profiles();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListVoiceProfiles`.
///
/// <p>Lists the voice profiles in a voice profile domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListVoiceProfilesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_voice_profiles::builders::ListVoiceProfilesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_voice_profiles::ListVoiceProfilesOutput,
        crate::operation::list_voice_profiles::ListVoiceProfilesError,
    > for ListVoiceProfilesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_voice_profiles::ListVoiceProfilesOutput,
            crate::operation::list_voice_profiles::ListVoiceProfilesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListVoiceProfilesFluentBuilder {
    /// Creates a new `ListVoiceProfilesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListVoiceProfiles as a reference.
    pub fn as_input(&self) -> &crate::operation::list_voice_profiles::builders::ListVoiceProfilesInputBuilder {
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
        crate::operation::list_voice_profiles::ListVoiceProfilesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_voice_profiles::ListVoiceProfilesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_voice_profiles::ListVoiceProfiles::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_voice_profiles::ListVoiceProfiles::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_voice_profiles::ListVoiceProfilesOutput,
        crate::operation::list_voice_profiles::ListVoiceProfilesError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_voice_profiles::paginator::ListVoiceProfilesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_voice_profiles::paginator::ListVoiceProfilesPaginator {
        crate::operation::list_voice_profiles::paginator::ListVoiceProfilesPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the voice profile domain.</p>
    pub fn voice_profile_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.voice_profile_domain_id(input.into());
        self
    }
    /// <p>The ID of the voice profile domain.</p>
    pub fn set_voice_profile_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_voice_profile_domain_id(input);
        self
    }
    /// <p>The ID of the voice profile domain.</p>
    pub fn get_voice_profile_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_voice_profile_domain_id()
    }
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results in the request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results in the request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results in the request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
