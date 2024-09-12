// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateKinesisStreamingDestinationOutput {
    /// <p>The table name for the Kinesis streaming destination output.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN for the Kinesis stream input.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The status of the attempt to update the Kinesis streaming destination output.</p>
    pub destination_status: ::std::option::Option<crate::types::DestinationStatus>,
    /// <p>The command to update the Kinesis streaming destination configuration.</p>
    pub update_kinesis_streaming_configuration: ::std::option::Option<crate::types::UpdateKinesisStreamingConfiguration>,
    _request_id: Option<String>,
}
impl UpdateKinesisStreamingDestinationOutput {
    /// <p>The table name for the Kinesis streaming destination output.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>The ARN for the Kinesis stream input.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
    /// <p>The status of the attempt to update the Kinesis streaming destination output.</p>
    pub fn destination_status(&self) -> ::std::option::Option<&crate::types::DestinationStatus> {
        self.destination_status.as_ref()
    }
    /// <p>The command to update the Kinesis streaming destination configuration.</p>
    pub fn update_kinesis_streaming_configuration(&self) -> ::std::option::Option<&crate::types::UpdateKinesisStreamingConfiguration> {
        self.update_kinesis_streaming_configuration.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateKinesisStreamingDestinationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateKinesisStreamingDestinationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateKinesisStreamingDestinationOutput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput).
    pub fn builder() -> crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder {
        crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationOutputBuilder::default()
    }
}

/// A builder for [`UpdateKinesisStreamingDestinationOutput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateKinesisStreamingDestinationOutputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) destination_status: ::std::option::Option<crate::types::DestinationStatus>,
    pub(crate) update_kinesis_streaming_configuration: ::std::option::Option<crate::types::UpdateKinesisStreamingConfiguration>,
    _request_id: Option<String>,
}
impl UpdateKinesisStreamingDestinationOutputBuilder {
    /// <p>The table name for the Kinesis streaming destination output.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The table name for the Kinesis streaming destination output.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The table name for the Kinesis streaming destination output.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>The ARN for the Kinesis stream input.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN for the Kinesis stream input.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The ARN for the Kinesis stream input.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// <p>The status of the attempt to update the Kinesis streaming destination output.</p>
    pub fn destination_status(mut self, input: crate::types::DestinationStatus) -> Self {
        self.destination_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the attempt to update the Kinesis streaming destination output.</p>
    pub fn set_destination_status(mut self, input: ::std::option::Option<crate::types::DestinationStatus>) -> Self {
        self.destination_status = input;
        self
    }
    /// <p>The status of the attempt to update the Kinesis streaming destination output.</p>
    pub fn get_destination_status(&self) -> &::std::option::Option<crate::types::DestinationStatus> {
        &self.destination_status
    }
    /// <p>The command to update the Kinesis streaming destination configuration.</p>
    pub fn update_kinesis_streaming_configuration(mut self, input: crate::types::UpdateKinesisStreamingConfiguration) -> Self {
        self.update_kinesis_streaming_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The command to update the Kinesis streaming destination configuration.</p>
    pub fn set_update_kinesis_streaming_configuration(
        mut self,
        input: ::std::option::Option<crate::types::UpdateKinesisStreamingConfiguration>,
    ) -> Self {
        self.update_kinesis_streaming_configuration = input;
        self
    }
    /// <p>The command to update the Kinesis streaming destination configuration.</p>
    pub fn get_update_kinesis_streaming_configuration(&self) -> &::std::option::Option<crate::types::UpdateKinesisStreamingConfiguration> {
        &self.update_kinesis_streaming_configuration
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateKinesisStreamingDestinationOutput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput).
    pub fn build(self) -> crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput {
        crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput {
            table_name: self.table_name,
            stream_arn: self.stream_arn,
            destination_status: self.destination_status,
            update_kinesis_streaming_configuration: self.update_kinesis_streaming_configuration,
            _request_id: self._request_id,
        }
    }
}
