// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_table::_import_table_output::ImportTableOutputBuilder;

pub use crate::operation::import_table::_import_table_input::ImportTableInputBuilder;

impl crate::operation::import_table::builders::ImportTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_table::ImportTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_table::ImportTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportTable`.
///
/// <p>Imports table data from an S3 bucket.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_table::builders::ImportTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::import_table::ImportTableOutput,
        crate::operation::import_table::ImportTableError,
    > for ImportTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::import_table::ImportTableOutput,
            crate::operation::import_table::ImportTableError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ImportTableFluentBuilder {
    /// Creates a new `ImportTableFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportTable as a reference.
    pub fn as_input(&self) -> &crate::operation::import_table::builders::ImportTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_table::ImportTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_table::ImportTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_table::ImportTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_table::ImportTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::import_table::ImportTableOutput,
        crate::operation::import_table::ImportTableError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Providing a <code>ClientToken</code> makes the call to <code>ImportTableInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p>
    /// <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p>
    /// <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>IdempotentParameterMismatch</code> exception.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub fn s3_bucket_source(mut self, input: crate::types::S3BucketSource) -> Self {
        self.inner = self.inner.s3_bucket_source(input);
        self
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub fn set_s3_bucket_source(mut self, input: ::std::option::Option<crate::types::S3BucketSource>) -> Self {
        self.inner = self.inner.set_s3_bucket_source(input);
        self
    }
    /// <p>The S3 bucket that provides the source for the import.</p>
    pub fn get_s3_bucket_source(&self) -> &::std::option::Option<crate::types::S3BucketSource> {
        self.inner.get_s3_bucket_source()
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn input_format(mut self, input: crate::types::InputFormat) -> Self {
        self.inner = self.inner.input_format(input);
        self
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn set_input_format(mut self, input: ::std::option::Option<crate::types::InputFormat>) -> Self {
        self.inner = self.inner.set_input_format(input);
        self
    }
    /// <p>The format of the source data. Valid values for <code>ImportFormat</code> are <code>CSV</code>, <code>DYNAMODB_JSON</code> or <code>ION</code>.</p>
    pub fn get_input_format(&self) -> &::std::option::Option<crate::types::InputFormat> {
        self.inner.get_input_format()
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn input_format_options(mut self, input: crate::types::InputFormatOptions) -> Self {
        self.inner = self.inner.input_format_options(input);
        self
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn set_input_format_options(mut self, input: ::std::option::Option<crate::types::InputFormatOptions>) -> Self {
        self.inner = self.inner.set_input_format_options(input);
        self
    }
    /// <p>Additional properties that specify how the input is formatted,</p>
    pub fn get_input_format_options(&self) -> &::std::option::Option<crate::types::InputFormatOptions> {
        self.inner.get_input_format_options()
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn input_compression_type(mut self, input: crate::types::InputCompressionType) -> Self {
        self.inner = self.inner.input_compression_type(input);
        self
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn set_input_compression_type(mut self, input: ::std::option::Option<crate::types::InputCompressionType>) -> Self {
        self.inner = self.inner.set_input_compression_type(input);
        self
    }
    /// <p>Type of compression to be used on the input coming from the imported table.</p>
    pub fn get_input_compression_type(&self) -> &::std::option::Option<crate::types::InputCompressionType> {
        self.inner.get_input_compression_type()
    }
    /// <p>Parameters for the table to import the data into.</p>
    pub fn table_creation_parameters(mut self, input: crate::types::TableCreationParameters) -> Self {
        self.inner = self.inner.table_creation_parameters(input);
        self
    }
    /// <p>Parameters for the table to import the data into.</p>
    pub fn set_table_creation_parameters(mut self, input: ::std::option::Option<crate::types::TableCreationParameters>) -> Self {
        self.inner = self.inner.set_table_creation_parameters(input);
        self
    }
    /// <p>Parameters for the table to import the data into.</p>
    pub fn get_table_creation_parameters(&self) -> &::std::option::Option<crate::types::TableCreationParameters> {
        self.inner.get_table_creation_parameters()
    }
}
