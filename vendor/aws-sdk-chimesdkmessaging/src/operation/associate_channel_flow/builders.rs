// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_channel_flow::_associate_channel_flow_output::AssociateChannelFlowOutputBuilder;

pub use crate::operation::associate_channel_flow::_associate_channel_flow_input::AssociateChannelFlowInputBuilder;

impl crate::operation::associate_channel_flow::builders::AssociateChannelFlowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_channel_flow::AssociateChannelFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_channel_flow::AssociateChannelFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_channel_flow();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateChannelFlow`.
///
/// <p>Associates a channel flow with a channel. Once associated, all messages to that channel go through channel flow processors. To stop processing, use the <code>DisassociateChannelFlow</code> API.</p><note>
/// <p>Only administrators or channel moderators can associate a channel flow. The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateChannelFlowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_channel_flow::builders::AssociateChannelFlowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_channel_flow::AssociateChannelFlowOutput,
        crate::operation::associate_channel_flow::AssociateChannelFlowError,
    > for AssociateChannelFlowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_channel_flow::AssociateChannelFlowOutput,
            crate::operation::associate_channel_flow::AssociateChannelFlowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateChannelFlowFluentBuilder {
    /// Creates a new `AssociateChannelFlowFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateChannelFlow as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_channel_flow::builders::AssociateChannelFlowInputBuilder {
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
        crate::operation::associate_channel_flow::AssociateChannelFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_channel_flow::AssociateChannelFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_channel_flow::AssociateChannelFlow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_channel_flow::AssociateChannelFlow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_channel_flow::AssociateChannelFlowOutput,
        crate::operation::associate_channel_flow::AssociateChannelFlowError,
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
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_arn(input.into());
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_arn(input);
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_arn()
    }
    /// <p>The ARN of the channel flow.</p>
    pub fn channel_flow_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_flow_arn(input.into());
        self
    }
    /// <p>The ARN of the channel flow.</p>
    pub fn set_channel_flow_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_flow_arn(input);
        self
    }
    /// <p>The ARN of the channel flow.</p>
    pub fn get_channel_flow_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_flow_arn()
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
}