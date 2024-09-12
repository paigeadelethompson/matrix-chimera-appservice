// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateChannelMembershipInput {
    /// <p>The ARN of the channel to which you're adding users.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    pub member_arn: ::std::option::Option<::std::string::String>,
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    pub r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub chime_bearer: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the SubChannel in the request.</p><note>
    /// <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>
    /// </note>
    pub sub_channel_id: ::std::option::Option<::std::string::String>,
}
impl CreateChannelMembershipInput {
    /// <p>The ARN of the channel to which you're adding users.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    pub fn member_arn(&self) -> ::std::option::Option<&str> {
        self.member_arn.as_deref()
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ChannelMembershipType> {
        self.r#type.as_ref()
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn chime_bearer(&self) -> ::std::option::Option<&str> {
        self.chime_bearer.as_deref()
    }
    /// <p>The ID of the SubChannel in the request.</p><note>
    /// <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>
    /// </note>
    pub fn sub_channel_id(&self) -> ::std::option::Option<&str> {
        self.sub_channel_id.as_deref()
    }
}
impl CreateChannelMembershipInput {
    /// Creates a new builder-style object to manufacture [`CreateChannelMembershipInput`](crate::operation::create_channel_membership::CreateChannelMembershipInput).
    pub fn builder() -> crate::operation::create_channel_membership::builders::CreateChannelMembershipInputBuilder {
        crate::operation::create_channel_membership::builders::CreateChannelMembershipInputBuilder::default()
    }
}

/// A builder for [`CreateChannelMembershipInput`](crate::operation::create_channel_membership::CreateChannelMembershipInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateChannelMembershipInputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) member_arn: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    pub(crate) chime_bearer: ::std::option::Option<::std::string::String>,
    pub(crate) sub_channel_id: ::std::option::Option<::std::string::String>,
}
impl CreateChannelMembershipInputBuilder {
    /// <p>The ARN of the channel to which you're adding users.</p>
    /// This field is required.
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel to which you're adding users.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The ARN of the channel to which you're adding users.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_arn
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    /// This field is required.
    pub fn member_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    pub fn set_member_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_arn = input;
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    pub fn get_member_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.member_arn
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    /// This field is required.
    pub fn r#type(mut self, input: crate::types::ChannelMembershipType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ChannelMembershipType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ChannelMembershipType> {
        &self.r#type
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    /// This field is required.
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.chime_bearer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.chime_bearer = input;
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        &self.chime_bearer
    }
    /// <p>The ID of the SubChannel in the request.</p><note>
    /// <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>
    /// </note>
    pub fn sub_channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sub_channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the SubChannel in the request.</p><note>
    /// <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>
    /// </note>
    pub fn set_sub_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sub_channel_id = input;
        self
    }
    /// <p>The ID of the SubChannel in the request.</p><note>
    /// <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>
    /// </note>
    pub fn get_sub_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.sub_channel_id
    }
    /// Consumes the builder and constructs a [`CreateChannelMembershipInput`](crate::operation::create_channel_membership::CreateChannelMembershipInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_channel_membership::CreateChannelMembershipInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_channel_membership::CreateChannelMembershipInput {
            channel_arn: self.channel_arn,
            member_arn: self.member_arn,
            r#type: self.r#type,
            chime_bearer: self.chime_bearer,
            sub_channel_id: self.sub_channel_id,
        })
    }
}
