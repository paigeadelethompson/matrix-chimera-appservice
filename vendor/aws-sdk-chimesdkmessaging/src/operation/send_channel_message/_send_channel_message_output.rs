// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SendChannelMessageOutput {
    /// <p>The ARN of the channel.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID string assigned to each message.</p>
    pub message_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the channel message.</p>
    pub status: ::std::option::Option<crate::types::ChannelMessageStatusStructure>,
    /// <p>The ID of the SubChannel in the response.</p>
    pub sub_channel_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SendChannelMessageOutput {
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The ID string assigned to each message.</p>
    pub fn message_id(&self) -> ::std::option::Option<&str> {
        self.message_id.as_deref()
    }
    /// <p>The status of the channel message.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ChannelMessageStatusStructure> {
        self.status.as_ref()
    }
    /// <p>The ID of the SubChannel in the response.</p>
    pub fn sub_channel_id(&self) -> ::std::option::Option<&str> {
        self.sub_channel_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for SendChannelMessageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SendChannelMessageOutput {
    /// Creates a new builder-style object to manufacture [`SendChannelMessageOutput`](crate::operation::send_channel_message::SendChannelMessageOutput).
    pub fn builder() -> crate::operation::send_channel_message::builders::SendChannelMessageOutputBuilder {
        crate::operation::send_channel_message::builders::SendChannelMessageOutputBuilder::default()
    }
}

/// A builder for [`SendChannelMessageOutput`](crate::operation::send_channel_message::SendChannelMessageOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SendChannelMessageOutputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) message_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ChannelMessageStatusStructure>,
    pub(crate) sub_channel_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SendChannelMessageOutputBuilder {
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_arn
    }
    /// <p>The ID string assigned to each message.</p>
    pub fn message_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID string assigned to each message.</p>
    pub fn set_message_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_id = input;
        self
    }
    /// <p>The ID string assigned to each message.</p>
    pub fn get_message_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_id
    }
    /// <p>The status of the channel message.</p>
    pub fn status(mut self, input: crate::types::ChannelMessageStatusStructure) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the channel message.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ChannelMessageStatusStructure>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the channel message.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ChannelMessageStatusStructure> {
        &self.status
    }
    /// <p>The ID of the SubChannel in the response.</p>
    pub fn sub_channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sub_channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the SubChannel in the response.</p>
    pub fn set_sub_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sub_channel_id = input;
        self
    }
    /// <p>The ID of the SubChannel in the response.</p>
    pub fn get_sub_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.sub_channel_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SendChannelMessageOutput`](crate::operation::send_channel_message::SendChannelMessageOutput).
    pub fn build(self) -> crate::operation::send_channel_message::SendChannelMessageOutput {
        crate::operation::send_channel_message::SendChannelMessageOutput {
            channel_arn: self.channel_arn,
            message_id: self.message_id,
            status: self.status,
            sub_channel_id: self.sub_channel_id,
            _request_id: self._request_id,
        }
    }
}
