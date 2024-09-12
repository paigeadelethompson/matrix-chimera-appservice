// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportTableInput {
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub s3_bucket_source: ::std::option::Option<crate::types::S3BucketSource>,
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub input_format: ::std::option::Option<crate::types::InputFormat>,
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub input_format_options: ::std::option::Option<crate::types::InputFormatOptions>,
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub input_compression_type: ::std::option::Option<crate::types::InputCompressionType>,
    /// <p>Parameters for the table to import the data into.</p>
    pub table_creation_parameters: ::std::option::Option<crate::types::TableCreationParameters>,
}
impl ImportTableInput {
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub fn s3_bucket_source(&self) -> ::std::option::Option<&crate::types::S3BucketSource> {
        self.s3_bucket_source.as_ref()
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn input_format(&self) -> ::std::option::Option<&crate::types::InputFormat> {
        self.input_format.as_ref()
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn input_format_options(&self) -> ::std::option::Option<&crate::types::InputFormatOptions> {
        self.input_format_options.as_ref()
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn input_compression_type(&self) -> ::std::option::Option<&crate::types::InputCompressionType> {
        self.input_compression_type.as_ref()
    }
    /// <p>Parameters for the table to import the data into.</p>
    pub fn table_creation_parameters(&self) -> ::std::option::Option<&crate::types::TableCreationParameters> {
        self.table_creation_parameters.as_ref()
    }
}
impl ImportTableInput {
    /// Creates a new builder-style object to manufacture [`ImportTableInput`](crate::operation::import_table::ImportTableInput).
    pub fn builder() -> crate::operation::import_table::builders::ImportTableInputBuilder {
        crate::operation::import_table::builders::ImportTableInputBuilder::default()
    }
}

/// A builder for [`ImportTableInput`](crate::operation::import_table::ImportTableInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ImportTableInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) s3_bucket_source: ::std::option::Option<crate::types::S3BucketSource>,
    pub(crate) input_format: ::std::option::Option<crate::types::InputFormat>,
    pub(crate) input_format_options: ::std::option::Option<crate::types::InputFormatOptions>,
    pub(crate) input_compression_type: ::std::option::Option<crate::types::InputCompressionType>,
    pub(crate) table_creation_parameters: ::std::option::Option<crate::types::TableCreationParameters>,
}
impl ImportTableInputBuilder {
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    /// This field is required.
    pub fn s3_bucket_source(mut self, input: crate::types::S3BucketSource) -> Self {
        self.s3_bucket_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub fn set_s3_bucket_source(mut self, input: ::std::option::Option<crate::types::S3BucketSource>) -> Self {
        self.s3_bucket_source = input;
        self
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub fn get_s3_bucket_source(&self) -> &::std::option::Option<crate::types::S3BucketSource> {
        &self.s3_bucket_source
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    /// This field is required.
    pub fn input_format(mut self, input: crate::types::InputFormat) -> Self {
        self.input_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn set_input_format(mut self, input: ::std::option::Option<crate::types::InputFormat>) -> Self {
        self.input_format = input;
        self
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn get_input_format(&self) -> &::std::option::Option<crate::types::InputFormat> {
        &self.input_format
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn input_format_options(mut self, input: crate::types::InputFormatOptions) -> Self {
        self.input_format_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn set_input_format_options(mut self, input: ::std::option::Option<crate::types::InputFormatOptions>) -> Self {
        self.input_format_options = input;
        self
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn get_input_format_options(&self) -> &::std::option::Option<crate::types::InputFormatOptions> {
        &self.input_format_options
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn input_compression_type(mut self, input: crate::types::InputCompressionType) -> Self {
        self.input_compression_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn set_input_compression_type(mut self, input: ::std::option::Option<crate::types::InputCompressionType>) -> Self {
        self.input_compression_type = input;
        self
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn get_input_compression_type(&self) -> &::std::option::Option<crate::types::InputCompressionType> {
        &self.input_compression_type
    }
    /// <p>Parameters for the table to import the data into.</p>
    /// This field is required.
    pub fn table_creation_parameters(mut self, input: crate::types::TableCreationParameters) -> Self {
        self.table_creation_parameters = ::std::option::Option::Some(input);
        self
    }
    /// <p>Parameters for the table to import the data into.</p>
    pub fn set_table_creation_parameters(mut self, input: ::std::option::Option<crate::types::TableCreationParameters>) -> Self {
        self.table_creation_parameters = input;
        self
    }
    /// <p>Parameters for the table to import the data into.</p>
    pub fn get_table_creation_parameters(&self) -> &::std::option::Option<crate::types::TableCreationParameters> {
        &self.table_creation_parameters
    }
    /// Consumes the builder and constructs a [`ImportTableInput`](crate::operation::import_table::ImportTableInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::import_table::ImportTableInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::import_table::ImportTableInput {
            client_token: self.client_token,
            s3_bucket_source: self.s3_bucket_source,
            input_format: self.input_format,
            input_format_options: self.input_format_options,
            input_compression_type: self.input_compression_type,
            table_creation_parameters: self.table_creation_parameters,
        })
    }
}