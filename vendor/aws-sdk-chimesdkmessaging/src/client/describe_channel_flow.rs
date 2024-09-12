// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeChannelFlow`](crate::operation::describe_channel_flow::builders::DescribeChannelFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_flow_arn(impl Into<String>)`](crate::operation::describe_channel_flow::builders::DescribeChannelFlowFluentBuilder::channel_flow_arn) / [`set_channel_flow_arn(Option<String>)`](crate::operation::describe_channel_flow::builders::DescribeChannelFlowFluentBuilder::set_channel_flow_arn):<br>required: **true**<br><p>The ARN of the channel flow.</p><br>
    /// - On success, responds with [`DescribeChannelFlowOutput`](crate::operation::describe_channel_flow::DescribeChannelFlowOutput) with field(s):
    ///   - [`channel_flow(Option<ChannelFlow>)`](crate::operation::describe_channel_flow::DescribeChannelFlowOutput::channel_flow): <p>The channel flow details.</p>
    /// - On failure, responds with [`SdkError<DescribeChannelFlowError>`](crate::operation::describe_channel_flow::DescribeChannelFlowError)
    pub fn describe_channel_flow(&self) -> crate::operation::describe_channel_flow::builders::DescribeChannelFlowFluentBuilder {
        crate::operation::describe_channel_flow::builders::DescribeChannelFlowFluentBuilder::new(self.handle.clone())
    }
}
