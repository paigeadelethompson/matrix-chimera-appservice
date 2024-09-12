// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteChannelMembership`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of the channel from which you want to remove the user.</p><br>
    ///   - [`member_arn(impl Into<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::member_arn) / [`set_member_arn(Option<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::set_member_arn):<br>required: **true**<br><p>The <code>AppInstanceUserArn</code> of the member that you're removing from the channel.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p><br>
    ///   - [`sub_channel_id(impl Into<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::sub_channel_id) / [`set_sub_channel_id(Option<String>)`](crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::set_sub_channel_id):<br>required: **false**<br><p>The ID of the SubChannel in the request.</p><note>  <p>Only for use by moderators.</p> </note><br>
    /// - On success, responds with [`DeleteChannelMembershipOutput`](crate::operation::delete_channel_membership::DeleteChannelMembershipOutput)
    /// - On failure, responds with [`SdkError<DeleteChannelMembershipError>`](crate::operation::delete_channel_membership::DeleteChannelMembershipError)
    pub fn delete_channel_membership(&self) -> crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder {
        crate::operation::delete_channel_membership::builders::DeleteChannelMembershipFluentBuilder::new(self.handle.clone())
    }
}
