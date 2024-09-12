// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_exports::_list_exports_output::ListExportsOutputBuilder;

pub use crate::operation::list_exports::_list_exports_input::ListExportsInputBuilder;

impl crate::operation::list_exports::builders::ListExportsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_exports::ListExportsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_exports::ListExportsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_exports();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListExports`.
///
/// <p>Lists completed exports within the past 90 days.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListExportsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_exports::builders::ListExportsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_exports::ListExportsOutput,
        crate::operation::list_exports::ListExportsError,
    > for ListExportsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_exports::ListExportsOutput,
            crate::operation::list_exports::ListExportsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListExportsFluentBuilder {
    /// Creates a new `ListExportsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListExports as a reference.
    pub fn as_input(&self) -> &crate::operation::list_exports::builders::ListExportsInputBuilder {
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
        crate::operation::list_exports::ListExportsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_exports::ListExportsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_exports::ListExports::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_exports::ListExports::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_exports::ListExportsOutput,
        crate::operation::list_exports::ListExportsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_exports::paginator::ListExportsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_exports::paginator::ListExportsPaginator {
        crate::operation::list_exports::paginator::ListExportsPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the exported table.</p>
    pub fn get_table_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_arn()
    }
    /// <p>Maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Maximum number of results to return per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to <code>ListExports</code>. When provided in this manner, the API fetches the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
