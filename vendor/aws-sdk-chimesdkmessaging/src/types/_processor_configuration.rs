// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A processor's metadata.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProcessorConfiguration {
    /// <p>Indicates that the processor is of type Lambda.</p>
    pub lambda: ::std::option::Option<crate::types::LambdaConfiguration>,
}
impl ProcessorConfiguration {
    /// <p>Indicates that the processor is of type Lambda.</p>
    pub fn lambda(&self) -> ::std::option::Option<&crate::types::LambdaConfiguration> {
        self.lambda.as_ref()
    }
}
impl ProcessorConfiguration {
    /// Creates a new builder-style object to manufacture [`ProcessorConfiguration`](crate::types::ProcessorConfiguration).
    pub fn builder() -> crate::types::builders::ProcessorConfigurationBuilder {
        crate::types::builders::ProcessorConfigurationBuilder::default()
    }
}

/// A builder for [`ProcessorConfiguration`](crate::types::ProcessorConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ProcessorConfigurationBuilder {
    pub(crate) lambda: ::std::option::Option<crate::types::LambdaConfiguration>,
}
impl ProcessorConfigurationBuilder {
    /// <p>Indicates that the processor is of type Lambda.</p>
    /// This field is required.
    pub fn lambda(mut self, input: crate::types::LambdaConfiguration) -> Self {
        self.lambda = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates that the processor is of type Lambda.</p>
    pub fn set_lambda(mut self, input: ::std::option::Option<crate::types::LambdaConfiguration>) -> Self {
        self.lambda = input;
        self
    }
    /// <p>Indicates that the processor is of type Lambda.</p>
    pub fn get_lambda(&self) -> &::std::option::Option<crate::types::LambdaConfiguration> {
        &self.lambda
    }
    /// Consumes the builder and constructs a [`ProcessorConfiguration`](crate::types::ProcessorConfiguration).
    pub fn build(self) -> crate::types::ProcessorConfiguration {
        crate::types::ProcessorConfiguration { lambda: self.lambda }
    }
}
