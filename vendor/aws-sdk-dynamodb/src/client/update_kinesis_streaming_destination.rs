// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateKinesisStreamingDestination`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<String>)`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::set_table_name):<br>required: **true**<br><p>The table name for the Kinesis streaming destination input. You can also provide the ARN of the table in this parameter.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::set_stream_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) for the Kinesis stream input.</p><br>
    ///   - [`update_kinesis_streaming_configuration(UpdateKinesisStreamingConfiguration)`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::update_kinesis_streaming_configuration) / [`set_update_kinesis_streaming_configuration(Option<UpdateKinesisStreamingConfiguration>)`](crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::set_update_kinesis_streaming_configuration):<br>required: **false**<br><p>The command to update the Kinesis stream configuration.</p><br>
    /// - On success, responds with [`UpdateKinesisStreamingDestinationOutput`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput) with field(s):
    ///   - [`table_name(Option<String>)`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput::table_name): <p>The table name for the Kinesis streaming destination output.</p>
    ///   - [`stream_arn(Option<String>)`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput::stream_arn): <p>The ARN for the Kinesis stream input.</p>
    ///   - [`destination_status(Option<DestinationStatus>)`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput::destination_status): <p>The status of the attempt to update the Kinesis streaming destination output.</p>
    ///   - [`update_kinesis_streaming_configuration(Option<UpdateKinesisStreamingConfiguration>)`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput::update_kinesis_streaming_configuration): <p>The command to update the Kinesis streaming destination configuration.</p>
    /// - On failure, responds with [`SdkError<UpdateKinesisStreamingDestinationError>`](crate::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationError)
    pub fn update_kinesis_streaming_destination(
        &self,
    ) -> crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder {
        crate::operation::update_kinesis_streaming_destination::builders::UpdateKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
    }
}
