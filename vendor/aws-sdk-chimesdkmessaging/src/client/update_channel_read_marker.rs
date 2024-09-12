// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateChannelReadMarker`](crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of the channel.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p><br>
    /// - On success, responds with [`UpdateChannelReadMarkerOutput`](crate::operation::update_channel_read_marker::UpdateChannelReadMarkerOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::operation::update_channel_read_marker::UpdateChannelReadMarkerOutput::channel_arn): <p>The ARN of the channel.</p>
    /// - On failure, responds with [`SdkError<UpdateChannelReadMarkerError>`](crate::operation::update_channel_read_marker::UpdateChannelReadMarkerError)
    pub fn update_channel_read_marker(&self) -> crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder {
        crate::operation::update_channel_read_marker::builders::UpdateChannelReadMarkerFluentBuilder::new(self.handle.clone())
    }
}