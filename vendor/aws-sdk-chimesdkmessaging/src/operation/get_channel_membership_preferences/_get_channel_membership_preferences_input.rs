// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetChannelMembershipPreferencesInput {
    /// <p>The ARN of the channel.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The <code>AppInstanceUserArn</code> of the member retrieving the preferences.</p>
    pub member_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub chime_bearer: ::std::option::Option<::std::string::String>,
}
impl GetChannelMembershipPreferencesInput {
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member retrieving the preferences.</p>
    pub fn member_arn(&self) -> ::std::option::Option<&str> {
        self.member_arn.as_deref()
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn chime_bearer(&self) -> ::std::option::Option<&str> {
        self.chime_bearer.as_deref()
    }
}
impl GetChannelMembershipPreferencesInput {
    /// Creates a new builder-style object to manufacture [`GetChannelMembershipPreferencesInput`](crate::operation::get_channel_membership_preferences::GetChannelMembershipPreferencesInput).
    pub fn builder() -> crate::operation::get_channel_membership_preferences::builders::GetChannelMembershipPreferencesInputBuilder {
        crate::operation::get_channel_membership_preferences::builders::GetChannelMembershipPreferencesInputBuilder::default()
    }
}

/// A builder for [`GetChannelMembershipPreferencesInput`](crate::operation::get_channel_membership_preferences::GetChannelMembershipPreferencesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetChannelMembershipPreferencesInputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) member_arn: ::std::option::Option<::std::string::String>,
    pub(crate) chime_bearer: ::std::option::Option<::std::string::String>,
}
impl GetChannelMembershipPreferencesInputBuilder {
    /// <p>The ARN of the channel.</p>
    /// This field is required.
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
    /// <p>The <code>AppInstanceUserArn</code> of the member retrieving the preferences.</p>
    /// This field is required.
    pub fn member_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member retrieving the preferences.</p>
    pub fn set_member_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_arn = input;
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the member retrieving the preferences.</p>
    pub fn get_member_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.member_arn
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
    /// Consumes the builder and constructs a [`GetChannelMembershipPreferencesInput`](crate::operation::get_channel_membership_preferences::GetChannelMembershipPreferencesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_channel_membership_preferences::GetChannelMembershipPreferencesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_channel_membership_preferences::GetChannelMembershipPreferencesInput {
                channel_arn: self.channel_arn,
                member_arn: self.member_arn,
                chime_bearer: self.chime_bearer,
            },
        )
    }
}