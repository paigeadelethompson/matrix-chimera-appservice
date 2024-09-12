// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeKinesisStreamingDestinationInput {
    /// <p>The name of the table being described. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeKinesisStreamingDestinationInput {
    /// <p>The name of the table being described. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
}
impl DescribeKinesisStreamingDestinationInput {
    /// Creates a new builder-style object to manufacture [`DescribeKinesisStreamingDestinationInput`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput).
    pub fn builder() -> crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
        crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder::default()
    }
}

/// A builder for [`DescribeKinesisStreamingDestinationInput`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeKinesisStreamingDestinationInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeKinesisStreamingDestinationInputBuilder {
    /// <p>The name of the table being described. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    /// This field is required.
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table being described. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The name of the table being described. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// Consumes the builder and constructs a [`DescribeKinesisStreamingDestinationInput`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput { table_name: self.table_name },
        )
    }
}