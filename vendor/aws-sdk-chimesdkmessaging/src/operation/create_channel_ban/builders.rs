// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_channel_ban::_create_channel_ban_output::CreateChannelBanOutputBuilder;

pub use crate::operation::create_channel_ban::_create_channel_ban_input::CreateChannelBanInputBuilder;

impl crate::operation::create_channel_ban::builders::CreateChannelBanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_channel_ban::CreateChannelBanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_channel_ban::CreateChannelBanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_channel_ban();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateChannelBan`.
///
/// <p>Permanently bans a member from a channel. Moderators can't add banned members to a channel. To undo a ban, you first have to <code>DeleteChannelBan</code>, and then <code>CreateChannelMembership</code>. Bans are cleaned up when you delete users or channels.</p>
/// <p>If you ban a user who is already part of a channel, that user is automatically kicked from the channel.</p><note>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateChannelBanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_channel_ban::builders::CreateChannelBanInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_channel_ban::CreateChannelBanOutput,
        crate::operation::create_channel_ban::CreateChannelBanError,
    > for CreateChannelBanFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_channel_ban::CreateChannelBanOutput,
            crate::operation::create_channel_ban::CreateChannelBanError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateChannelBanFluentBuilder {
    /// Creates a new `CreateChannelBanFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateChannelBan as a reference.
    pub fn as_input(&self) -> &crate::operation::create_channel_ban::builders::CreateChannelBanInputBuilder {
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
        crate::operation::create_channel_ban::CreateChannelBanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_channel_ban::CreateChannelBanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_channel_ban::CreateChannelBan::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_channel_ban::CreateChannelBan::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_channel_ban::CreateChannelBanOutput,
        crate::operation::create_channel_ban::CreateChannelBanError,
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
    /// <p>The ARN of the ban request.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_arn(input.into());
        self
    }
    /// <p>The ARN of the ban request.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_arn(input);
        self
    }
    /// <p>The ARN of the ban request.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_arn()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member being banned.</p>
    pub fn member_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.member_arn(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member being banned.</p>
    pub fn set_member_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_member_arn(input);
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member being banned.</p>
    pub fn get_member_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_member_arn()
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
