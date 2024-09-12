// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteMessagingStreamingConfigurations`](crate::operation::delete_messaging_streaming_configurations::builders::DeleteMessagingStreamingConfigurationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_instance_arn(impl Into<String>)`](crate::operation::delete_messaging_streaming_configurations::builders::DeleteMessagingStreamingConfigurationsFluentBuilder::app_instance_arn) / [`set_app_instance_arn(Option<String>)`](crate::operation::delete_messaging_streaming_configurations::builders::DeleteMessagingStreamingConfigurationsFluentBuilder::set_app_instance_arn):<br>required: **true**<br><p>The ARN of the streaming configurations being deleted.</p><br>
    /// - On success, responds with [`DeleteMessagingStreamingConfigurationsOutput`](crate::operation::delete_messaging_streaming_configurations::DeleteMessagingStreamingConfigurationsOutput)
    /// - On failure, responds with [`SdkError<DeleteMessagingStreamingConfigurationsError>`](crate::operation::delete_messaging_streaming_configurations::DeleteMessagingStreamingConfigurationsError)
    pub fn delete_messaging_streaming_configurations(
        &self,
    ) -> crate::operation::delete_messaging_streaming_configurations::builders::DeleteMessagingStreamingConfigurationsFluentBuilder {
        crate::operation::delete_messaging_streaming_configurations::builders::DeleteMessagingStreamingConfigurationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
