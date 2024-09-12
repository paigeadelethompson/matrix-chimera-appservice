// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The membership information, including member ARNs, the channel ARN, and membership types.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchChannelMemberships {
    /// <p>The identifier of the member who invited another member.</p>
    pub invited_by: ::std::option::Option<crate::types::Identity>,
    /// <p>The membership types set for the channel members.</p>
    pub r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    /// <p>The users successfully added to the request.</p>
    pub members: ::std::option::Option<::std::vec::Vec<crate::types::Identity>>,
    /// <p>The ARN of the channel to which you're adding members.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the SubChannel.</p>
    pub sub_channel_id: ::std::option::Option<::std::string::String>,
}
impl BatchChannelMemberships {
    /// <p>The identifier of the member who invited another member.</p>
    pub fn invited_by(&self) -> ::std::option::Option<&crate::types::Identity> {
        self.invited_by.as_ref()
    }
    /// <p>The membership types set for the channel members.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ChannelMembershipType> {
        self.r#type.as_ref()
    }
    /// <p>The users successfully added to the request.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.members.is_none()`.
    pub fn members(&self) -> &[crate::types::Identity] {
        self.members.as_deref().unwrap_or_default()
    }
    /// <p>The ARN of the channel to which you're adding members.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The ID of the SubChannel.</p>
    pub fn sub_channel_id(&self) -> ::std::option::Option<&str> {
        self.sub_channel_id.as_deref()
    }
}
impl BatchChannelMemberships {
    /// Creates a new builder-style object to manufacture [`BatchChannelMemberships`](crate::types::BatchChannelMemberships).
    pub fn builder() -> crate::types::builders::BatchChannelMembershipsBuilder {
        crate::types::builders::BatchChannelMembershipsBuilder::default()
    }
}

/// A builder for [`BatchChannelMemberships`](crate::types::BatchChannelMemberships).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BatchChannelMembershipsBuilder {
    pub(crate) invited_by: ::std::option::Option<crate::types::Identity>,
    pub(crate) r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    pub(crate) members: ::std::option::Option<::std::vec::Vec<crate::types::Identity>>,
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) sub_channel_id: ::std::option::Option<::std::string::String>,
}
impl BatchChannelMembershipsBuilder {
    /// <p>The identifier of the member who invited another member.</p>
    pub fn invited_by(mut self, input: crate::types::Identity) -> Self {
        self.invited_by = ::std::option::Option::Some(input);
        self
    }
    /// <p>The identifier of the member who invited another member.</p>
    pub fn set_invited_by(mut self, input: ::std::option::Option<crate::types::Identity>) -> Self {
        self.invited_by = input;
        self
    }
    /// <p>The identifier of the member who invited another member.</p>
    pub fn get_invited_by(&self) -> &::std::option::Option<crate::types::Identity> {
        &self.invited_by
    }
    /// <p>The membership types set for the channel members.</p>
    pub fn r#type(mut self, input: crate::types::ChannelMembershipType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The membership types set for the channel members.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ChannelMembershipType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The membership types set for the channel members.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ChannelMembershipType> {
        &self.r#type
    }
    /// Appends an item to `members`.
    ///
    /// To override the contents of this collection use [`set_members`](Self::set_members).
    ///
    /// <p>The users successfully added to the request.</p>
    pub fn members(mut self, input: crate::types::Identity) -> Self {
        let mut v = self.members.unwrap_or_default();
        v.push(input);
        self.members = ::std::option::Option::Some(v);
        self
    }
    /// <p>The users successfully added to the request.</p>
    pub fn set_members(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Identity>>) -> Self {
        self.members = input;
        self
    }
    /// <p>The users successfully added to the request.</p>
    pub fn get_members(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Identity>> {
        &self.members
    }
    /// <p>The ARN of the channel to which you're adding members.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel to which you're adding members.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The ARN of the channel to which you're adding members.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_arn
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
    /// Consumes the builder and constructs a [`BatchChannelMemberships`](crate::types::BatchChannelMemberships).
    pub fn build(self) -> crate::types::BatchChannelMemberships {
        crate::types::BatchChannelMemberships {
            invited_by: self.invited_by,
            r#type: self.r#type,
            members: self.members,
            channel_arn: self.channel_arn,
            sub_channel_id: self.sub_channel_id,
        }
    }
}
