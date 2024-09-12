// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateChannelFlow`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of the channel.</p><br>
    ///   - [`channel_flow_arn(impl Into<String>)`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::channel_flow_arn) / [`set_channel_flow_arn(Option<String>)`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::set_channel_flow_arn):<br>required: **true**<br><p>The ARN of the channel flow.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The <code>AppInstanceUserArn</code> of the user making the API call.</p><br>
    /// - On success, responds with [`DisassociateChannelFlowOutput`](crate::operation::disassociate_channel_flow::DisassociateChannelFlowOutput)
    /// - On failure, responds with [`SdkError<DisassociateChannelFlowError>`](crate::operation::disassociate_channel_flow::DisassociateChannelFlowError)
    pub fn disassociate_channel_flow(&self) -> crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder {
        crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowFluentBuilder::new(self.handle.clone())
    }
}