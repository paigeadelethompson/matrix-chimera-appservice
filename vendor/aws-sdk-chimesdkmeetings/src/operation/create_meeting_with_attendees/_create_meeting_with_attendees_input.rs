// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateMeetingWithAttendeesInput {
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub client_request_token: ::std::option::Option<::std::string::String>,
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub media_region: ::std::option::Option<::std::string::String>,
    /// <p>Reserved.</p>
    pub meeting_host_id: ::std::option::Option<::std::string::String>,
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub external_meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub meeting_features: ::std::option::Option<crate::types::MeetingFeaturesConfiguration>,
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub notifications_configuration: ::std::option::Option<crate::types::NotificationsConfiguration>,
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub attendees: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>,
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub primary_meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub tenant_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The tags in the request.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateMeetingWithAttendeesInput {
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub fn media_region(&self) -> ::std::option::Option<&str> {
        self.media_region.as_deref()
    }
    /// <p>Reserved.</p>
    pub fn meeting_host_id(&self) -> ::std::option::Option<&str> {
        self.meeting_host_id.as_deref()
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub fn external_meeting_id(&self) -> ::std::option::Option<&str> {
        self.external_meeting_id.as_deref()
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn meeting_features(&self) -> ::std::option::Option<&crate::types::MeetingFeaturesConfiguration> {
        self.meeting_features.as_ref()
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn notifications_configuration(&self) -> ::std::option::Option<&crate::types::NotificationsConfiguration> {
        self.notifications_configuration.as_ref()
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.attendees.is_none()`.
    pub fn attendees(&self) -> &[crate::types::CreateAttendeeRequestItem] {
        self.attendees.as_deref().unwrap_or_default()
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn primary_meeting_id(&self) -> ::std::option::Option<&str> {
        self.primary_meeting_id.as_deref()
    }
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tenant_ids.is_none()`.
    pub fn tenant_ids(&self) -> &[::std::string::String] {
        self.tenant_ids.as_deref().unwrap_or_default()
    }
    /// <p>The tags in the request.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for CreateMeetingWithAttendeesInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMeetingWithAttendeesInput");
        formatter.field("client_request_token", &"*** Sensitive Data Redacted ***");
        formatter.field("media_region", &self.media_region);
        formatter.field("meeting_host_id", &"*** Sensitive Data Redacted ***");
        formatter.field("external_meeting_id", &"*** Sensitive Data Redacted ***");
        formatter.field("meeting_features", &self.meeting_features);
        formatter.field("notifications_configuration", &self.notifications_configuration);
        formatter.field("attendees", &self.attendees);
        formatter.field("primary_meeting_id", &self.primary_meeting_id);
        formatter.field("tenant_ids", &self.tenant_ids);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
impl CreateMeetingWithAttendeesInput {
    /// Creates a new builder-style object to manufacture [`CreateMeetingWithAttendeesInput`](crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesInput).
    pub fn builder() -> crate::operation::create_meeting_with_attendees::builders::CreateMeetingWithAttendeesInputBuilder {
        crate::operation::create_meeting_with_attendees::builders::CreateMeetingWithAttendeesInputBuilder::default()
    }
}

/// A builder for [`CreateMeetingWithAttendeesInput`](crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CreateMeetingWithAttendeesInputBuilder {
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) media_region: ::std::option::Option<::std::string::String>,
    pub(crate) meeting_host_id: ::std::option::Option<::std::string::String>,
    pub(crate) external_meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) meeting_features: ::std::option::Option<crate::types::MeetingFeaturesConfiguration>,
    pub(crate) notifications_configuration: ::std::option::Option<crate::types::NotificationsConfiguration>,
    pub(crate) attendees: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>,
    pub(crate) primary_meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) tenant_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateMeetingWithAttendeesInputBuilder {
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    /// This field is required.
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_request_token = input;
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_request_token
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    /// This field is required.
    pub fn media_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.media_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub fn set_media_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.media_region = input;
        self
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub fn get_media_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.media_region
    }
    /// <p>Reserved.</p>
    pub fn meeting_host_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.meeting_host_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Reserved.</p>
    pub fn set_meeting_host_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.meeting_host_id = input;
        self
    }
    /// <p>Reserved.</p>
    pub fn get_meeting_host_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.meeting_host_id
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    /// This field is required.
    pub fn external_meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.external_meeting_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub fn set_external_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.external_meeting_id = input;
        self
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub fn get_external_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.external_meeting_id
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn meeting_features(mut self, input: crate::types::MeetingFeaturesConfiguration) -> Self {
        self.meeting_features = ::std::option::Option::Some(input);
        self
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn set_meeting_features(mut self, input: ::std::option::Option<crate::types::MeetingFeaturesConfiguration>) -> Self {
        self.meeting_features = input;
        self
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn get_meeting_features(&self) -> &::std::option::Option<crate::types::MeetingFeaturesConfiguration> {
        &self.meeting_features
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn notifications_configuration(mut self, input: crate::types::NotificationsConfiguration) -> Self {
        self.notifications_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn set_notifications_configuration(mut self, input: ::std::option::Option<crate::types::NotificationsConfiguration>) -> Self {
        self.notifications_configuration = input;
        self
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn get_notifications_configuration(&self) -> &::std::option::Option<crate::types::NotificationsConfiguration> {
        &self.notifications_configuration
    }
    /// Appends an item to `attendees`.
    ///
    /// To override the contents of this collection use [`set_attendees`](Self::set_attendees).
    ///
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn attendees(mut self, input: crate::types::CreateAttendeeRequestItem) -> Self {
        let mut v = self.attendees.unwrap_or_default();
        v.push(input);
        self.attendees = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn set_attendees(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>) -> Self {
        self.attendees = input;
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn get_attendees(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>> {
        &self.attendees
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn primary_meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.primary_meeting_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn set_primary_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.primary_meeting_id = input;
        self
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn get_primary_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.primary_meeting_id
    }
    /// Appends an item to `tenant_ids`.
    ///
    /// To override the contents of this collection use [`set_tenant_ids`](Self::set_tenant_ids).
    ///
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub fn tenant_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.tenant_ids.unwrap_or_default();
        v.push(input.into());
        self.tenant_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub fn set_tenant_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.tenant_ids = input;
        self
    }
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub fn get_tenant_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.tenant_ids
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags in the request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags in the request.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags in the request.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateMeetingWithAttendeesInput`](crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesInput {
            client_request_token: self.client_request_token,
            media_region: self.media_region,
            meeting_host_id: self.meeting_host_id,
            external_meeting_id: self.external_meeting_id,
            meeting_features: self.meeting_features,
            notifications_configuration: self.notifications_configuration,
            attendees: self.attendees,
            primary_meeting_id: self.primary_meeting_id,
            tenant_ids: self.tenant_ids,
            tags: self.tags,
        })
    }
}
impl ::std::fmt::Debug for CreateMeetingWithAttendeesInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMeetingWithAttendeesInputBuilder");
        formatter.field("client_request_token", &"*** Sensitive Data Redacted ***");
        formatter.field("media_region", &self.media_region);
        formatter.field("meeting_host_id", &"*** Sensitive Data Redacted ***");
        formatter.field("external_meeting_id", &"*** Sensitive Data Redacted ***");
        formatter.field("meeting_features", &self.meeting_features);
        formatter.field("notifications_configuration", &self.notifications_configuration);
        formatter.field("attendees", &self.attendees);
        formatter.field("primary_meeting_id", &self.primary_meeting_id);
        formatter.field("tenant_ids", &self.tenant_ids);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}