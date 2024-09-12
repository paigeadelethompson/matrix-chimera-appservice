// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableKinesisStreamingDestination`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_table_name):<br>required: **true**<br><p>The name of the DynamoDB table. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_stream_arn):<br>required: **true**<br><p>The ARN for a Kinesis data stream.</p><br>
    ///   - [`enable_kinesis_streaming_configuration(EnableKinesisStreamingConfiguration)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::enable_kinesis_streaming_configuration) / [`set_enable_kinesis_streaming_configuration(Option<EnableKinesisStreamingConfiguration>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_enable_kinesis_streaming_configuration):<br>required: **false**<br><p>The source for the Kinesis streaming information that is being enabled.</p><br>
    /// - On success, responds with [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput) with field(s):
    ///   - [`table_name(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::table_name): <p>The name of the table being modified.</p>
    ///   - [`stream_arn(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::stream_arn): <p>The ARN for the specific Kinesis data stream.</p>
    ///   - [`destination_status(Option<DestinationStatus>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::destination_status): <p>The current status of the replication.</p>
    ///   - [`enable_kinesis_streaming_configuration(Option<EnableKinesisStreamingConfiguration>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::enable_kinesis_streaming_configuration): <p>The destination for the Kinesis streaming information that is being enabled.</p>
    /// - On failure, responds with [`SdkError<EnableKinesisStreamingDestinationError>`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError)
    pub fn enable_kinesis_streaming_destination(
        &self,
    ) -> crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder {
        crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
    }
}
