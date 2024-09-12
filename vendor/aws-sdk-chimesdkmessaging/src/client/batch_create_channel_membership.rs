// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchCreateChannelMembership`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of the channel to which you're adding users or bots.</p><br>
    ///   - [`r#type(ChannelMembershipType)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::type) / [`set_type(Option<ChannelMembershipType>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::set_type):<br>required: **false**<br><p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p><br>
    ///   - [`member_arns(impl Into<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::member_arns) / [`set_member_arns(Option<Vec::<String>>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::set_member_arns):<br>required: **true**<br><p>The ARNs of the members you want to add to the channel. Only <code>AppInstanceUsers</code> and <code>AppInstanceBots</code> can be added as a channel member.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p><br>
    ///   - [`sub_channel_id(impl Into<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::sub_channel_id) / [`set_sub_channel_id(Option<String>)`](crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::set_sub_channel_id):<br>required: **false**<br><p>The ID of the SubChannel in the request.</p><note>  <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p> </note><br>
    /// - On success, responds with [`BatchCreateChannelMembershipOutput`](crate::operation::batch_create_channel_membership::BatchCreateChannelMembershipOutput) with field(s):
    ///   - [`batch_channel_memberships(Option<BatchChannelMemberships>)`](crate::operation::batch_create_channel_membership::BatchCreateChannelMembershipOutput::batch_channel_memberships): <p>The list of channel memberships in the response.</p>
    ///   - [`errors(Option<Vec::<BatchCreateChannelMembershipError>>)`](crate::operation::batch_create_channel_membership::BatchCreateChannelMembershipOutput::errors): <p>If the action fails for one or more of the memberships in the request, a list of the memberships is returned, along with error codes and error messages.</p>
    /// - On failure, responds with [`SdkError<BatchCreateChannelMembershipError>`](crate::operation::batch_create_channel_membership::BatchCreateChannelMembershipError)
    pub fn batch_create_channel_membership(
        &self,
    ) -> crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder {
        crate::operation::batch_create_channel_membership::builders::BatchCreateChannelMembershipFluentBuilder::new(self.handle.clone())
    }
}