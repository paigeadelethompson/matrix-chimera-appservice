// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The information about a processor in a channel flow.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Processor {
    /// <p>The name of the channel flow.</p>
    pub name: ::std::string::String,
    /// <p>The information about the type of processor and its identifier.</p>
    pub configuration: ::std::option::Option<crate::types::ProcessorConfiguration>,
    /// <p>The sequence in which processors run. If you have multiple processors in a channel flow, message processing goes through each processor in the sequence. The value determines the sequence. At this point, we support only 1 processor within a flow.</p>
    pub execution_order: i32,
    /// <p>Determines whether to continue with message processing or stop it in cases where communication with a processor fails. If a processor has a fallback action of <code>ABORT</code> and communication with it fails, the processor sets the message status to <code>FAILED</code> and does not send the message to any recipients. Note that if the last processor in the channel flow sequence has a fallback action of <code>CONTINUE</code> and communication with the processor fails, then the message is considered processed and sent to recipients of the channel.</p>
    pub fallback_action: crate::types::FallbackAction,
}
impl Processor {
    /// <p>The name of the channel flow.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>The information about the type of processor and its identifier.</p>
    pub fn configuration(&self) -> ::std::option::Option<&crate::types::ProcessorConfiguration> {
        self.configuration.as_ref()
    }
    /// <p>The sequence in which processors run. If you have multiple processors in a channel flow, message processing goes through each processor in the sequence. The value determines the sequence. At this point, we support only 1 processor within a flow.</p>
    pub fn execution_order(&self) -> i32 {
        self.execution_order
    }
    /// <p>Determines whether to continue with message processing or stop it in cases where communication with a processor fails. If a processor has a fallback action of <code>ABORT</code> and communication with it fails, the processor sets the message status to <code>FAILED</code> and does not send the message to any recipients. Note that if the last processor in the channel flow sequence has a fallback action of <code>CONTINUE</code> and communication with the processor fails, then the message is considered processed and sent to recipients of the channel.</p>
    pub fn fallback_action(&self) -> &crate::types::FallbackAction {
        &self.fallback_action
    }
}
impl ::std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Processor");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("configuration", &self.configuration);
        formatter.field("execution_order", &self.execution_order);
        formatter.field("fallback_action", &self.fallback_action);
        formatter.finish()
    }
}
impl Processor {
    /// Creates a new builder-style object to manufacture [`Processor`](crate::types::Processor).
    pub fn builder() -> crate::types::builders::ProcessorBuilder {
        crate::types::builders::ProcessorBuilder::default()
    }
}

/// A builder for [`Processor`](crate::types::Processor).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct ProcessorBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) configuration: ::std::option::Option<crate::types::ProcessorConfiguration>,
    pub(crate) execution_order: ::std::option::Option<i32>,
    pub(crate) fallback_action: ::std::option::Option<crate::types::FallbackAction>,
}
impl ProcessorBuilder {
    /// <p>The name of the channel flow.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the channel flow.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the channel flow.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The information about the type of processor and its identifier.</p>
    /// This field is required.
    pub fn configuration(mut self, input: crate::types::ProcessorConfiguration) -> Self {
        self.configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The information about the type of processor and its identifier.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::ProcessorConfiguration>) -> Self {
        self.configuration = input;
        self
    }
    /// <p>The information about the type of processor and its identifier.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::ProcessorConfiguration> {
        &self.configuration
    }
    /// <p>The sequence in which processors run. If you have multiple processors in a channel flow, message processing goes through each processor in the sequence. The value determines the sequence. At this point, we support only 1 processor within a flow.</p>
    /// This field is required.
    pub fn execution_order(mut self, input: i32) -> Self {
        self.execution_order = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sequence in which processors run. If you have multiple processors in a channel flow, message processing goes through each processor in the sequence. The value determines the sequence. At this point, we support only 1 processor within a flow.</p>
    pub fn set_execution_order(mut self, input: ::std::option::Option<i32>) -> Self {
        self.execution_order = input;
        self
    }
    /// <p>The sequence in which processors run. If you have multiple processors in a channel flow, message processing goes through each processor in the sequence. The value determines the sequence. At this point, we support only 1 processor within a flow.</p>
    pub fn get_execution_order(&self) -> &::std::option::Option<i32> {
        &self.execution_order
    }
    /// <p>Determines whether to continue with message processing or stop it in cases where communication with a processor fails. If a processor has a fallback action of <code>ABORT</code> and communication with it fails, the processor sets the message status to <code>FAILED</code> and does not send the message to any recipients. Note that if the last processor in the channel flow sequence has a fallback action of <code>CONTINUE</code> and communication with the processor fails, then the message is considered processed and sent to recipients of the channel.</p>
    /// This field is required.
    pub fn fallback_action(mut self, input: crate::types::FallbackAction) -> Self {
        self.fallback_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines whether to continue with message processing or stop it in cases where communication with a processor fails. If a processor has a fallback action of <code>ABORT</code> and communication with it fails, the processor sets the message status to <code>FAILED</code> and does not send the message to any recipients. Note that if the last processor in the channel flow sequence has a fallback action of <code>CONTINUE</code> and communication with the processor fails, then the message is considered processed and sent to recipients of the channel.</p>
    pub fn set_fallback_action(mut self, input: ::std::option::Option<crate::types::FallbackAction>) -> Self {
        self.fallback_action = input;
        self
    }
    /// <p>Determines whether to continue with message processing or stop it in cases where communication with a processor fails. If a processor has a fallback action of <code>ABORT</code> and communication with it fails, the processor sets the message status to <code>FAILED</code> and does not send the message to any recipients. Note that if the last processor in the channel flow sequence has a fallback action of <code>CONTINUE</code> and communication with the processor fails, then the message is considered processed and sent to recipients of the channel.</p>
    pub fn get_fallback_action(&self) -> &::std::option::Option<crate::types::FallbackAction> {
        &self.fallback_action
    }
    /// Consumes the builder and constructs a [`Processor`](crate::types::Processor).
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](crate::types::builders::ProcessorBuilder::name)
    /// - [`execution_order`](crate::types::builders::ProcessorBuilder::execution_order)
    /// - [`fallback_action`](crate::types::builders::ProcessorBuilder::fallback_action)
    pub fn build(self) -> ::std::result::Result<crate::types::Processor, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Processor {
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building Processor",
                )
            })?,
            configuration: self.configuration,
            execution_order: self.execution_order.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "execution_order",
                    "execution_order was not specified but it is required when building Processor",
                )
            })?,
            fallback_action: self.fallback_action.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "fallback_action",
                    "fallback_action was not specified but it is required when building Processor",
                )
            })?,
        })
    }
}
impl ::std::fmt::Debug for ProcessorBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ProcessorBuilder");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("configuration", &self.configuration);
        formatter.field("execution_order", &self.execution_order);
        formatter.field("fallback_action", &self.fallback_action);
        formatter.finish()
    }
}
