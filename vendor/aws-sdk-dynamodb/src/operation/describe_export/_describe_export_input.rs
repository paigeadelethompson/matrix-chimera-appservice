// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExportInput {
    /// <p>The Amazon Resource Name (ARN) associated with the export.</p>
    pub export_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeExportInput {
    /// <p>The Amazon Resource Name (ARN) associated with the export.</p>
    pub fn export_arn(&self) -> ::std::option::Option<&str> {
        self.export_arn.as_deref()
    }
}
impl DescribeExportInput {
    /// Creates a new builder-style object to manufacture [`DescribeExportInput`](crate::operation::describe_export::DescribeExportInput).
    pub fn builder() -> crate::operation::describe_export::builders::DescribeExportInputBuilder {
        crate::operation::describe_export::builders::DescribeExportInputBuilder::default()
    }
}

/// A builder for [`DescribeExportInput`](crate::operation::describe_export::DescribeExportInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeExportInputBuilder {
    pub(crate) export_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeExportInputBuilder {
    /// <p>The Amazon Resource Name (ARN) associated with the export.</p>
    /// This field is required.
    pub fn export_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.export_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the export.</p>
    pub fn set_export_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.export_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the export.</p>
    pub fn get_export_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.export_arn
    }
    /// Consumes the builder and constructs a [`DescribeExportInput`](crate::operation::describe_export::DescribeExportInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_export::DescribeExportInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_export::DescribeExportInput { export_arn: self.export_arn })
    }
}
