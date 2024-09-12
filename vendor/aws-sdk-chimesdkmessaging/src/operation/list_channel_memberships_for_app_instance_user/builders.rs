// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_channel_memberships_for_app_instance_user::_list_channel_memberships_for_app_instance_user_output::ListChannelMembershipsForAppInstanceUserOutputBuilder;

pub use crate::operation::list_channel_memberships_for_app_instance_user::_list_channel_memberships_for_app_instance_user_input::ListChannelMembershipsForAppInstanceUserInputBuilder;

impl crate::operation::list_channel_memberships_for_app_instance_user::builders::ListChannelMembershipsForAppInstanceUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_channel_memberships_for_app_instance_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListChannelMembershipsForAppInstanceUser`.
///
/// <p>Lists all channels that an <code>AppInstanceUser</code> or <code>AppInstanceBot</code> is a part of. Only an <code>AppInstanceAdmin</code> can call the API with a user ARN that is not their own.</p><note>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListChannelMembershipsForAppInstanceUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_channel_memberships_for_app_instance_user::builders::ListChannelMembershipsForAppInstanceUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserOutput,
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserError,
    > for ListChannelMembershipsForAppInstanceUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserOutput,
            crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListChannelMembershipsForAppInstanceUserFluentBuilder {
    /// Creates a new `ListChannelMembershipsForAppInstanceUserFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListChannelMembershipsForAppInstanceUser as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::list_channel_memberships_for_app_instance_user::builders::ListChannelMembershipsForAppInstanceUserInputBuilder {
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
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUser::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUser::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserOutput,
        crate::operation::list_channel_memberships_for_app_instance_user::ListChannelMembershipsForAppInstanceUserError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_channel_memberships_for_app_instance_user::paginator::ListChannelMembershipsForAppInstanceUserPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_channel_memberships_for_app_instance_user::paginator::ListChannelMembershipsForAppInstanceUserPaginator {
        crate::operation::list_channel_memberships_for_app_instance_user::paginator::ListChannelMembershipsForAppInstanceUserPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ARN of the user or bot.</p>
    pub fn app_instance_user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_user_arn(input.into());
        self
    }
    /// <p>The ARN of the user or bot.</p>
    pub fn set_app_instance_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_user_arn(input);
        self
    }
    /// <p>The ARN of the user or bot.</p>
    pub fn get_app_instance_user_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_user_arn()
    }
    /// <p>The maximum number of users that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of users that you want returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of users that you want returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token returned from previous API requests until the number of channel memberships is reached.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from previous API requests until the number of channel memberships is reached.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned from previous API requests until the number of channel memberships is reached.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_chime_bearer()
    }
}
