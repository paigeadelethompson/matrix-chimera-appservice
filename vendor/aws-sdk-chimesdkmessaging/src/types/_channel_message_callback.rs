// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Stores information about a callback.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ChannelMessageCallback {
    /// <p>The message ID.</p>
    pub message_id: ::std::string::String,
    /// <p>The message content. For Amazon Lex V2 bot responses, this field holds a list of messages originating from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub content: ::std::option::Option<::std::string::String>,
    /// <p>The message metadata.</p>
    pub metadata: ::std::option::Option<::std::string::String>,
    /// <p>The push notification configuration of the message.</p>
    pub push_notification: ::std::option::Option<crate::types::PushNotificationConfiguration>,
    /// <p>The attributes for the channel message. For Amazon Lex V2 bot responses, the attributes are mapped to specific fields from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    /// <p>The ID of the SubChannel.</p>
    pub sub_channel_id: ::std::option::Option<::std::string::String>,
    /// <p>The content type of the call-back message. For Amazon Lex V2 bot responses, the content type is <code>application/amz-chime-lex-msgs</code> for success responses and <code>application/amz-chime-lex-error</code> for failure responses. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub content_type: ::std::option::Option<::std::string::String>,
}
impl ChannelMessageCallback {
    /// <p>The message ID.</p>
    pub fn message_id(&self) -> &str {
        use std::ops::Deref;
        self.message_id.deref()
    }
    /// <p>The message content. For Amazon Lex V2 bot responses, this field holds a list of messages originating from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn content(&self) -> ::std::option::Option<&str> {
        self.content.as_deref()
    }
    /// <p>The message metadata.</p>
    pub fn metadata(&self) -> ::std::option::Option<&str> {
        self.metadata.as_deref()
    }
    /// <p>The push notification configuration of the message.</p>
    pub fn push_notification(&self) -> ::std::option::Option<&crate::types::PushNotificationConfiguration> {
        self.push_notification.as_ref()
    }
    /// <p>The attributes for the channel message. For Amazon Lex V2 bot responses, the attributes are mapped to specific fields from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn message_attributes(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>> {
        self.message_attributes.as_ref()
    }
    /// <p>The ID of the SubChannel.</p>
    pub fn sub_channel_id(&self) -> ::std::option::Option<&str> {
        self.sub_channel_id.as_deref()
    }
    /// <p>The content type of the call-back message. For Amazon Lex V2 bot responses, the content type is <code>application/amz-chime-lex-msgs</code> for success responses and <code>application/amz-chime-lex-error</code> for failure responses. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn content_type(&self) -> ::std::option::Option<&str> {
        self.content_type.as_deref()
    }
}
impl ::std::fmt::Debug for ChannelMessageCallback {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ChannelMessageCallback");
        formatter.field("message_id", &self.message_id);
        formatter.field("content", &"*** Sensitive Data Redacted ***");
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.field("push_notification", &self.push_notification);
        formatter.field("message_attributes", &"*** Sensitive Data Redacted ***");
        formatter.field("sub_channel_id", &self.sub_channel_id);
        formatter.field("content_type", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ChannelMessageCallback {
    /// Creates a new builder-style object to manufacture [`ChannelMessageCallback`](crate::types::ChannelMessageCallback).
    pub fn builder() -> crate::types::builders::ChannelMessageCallbackBuilder {
        crate::types::builders::ChannelMessageCallbackBuilder::default()
    }
}

/// A builder for [`ChannelMessageCallback`](crate::types::ChannelMessageCallback).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct ChannelMessageCallbackBuilder {
    pub(crate) message_id: ::std::option::Option<::std::string::String>,
    pub(crate) content: ::std::option::Option<::std::string::String>,
    pub(crate) metadata: ::std::option::Option<::std::string::String>,
    pub(crate) push_notification: ::std::option::Option<crate::types::PushNotificationConfiguration>,
    pub(crate) message_attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    pub(crate) sub_channel_id: ::std::option::Option<::std::string::String>,
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
}
impl ChannelMessageCallbackBuilder {
    /// <p>The message ID.</p>
    /// This field is required.
    pub fn message_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message ID.</p>
    pub fn set_message_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message_id = input;
        self
    }
    /// <p>The message ID.</p>
    pub fn get_message_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.message_id
    }
    /// <p>The message content. For Amazon Lex V2 bot responses, this field holds a list of messages originating from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message content. For Amazon Lex V2 bot responses, this field holds a list of messages originating from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content = input;
        self
    }
    /// <p>The message content. For Amazon Lex V2 bot responses, this field holds a list of messages originating from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn get_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.content
    }
    /// <p>The message metadata.</p>
    pub fn metadata(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metadata = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message metadata.</p>
    pub fn set_metadata(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metadata = input;
        self
    }
    /// <p>The message metadata.</p>
    pub fn get_metadata(&self) -> &::std::option::Option<::std::string::String> {
        &self.metadata
    }
    /// <p>The push notification configuration of the message.</p>
    pub fn push_notification(mut self, input: crate::types::PushNotificationConfiguration) -> Self {
        self.push_notification = ::std::option::Option::Some(input);
        self
    }
    /// <p>The push notification configuration of the message.</p>
    pub fn set_push_notification(mut self, input: ::std::option::Option<crate::types::PushNotificationConfiguration>) -> Self {
        self.push_notification = input;
        self
    }
    /// <p>The push notification configuration of the message.</p>
    pub fn get_push_notification(&self) -> &::std::option::Option<crate::types::PushNotificationConfiguration> {
        &self.push_notification
    }
    /// Adds a key-value pair to `message_attributes`.
    ///
    /// To override the contents of this collection use [`set_message_attributes`](Self::set_message_attributes).
    ///
    /// <p>The attributes for the channel message. For Amazon Lex V2 bot responses, the attributes are mapped to specific fields from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn message_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::MessageAttributeValue) -> Self {
        let mut hash_map = self.message_attributes.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.message_attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The attributes for the channel message. For Amazon Lex V2 bot responses, the attributes are mapped to specific fields from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn set_message_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    ) -> Self {
        self.message_attributes = input;
        self
    }
    /// <p>The attributes for the channel message. For Amazon Lex V2 bot responses, the attributes are mapped to specific fields from the bot. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn get_message_attributes(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>> {
        &self.message_attributes
    }
    /// <p>The ID of the SubChannel.</p>
    pub fn sub_channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sub_channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the SubChannel.</p>
    pub fn set_sub_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sub_channel_id = input;
        self
    }
    /// <p>The ID of the SubChannel.</p>
    pub fn get_sub_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.sub_channel_id
    }
    /// <p>The content type of the call-back message. For Amazon Lex V2 bot responses, the content type is <code>application/amz-chime-lex-msgs</code> for success responses and <code>application/amz-chime-lex-error</code> for failure responses. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn content_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The content type of the call-back message. For Amazon Lex V2 bot responses, the content type is <code>application/amz-chime-lex-msgs</code> for success responses and <code>application/amz-chime-lex-error</code> for failure responses. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn set_content_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_type = input;
        self
    }
    /// <p>The content type of the call-back message. For Amazon Lex V2 bot responses, the content type is <code>application/amz-chime-lex-msgs</code> for success responses and <code>application/amz-chime-lex-error</code> for failure responses. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/appinstance-bots#process-response.html">Processing responses from an AppInstanceBot</a> in the <i>Amazon Chime SDK Messaging Developer Guide</i>.</p>
    pub fn get_content_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_type
    }
    /// Consumes the builder and constructs a [`ChannelMessageCallback`](crate::types::ChannelMessageCallback).
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](crate::types::builders::ChannelMessageCallbackBuilder::message_id)
    pub fn build(self) -> ::std::result::Result<crate::types::ChannelMessageCallback, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ChannelMessageCallback {
            message_id: self.message_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "message_id",
                    "message_id was not specified but it is required when building ChannelMessageCallback",
                )
            })?,
            content: self.content,
            metadata: self.metadata,
            push_notification: self.push_notification,
            message_attributes: self.message_attributes,
            sub_channel_id: self.sub_channel_id,
            content_type: self.content_type,
        })
    }
}
impl ::std::fmt::Debug for ChannelMessageCallbackBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ChannelMessageCallbackBuilder");
        formatter.field("message_id", &self.message_id);
        formatter.field("content", &"*** Sensitive Data Redacted ***");
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.field("push_notification", &self.push_notification);
        formatter.field("message_attributes", &"*** Sensitive Data Redacted ***");
        formatter.field("sub_channel_id", &self.sub_channel_id);
        formatter.field("content_type", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}