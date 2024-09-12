// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendChannelMessage`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of the channel.</p><br>
    ///   - [`content(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::content) / [`set_content(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_content):<br>required: **true**<br><p>The content of the channel message.</p><br>
    ///   - [`r#type(ChannelMessageType)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::type) / [`set_type(Option<ChannelMessageType>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_type):<br>required: **true**<br><p>The type of message, <code>STANDARD</code> or <code>CONTROL</code>.</p> <p><code>STANDARD</code> messages can be up to 4KB in size and contain metadata. Metadata is arbitrary, and you can use it in a variety of ways, such as containing a link to an attachment.</p> <p><code>CONTROL</code> messages are limited to 30 bytes and do not contain metadata.</p><br>
    ///   - [`persistence(ChannelMessagePersistenceType)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::persistence) / [`set_persistence(Option<ChannelMessagePersistenceType>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_persistence):<br>required: **true**<br><p>Boolean that controls whether the message is persisted on the back end. Required.</p><br>
    ///   - [`metadata(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::metadata) / [`set_metadata(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_metadata):<br>required: **false**<br><p>The optional metadata for each message.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_client_request_token):<br>required: **true**<br><p>The <code>Idempotency</code> token for each client request.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p><br>
    ///   - [`push_notification(PushNotificationConfiguration)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::push_notification) / [`set_push_notification(Option<PushNotificationConfiguration>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_push_notification):<br>required: **false**<br><p>The push notification configuration of the message.</p><br>
    ///   - [`message_attributes(impl Into<String>, MessageAttributeValue)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::message_attributes) / [`set_message_attributes(Option<HashMap::<String, MessageAttributeValue>>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_message_attributes):<br>required: **false**<br><p>The attributes for the message, used for message filtering along with a <code>FilterRule</code> defined in the <code>PushNotificationPreferences</code>.</p><br>
    ///   - [`sub_channel_id(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::sub_channel_id) / [`set_sub_channel_id(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_sub_channel_id):<br>required: **false**<br><p>The ID of the SubChannel in the request.</p><br>
    ///   - [`content_type(impl Into<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::content_type) / [`set_content_type(Option<String>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_content_type):<br>required: **false**<br><p>The content type of the channel message.</p><br>
    ///   - [`target(Target)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::target) / [`set_target(Option<Vec::<Target>>)`](crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::set_target):<br>required: **false**<br><p>The target of a message. Must be a member of the channel, such as another user, a bot, or the sender. Only the target and the sender can view targeted messages. Only users who can see targeted messages can take actions on them. However, administrators can delete targeted messages that they can’t see.</p><br>
    /// - On success, responds with [`SendChannelMessageOutput`](crate::operation::send_channel_message::SendChannelMessageOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::operation::send_channel_message::SendChannelMessageOutput::channel_arn): <p>The ARN of the channel.</p>
    ///   - [`message_id(Option<String>)`](crate::operation::send_channel_message::SendChannelMessageOutput::message_id): <p>The ID string assigned to each message.</p>
    ///   - [`status(Option<ChannelMessageStatusStructure>)`](crate::operation::send_channel_message::SendChannelMessageOutput::status): <p>The status of the channel message.</p>
    ///   - [`sub_channel_id(Option<String>)`](crate::operation::send_channel_message::SendChannelMessageOutput::sub_channel_id): <p>The ID of the SubChannel in the response.</p>
    /// - On failure, responds with [`SdkError<SendChannelMessageError>`](crate::operation::send_channel_message::SendChannelMessageError)
    pub fn send_channel_message(&self) -> crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder {
        crate::operation::send_channel_message::builders::SendChannelMessageFluentBuilder::new(self.handle.clone())
    }
}