// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_meeting::_create_meeting_output::CreateMeetingOutputBuilder;

pub use crate::operation::create_meeting::_create_meeting_input::CreateMeetingInputBuilder;

impl crate::operation::create_meeting::builders::CreateMeetingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_meeting::CreateMeetingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_meeting::CreateMeetingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_meeting();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateMeeting`.
///
/// <p>Creates a new Amazon Chime SDK meeting in the specified media Region with no initial attendees. For more information about specifying media Regions, see <a href="https://docs.aws.amazon.com/chime/latest/dg/chime-sdk-meetings-regions.html">Amazon Chime SDK Media Regions</a> in the <i>Amazon Chime Developer Guide</i>. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMeetingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_meeting::builders::CreateMeetingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_meeting::CreateMeetingOutput,
        crate::operation::create_meeting::CreateMeetingError,
    > for CreateMeetingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_meeting::CreateMeetingOutput,
            crate::operation::create_meeting::CreateMeetingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateMeetingFluentBuilder {
    /// Creates a new `CreateMeetingFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateMeeting as a reference.
    pub fn as_input(&self) -> &crate::operation::create_meeting::builders::CreateMeetingInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_meeting::CreateMeetingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_meeting::CreateMeetingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_meeting::CreateMeeting::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_meeting::CreateMeeting::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_meeting::CreateMeetingOutput,
        crate::operation::create_meeting::CreateMeetingError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub fn media_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.media_region(input.into());
        self
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub fn set_media_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_media_region(input);
        self
    }
    /// <p>The Region in which to create the meeting.</p>
    /// <p>Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    /// <p>Available values in Amazon Web Services GovCloud (US) Regions: <code>us-gov-east-1</code>, <code>us-gov-west-1</code>.</p>
    pub fn get_media_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_media_region()
    }
    /// <p>Reserved.</p>
    pub fn meeting_host_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meeting_host_id(input.into());
        self
    }
    /// <p>Reserved.</p>
    pub fn set_meeting_host_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_host_id(input);
        self
    }
    /// <p>Reserved.</p>
    pub fn get_meeting_host_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meeting_host_id()
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub fn external_meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.external_meeting_id(input.into());
        self
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub fn set_external_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_external_meeting_id(input);
        self
    }
    /// <p>The external meeting ID.</p>
    /// <p>Pattern: <code>\[-_&amp;@+=,(){}\\[\\]\/«».:|'"#a-zA-Z0-9À-ÿ\s\]*</code></p>
    /// <p>Values that begin with <code>aws:</code> are reserved. You can't configure a value that uses this prefix. Case insensitive.</p>
    pub fn get_external_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_external_meeting_id()
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn notifications_configuration(mut self, input: crate::types::NotificationsConfiguration) -> Self {
        self.inner = self.inner.notifications_configuration(input);
        self
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn set_notifications_configuration(mut self, input: ::std::option::Option<crate::types::NotificationsConfiguration>) -> Self {
        self.inner = self.inner.set_notifications_configuration(input);
        self
    }
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    pub fn get_notifications_configuration(&self) -> &::std::option::Option<crate::types::NotificationsConfiguration> {
        self.inner.get_notifications_configuration()
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn meeting_features(mut self, input: crate::types::MeetingFeaturesConfiguration) -> Self {
        self.inner = self.inner.meeting_features(input);
        self
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn set_meeting_features(mut self, input: ::std::option::Option<crate::types::MeetingFeaturesConfiguration>) -> Self {
        self.inner = self.inner.set_meeting_features(input);
        self
    }
    /// <p>Lists the audio and video features enabled for a meeting, such as echo reduction.</p>
    pub fn get_meeting_features(&self) -> &::std::option::Option<crate::types::MeetingFeaturesConfiguration> {
        self.inner.get_meeting_features()
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn primary_meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.primary_meeting_id(input.into());
        self
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn set_primary_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_primary_meeting_id(input);
        self
    }
    /// <p>When specified, replicates the media from the primary meeting to the new meeting.</p>
    pub fn get_primary_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_primary_meeting_id()
    }
    ///
    /// Appends an item to `TenantIds`.
    ///
    /// To override the contents of this collection use [`set_tenant_ids`](Self::set_tenant_ids).
    ///
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub fn tenant_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tenant_ids(input.into());
        self
    }
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub fn set_tenant_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_tenant_ids(input);
        self
    }
    /// <p>A consistent and opaque identifier, created and maintained by the builder to represent a segment of their users.</p>
    pub fn get_tenant_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_tenant_ids()
    }
    ///
    /// Appends an item to `Tags`.
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
        self.inner = self.inner.tags(input);
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
        self.inner = self.inner.set_tags(input);
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
        self.inner.get_tags()
    }
}