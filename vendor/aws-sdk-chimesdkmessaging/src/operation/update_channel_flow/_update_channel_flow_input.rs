// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct UpdateChannelFlowInput {
    /// <p>The ARN of the channel flow.</p>
    pub channel_flow_arn: ::std::option::Option<::std::string::String>,
    /// <p>Information about the processor Lambda functions</p>
    pub processors: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    /// <p>The name of the channel flow.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl UpdateChannelFlowInput {
    /// <p>The ARN of the channel flow.</p>
    pub fn channel_flow_arn(&self) -> ::std::option::Option<&str> {
        self.channel_flow_arn.as_deref()
    }
    /// <p>Information about the processor Lambda functions</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.processors.is_none()`.
    pub fn processors(&self) -> &[crate::types::Processor] {
        self.processors.as_deref().unwrap_or_default()
    }
    /// <p>The name of the channel flow.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ::std::fmt::Debug for UpdateChannelFlowInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateChannelFlowInput");
        formatter.field("channel_flow_arn", &self.channel_flow_arn);
        formatter.field("processors", &self.processors);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl UpdateChannelFlowInput {
    /// Creates a new builder-style object to manufacture [`UpdateChannelFlowInput`](crate::operation::update_channel_flow::UpdateChannelFlowInput).
    pub fn builder() -> crate::operation::update_channel_flow::builders::UpdateChannelFlowInputBuilder {
        crate::operation::update_channel_flow::builders::UpdateChannelFlowInputBuilder::default()
    }
}

/// A builder for [`UpdateChannelFlowInput`](crate::operation::update_channel_flow::UpdateChannelFlowInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct UpdateChannelFlowInputBuilder {
    pub(crate) channel_flow_arn: ::std::option::Option<::std::string::String>,
    pub(crate) processors: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl UpdateChannelFlowInputBuilder {
    /// <p>The ARN of the channel flow.</p>
    /// This field is required.
    pub fn channel_flow_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_flow_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel flow.</p>
    pub fn set_channel_flow_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_flow_arn = input;
        self
    }
    /// <p>The ARN of the channel flow.</p>
    pub fn get_channel_flow_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_flow_arn
    }
    /// Appends an item to `processors`.
    ///
    /// To override the contents of this collection use [`set_processors`](Self::set_processors).
    ///
    /// <p>Information about the processor Lambda functions</p>
    pub fn processors(mut self, input: crate::types::Processor) -> Self {
        let mut v = self.processors.unwrap_or_default();
        v.push(input);
        self.processors = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the processor Lambda functions</p>
    pub fn set_processors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Processor>>) -> Self {
        self.processors = input;
        self
    }
    /// <p>Information about the processor Lambda functions</p>
    pub fn get_processors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Processor>> {
        &self.processors
    }
    /// <p>The name of the channel flow.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the channel flow.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the channel flow.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`UpdateChannelFlowInput`](crate::operation::update_channel_flow::UpdateChannelFlowInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_channel_flow::UpdateChannelFlowInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_channel_flow::UpdateChannelFlowInput {
            channel_flow_arn: self.channel_flow_arn,
            processors: self.processors,
            name: self.name,
        })
    }
}
impl ::std::fmt::Debug for UpdateChannelFlowInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateChannelFlowInputBuilder");
        formatter.field("channel_flow_arn", &self.channel_flow_arn);
        formatter.field("processors", &self.processors);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}