// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableKinesisStreamingDestination`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<String>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::set_table_name):<br>required: **true**<br><p>The name of the DynamoDB table. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::set_stream_arn):<br>required: **true**<br><p>The ARN for a Kinesis data stream.</p><br>
    ///   - [`enable_kinesis_streaming_configuration(EnableKinesisStreamingConfiguration)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::enable_kinesis_streaming_configuration) / [`set_enable_kinesis_streaming_configuration(Option<EnableKinesisStreamingConfiguration>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::set_enable_kinesis_streaming_configuration):<br>required: **false**<br><p>The source for the Kinesis streaming information that is being enabled.</p><br>
    /// - On success, responds with [`DisableKinesisStreamingDestinationOutput`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput) with field(s):
    ///   - [`table_name(Option<String>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::table_name): <p>The name of the table being modified.</p>
    ///   - [`stream_arn(Option<String>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::stream_arn): <p>The ARN for the specific Kinesis data stream.</p>
    ///   - [`destination_status(Option<DestinationStatus>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::destination_status): <p>The current status of the replication.</p>
    ///   - [`enable_kinesis_streaming_configuration(Option<EnableKinesisStreamingConfiguration>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::enable_kinesis_streaming_configuration): <p>The destination for the Kinesis streaming information that is being enabled.</p>
    /// - On failure, responds with [`SdkError<DisableKinesisStreamingDestinationError>`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError)
    pub fn disable_kinesis_streaming_destination(
        &self,
    ) -> crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder {
        crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
    }
}