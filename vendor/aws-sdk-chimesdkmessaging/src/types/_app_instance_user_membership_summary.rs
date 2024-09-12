// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary of the membership details of an <code>AppInstanceUser</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AppInstanceUserMembershipSummary {
    /// <p>The type of <code>ChannelMembership</code>.</p>
    pub r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    /// <p>The time at which an <code>AppInstanceUser</code> last marked a channel as read.</p>
    pub read_marker_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The ID of the SubChannel that the <code>AppInstanceUser</code> is a member of.</p>
    pub sub_channel_id: ::std::option::Option<::std::string::String>,
}
impl AppInstanceUserMembershipSummary {
    /// <p>The type of <code>ChannelMembership</code>.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ChannelMembershipType> {
        self.r#type.as_ref()
    }
    /// <p>The time at which an <code>AppInstanceUser</code> last marked a channel as read.</p>
    pub fn read_marker_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.read_marker_timestamp.as_ref()
    }
    /// <p>The ID of the SubChannel that the <code>AppInstanceUser</code> is a member of.</p>
    pub fn sub_channel_id(&self) -> ::std::option::Option<&str> {
        self.sub_channel_id.as_deref()
    }
}
impl AppInstanceUserMembershipSummary {
    /// Creates a new builder-style object to manufacture [`AppInstanceUserMembershipSummary`](crate::types::AppInstanceUserMembershipSummary).
    pub fn builder() -> crate::types::builders::AppInstanceUserMembershipSummaryBuilder {
        crate::types::builders::AppInstanceUserMembershipSummaryBuilder::default()
    }
}

/// A builder for [`AppInstanceUserMembershipSummary`](crate::types::AppInstanceUserMembershipSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AppInstanceUserMembershipSummaryBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::ChannelMembershipType>,
    pub(crate) read_marker_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) sub_channel_id: ::std::option::Option<::std::string::String>,
}
impl AppInstanceUserMembershipSummaryBuilder {
    /// <p>The type of <code>ChannelMembership</code>.</p>
    pub fn r#type(mut self, input: crate::types::ChannelMembershipType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of <code>ChannelMembership</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ChannelMembershipType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of <code>ChannelMembership</code>.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ChannelMembershipType> {
        &self.r#type
    }
    /// <p>The time at which an <code>AppInstanceUser</code> last marked a channel as read.</p>
    pub fn read_marker_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.read_marker_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which an <code>AppInstanceUser</code> last marked a channel as read.</p>
    pub fn set_read_marker_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.read_marker_timestamp = input;
        self
    }
    /// <p>The time at which an <code>AppInstanceUser</code> last marked a channel as read.</p>
    pub fn get_read_marker_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.read_marker_timestamp
    }
    /// <p>The ID of the SubChannel that the <code>AppInstanceUser</code> is a member of.</p>
    pub fn sub_channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sub_channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the SubChannel that the <code>AppInstanceUser</code> is a member of.</p>
    pub fn set_sub_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sub_channel_id = input;
        self
    }
    /// <p>The ID of the SubChannel that the <code>AppInstanceUser</code> is a member of.</p>
    pub fn get_sub_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.sub_channel_id
    }
    /// Consumes the builder and constructs a [`AppInstanceUserMembershipSummary`](crate::types::AppInstanceUserMembershipSummary).
    pub fn build(self) -> crate::types::AppInstanceUserMembershipSummary {
        crate::types::AppInstanceUserMembershipSummary {
            r#type: self.r#type,
            read_marker_timestamp: self.read_marker_timestamp,
            sub_channel_id: self.sub_channel_id,
        }
    }
}