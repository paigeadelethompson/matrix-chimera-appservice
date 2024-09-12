// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary information about an export task.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportSummary {
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub export_arn: ::std::option::Option<::std::string::String>,
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub export_status: ::std::option::Option<crate::types::ExportStatus>,
    /// <p>The type of export that was performed. Valid values are <code>FULL_EXPORT</code> or <code>INCREMENTAL_EXPORT</code>.</p>
    pub export_type: ::std::option::Option<crate::types::ExportType>,
}
impl ExportSummary {
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn export_arn(&self) -> ::std::option::Option<&str> {
        self.export_arn.as_deref()
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn export_status(&self) -> ::std::option::Option<&crate::types::ExportStatus> {
        self.export_status.as_ref()
    }
    /// <p>The type of export that was performed. Valid values are <code>FULL_EXPORT</code> or <code>INCREMENTAL_EXPORT</code>.</p>
    pub fn export_type(&self) -> ::std::option::Option<&crate::types::ExportType> {
        self.export_type.as_ref()
    }
}
impl ExportSummary {
    /// Creates a new builder-style object to manufacture [`ExportSummary`](crate::types::ExportSummary).
    pub fn builder() -> crate::types::builders::ExportSummaryBuilder {
        crate::types::builders::ExportSummaryBuilder::default()
    }
}

/// A builder for [`ExportSummary`](crate::types::ExportSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ExportSummaryBuilder {
    pub(crate) export_arn: ::std::option::Option<::std::string::String>,
    pub(crate) export_status: ::std::option::Option<crate::types::ExportStatus>,
    pub(crate) export_type: ::std::option::Option<crate::types::ExportType>,
}
impl ExportSummaryBuilder {
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn export_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.export_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn set_export_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.export_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the export.</p>
    pub fn get_export_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.export_arn
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn export_status(mut self, input: crate::types::ExportStatus) -> Self {
        self.export_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn set_export_status(mut self, input: ::std::option::Option<crate::types::ExportStatus>) -> Self {
        self.export_status = input;
        self
    }
    /// <p>Export can be in one of the following states: IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub fn get_export_status(&self) -> &::std::option::Option<crate::types::ExportStatus> {
        &self.export_status
    }
    /// <p>The type of export that was performed. Valid values are <code>FULL_EXPORT</code> or <code>INCREMENTAL_EXPORT</code>.</p>
    pub fn export_type(mut self, input: crate::types::ExportType) -> Self {
        self.export_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of export that was performed. Valid values are <code>FULL_EXPORT</code> or <code>INCREMENTAL_EXPORT</code>.</p>
    pub fn set_export_type(mut self, input: ::std::option::Option<crate::types::ExportType>) -> Self {
        self.export_type = input;
        self
    }
    /// <p>The type of export that was performed. Valid values are <code>FULL_EXPORT</code> or <code>INCREMENTAL_EXPORT</code>.</p>
    pub fn get_export_type(&self) -> &::std::option::Option<crate::types::ExportType> {
        &self.export_type
    }
    /// Consumes the builder and constructs a [`ExportSummary`](crate::types::ExportSummary).
    pub fn build(self) -> crate::types::ExportSummary {
        crate::types::ExportSummary {
            export_arn: self.export_arn,
            export_status: self.export_status,
            export_type: self.export_type,
        }
    }
}