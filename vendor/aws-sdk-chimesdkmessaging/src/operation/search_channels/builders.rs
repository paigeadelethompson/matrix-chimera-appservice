// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_channels::_search_channels_output::SearchChannelsOutputBuilder;

pub use crate::operation::search_channels::_search_channels_input::SearchChannelsInputBuilder;

impl crate::operation::search_channels::builders::SearchChannelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_channels::SearchChannelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_channels::SearchChannelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_channels();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchChannels`.
///
/// <p>Allows the <code>ChimeBearer</code> to search channels by channel members. Users or bots can search across the channels that they belong to. Users in the <code>AppInstanceAdmin</code> role can search across all channels.</p>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchChannelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_channels::builders::SearchChannelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_channels::SearchChannelsOutput,
        crate::operation::search_channels::SearchChannelsError,
    > for SearchChannelsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_channels::SearchChannelsOutput,
            crate::operation::search_channels::SearchChannelsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchChannelsFluentBuilder {
    /// Creates a new `SearchChannelsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchChannels as a reference.
    pub fn as_input(&self) -> &crate::operation::search_channels::builders::SearchChannelsInputBuilder {
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
        crate::operation::search_channels::SearchChannelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_channels::SearchChannelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_channels::SearchChannels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_channels::SearchChannels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::search_channels::SearchChannelsOutput,
        crate::operation::search_channels::SearchChannelsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_channels::paginator::SearchChannelsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::search_channels::paginator::SearchChannelsPaginator {
        crate::operation::search_channels::paginator::SearchChannelsPaginator::new(self.handle, self.inner)
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_chime_bearer()
    }
    ///
    /// Appends an item to `Fields`.
    ///
    /// To override the contents of this collection use [`set_fields`](Self::set_fields).
    ///
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn fields(mut self, input: crate::types::SearchField) -> Self {
        self.inner = self.inner.fields(input);
        self
    }
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn set_fields(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SearchField>>) -> Self {
        self.inner = self.inner.set_fields(input);
        self
    }
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn get_fields(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SearchField>> {
        self.inner.get_fields()
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}