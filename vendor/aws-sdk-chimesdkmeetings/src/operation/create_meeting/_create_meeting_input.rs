// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateMeetingInput {
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
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub notifications_configuration: ::std::option::Option<crate::types::NotificationsConfiguration>,
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub meeting_features: ::std::option::Option<crate::types::MeetingFeaturesConfiguration>,
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub primary_meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub tenant_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Applies one or more tags to an Amazon Chime SDK meeting. Note the following:</p>
    /// <ul>
    /// <li>
    /// <p>Not all resources have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>. If the resource doesn't yet support this operation, the resource's service might support tagging using its own API operations. For more information, refer to the documentation for that service.</p></li>
    /// <li>
    /// <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference</i>.</p></li>
    /// <li>
    /// <p>You can only tag resources that are located in the specified Amazon Web Services Region for the Amazon Web Services account.</p></li>
    /// <li>
    /// <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p></li>
    /// </ul><important>
    /// <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p>
    /// </important>
    /// <p><b>Minimum permissions</b></p>
    /// <p>In addition to the <code>tag:TagResources</code> permission required by this operation, you must also have the tagging permission defined by the service that created the resource. For example, to tag a <code>ChimeSDKMeetings</code> instance using the <code>TagResources</code> operation, you must have both of the following permissions:</p>
    /// <p><code>tag:TagResources</code></p>
    /// <p><code>ChimeSDKMeetings:CreateTags</code></p><note>
    /// <p>Some services might have specific requirements for tagging some resources. For example, to tag an Amazon S3 bucket, you must also have the <code>s3:GetBucketTagging</code> permission. If the expected minimum permissions don't work, check the documentation for that service's tagging APIs for more information.</p>
    /// </note>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateMeetingInput {
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
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn notifications_configuration(&self) -> ::std::option::Option<&crate::types::NotificationsConfiguration> {
        self.notifications_configuration.as_ref()
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn meeting_features(&self) -> ::std::option::Option<&crate::types::MeetingFeaturesConfiguration> {
        self.meeting_features.as_ref()
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
    /// <p>Applies one or more tags to an Amazon Chime SDK meeting. Note the following:</p>
    /// <ul>
    /// <li>
    /// <p>Not all resources have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>. If the resource doesn't yet support this operation, the resource's service might support tagging using its own API operations. For more information, refer to the documentation for that service.</p></li>
    /// <li>
    /// <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference</i>.</p></li>
    /// <li>
    /// <p>You can only tag resources that are located in the specified Amazon Web Services Region for the Amazon Web Services account.</p></li>
    /// <li>
    /// <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p></li>
    /// </ul><important>
    /// <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p>
    /// </important>
    /// <p><b>Minimum permissions</b></p>
    /// <p>In addition to the <code>tag:TagResources</code> permission required by this operation, you must also have the tagging permission defined by the service that created the resource. For example, to tag a <code>ChimeSDKMeetings</code> instance using the <code>TagResources</code> operation, you must have both of the following permissions:</p>
    /// <p><code>tag:TagResources</code></p>
    /// <p><code>ChimeSDKMeetings:CreateTags</code></p><note>
    /// <p>Some services might have specific requirements for tagging some resources. For example, to tag an Amazon S3 bucket, you must also have the <code>s3:GetBucketTagging</code> permission. If the expected minimum permissions don't work, check the documentation for that service's tagging APIs for more information.</p>
    /// </note>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for CreateMeetingInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMeetingInput");
        formatter.field("client_request_token", &"*** Sensitive Data Redacted ***");
        formatter.field("media_region", &self.media_region);
        formatter.field("meeting_host_id", &"*** Sensitive Data Redacted ***");
        formatter.field("external_meeting_id", &"*** Sensitive Data Redacted ***");
        formatter.field("notifications_configuration", &self.notifications_configuration);
        formatter.field("meeting_features", &self.meeting_features);
        formatter.field("primary_meeting_id", &self.primary_meeting_id);
        formatter.field("tenant_ids", &self.tenant_ids);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
impl CreateMeetingInput {
    /// Creates a new builder-style object to manufacture [`CreateMeetingInput`](crate::operation::create_meeting::CreateMeetingInput).
    pub fn builder() -> crate::operation::create_meeting::builders::CreateMeetingInputBuilder {
        crate::operation::create_meeting::builders::CreateMeetingInputBuilder::default()
    }
}

/// A builder for [`CreateMeetingInput`](crate::operation::create_meeting::CreateMeetingInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CreateMeetingInputBuilder {
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) media_region: ::std::option::Option<::std::string::String>,
    pub(crate) meeting_host_id: ::std::option::Option<::std::string::String>,
    pub(crate) external_meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) notifications_configuration: ::std::option::Option<crate::types::NotificationsConfiguration>,
    pub(crate) meeting_features: ::std::option::Option<crate::types::MeetingFeaturesConfiguration>,
    pub(crate) primary_meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) tenant_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateMeetingInputBuilder {
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
    /// <p>Applies one or more tags to an Amazon Chime SDK meeting. Note the following:</p>
    /// <ul>
    /// <li>
    /// <p>Not all resources have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>. If the resource doesn't yet support this operation, the resource's service might support tagging using its own API operations. For more information, refer to the documentation for that service.</p></li>
    /// <li>
    /// <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference</i>.</p></li>
    /// <li>
    /// <p>You can only tag resources that are located in the specified Amazon Web Services Region for the Amazon Web Services account.</p></li>
    /// <li>
    /// <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p></li>
    /// </ul><important>
    /// <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p>
    /// </important>
    /// <p><b>Minimum permissions</b></p>
    /// <p>In addition to the <code>tag:TagResources</code> permission required by this operation, you must also have the tagging permission defined by the service that created the resource. For example, to tag a <code>ChimeSDKMeetings</code> instance using the <code>TagResources</code> operation, you must have both of the following permissions:</p>
    /// <p><code>tag:TagResources</code></p>
    /// <p><code>ChimeSDKMeetings:CreateTags</code></p><note>
    /// <p>Some services might have specific requirements for tagging some resources. For example, to tag an Amazon S3 bucket, you must also have the <code>s3:GetBucketTagging</code> permission. If the expected minimum permissions don't work, check the documentation for that service's tagging APIs for more information.</p>
    /// </note>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Applies one or more tags to an Amazon Chime SDK meeting. Note the following:</p>
    /// <ul>
    /// <li>
    /// <p>Not all resources have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>. If the resource doesn't yet support this operation, the resource's service might support tagging using its own API operations. For more information, refer to the documentation for that service.</p></li>
    /// <li>
    /// <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference</i>.</p></li>
    /// <li>
    /// <p>You can only tag resources that are located in the specified Amazon Web Services Region for the Amazon Web Services account.</p></li>
    /// <li>
    /// <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p></li>
    /// </ul><important>
    /// <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p>
    /// </important>
    /// <p><b>Minimum permissions</b></p>
    /// <p>In addition to the <code>tag:TagResources</code> permission required by this operation, you must also have the tagging permission defined by the service that created the resource. For example, to tag a <code>ChimeSDKMeetings</code> instance using the <code>TagResources</code> operation, you must have both of the following permissions:</p>
    /// <p><code>tag:TagResources</code></p>
    /// <p><code>ChimeSDKMeetings:CreateTags</code></p><note>
    /// <p>Some services might have specific requirements for tagging some resources. For example, to tag an Amazon S3 bucket, you must also have the <code>s3:GetBucketTagging</code> permission. If the expected minimum permissions don't work, check the documentation for that service's tagging APIs for more information.</p>
    /// </note>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Applies one or more tags to an Amazon Chime SDK meeting. Note the following:</p>
    /// <ul>
    /// <li>
    /// <p>Not all resources have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>. If the resource doesn't yet support this operation, the resource's service might support tagging using its own API operations. For more information, refer to the documentation for that service.</p></li>
    /// <li>
    /// <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference</i>.</p></li>
    /// <li>
    /// <p>You can only tag resources that are located in the specified Amazon Web Services Region for the Amazon Web Services account.</p></li>
    /// <li>
    /// <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p></li>
    /// </ul><important>
    /// <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p>
    /// </important>
    /// <p><b>Minimum permissions</b></p>
    /// <p>In addition to the <code>tag:TagResources</code> permission required by this operation, you must also have the tagging permission defined by the service that created the resource. For example, to tag a <code>ChimeSDKMeetings</code> instance using the <code>TagResources</code> operation, you must have both of the following permissions:</p>
    /// <p><code>tag:TagResources</code></p>
    /// <p><code>ChimeSDKMeetings:CreateTags</code></p><note>
    /// <p>Some services might have specific requirements for tagging some resources. For example, to tag an Amazon S3 bucket, you must also have the <code>s3:GetBucketTagging</code> permission. If the expected minimum permissions don't work, check the documentation for that service's tagging APIs for more information.</p>
    /// </note>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateMeetingInput`](crate::operation::create_meeting::CreateMeetingInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_meeting::CreateMeetingInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_meeting::CreateMeetingInput {
            client_request_token: self.client_request_token,
            media_region: self.media_region,
            meeting_host_id: self.meeting_host_id,
            external_meeting_id: self.external_meeting_id,
            notifications_configuration: self.notifications_configuration,
            meeting_features: self.meeting_features,
            primary_meeting_id: self.primary_meeting_id,
            tenant_ids: self.tenant_ids,
            tags: self.tags,
        })
    }
}
impl ::std::fmt::Debug for CreateMeetingInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMeetingInputBuilder");
        formatter.field("client_request_token", &"*** Sensitive Data Redacted ***");
        formatter.field("media_region", &self.media_region);
        formatter.field("meeting_host_id", &"*** Sensitive Data Redacted ***");
        formatter.field("external_meeting_id", &"*** Sensitive Data Redacted ***");
        formatter.field("notifications_configuration", &self.notifications_configuration);
        formatter.field("meeting_features", &self.meeting_features);
        formatter.field("primary_meeting_id", &self.primary_meeting_id);
        formatter.field("tenant_ids", &self.tenant_ids);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
