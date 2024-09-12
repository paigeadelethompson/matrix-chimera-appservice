// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExportTableToPointInTime`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_arn(impl Into<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::table_arn) / [`set_table_arn(Option<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_table_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) associated with the table to export.</p><br>
    ///   - [`export_time(DateTime)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::export_time) / [`set_export_time(Option<DateTime>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_export_time):<br>required: **false**<br><p>Time in the past from which to export table data, counted in seconds from the start of the Unix epoch. The table export will be a snapshot of the table's state at this point in time.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_client_token):<br>required: **false**<br><p>Providing a <code>ClientToken</code> makes the call to <code>ExportTableToPointInTimeInput</code> idempotent, meaning that multiple identical calls have the same effect as one single call.</p> <p>A client token is valid for 8 hours after the first request that uses it is completed. After 8 hours, any request with the same client token is treated as a new request. Do not resubmit the same request with the same client token for more than 8 hours, or the result might not be idempotent.</p> <p>If you submit a request with the same client token but a change in other parameters within the 8-hour idempotency window, DynamoDB returns an <code>ImportConflictException</code>.</p><br>
    ///   - [`s3_bucket(impl Into<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_bucket) / [`set_s3_bucket(Option<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_bucket):<br>required: **true**<br><p>The name of the Amazon S3 bucket to export the snapshot to.</p><br>
    ///   - [`s3_bucket_owner(impl Into<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_bucket_owner) / [`set_s3_bucket_owner(Option<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_bucket_owner):<br>required: **false**<br><p>The ID of the Amazon Web Services account that owns the bucket the export will be stored in.</p><note>  <p>S3BucketOwner is a required parameter when exporting to a S3 bucket in another account.</p> </note><br>
    ///   - [`s3_prefix(impl Into<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_prefix) / [`set_s3_prefix(Option<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_prefix):<br>required: **false**<br><p>The Amazon S3 bucket prefix to use as the file name and path of the exported snapshot.</p><br>
    ///   - [`s3_sse_algorithm(S3SseAlgorithm)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_sse_algorithm) / [`set_s3_sse_algorithm(Option<S3SseAlgorithm>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_sse_algorithm):<br>required: **false**<br><p>Type of encryption used on the bucket where export data will be stored. Valid values for <code>S3SseAlgorithm</code> are:</p> <ul>  <li>   <p><code>AES256</code> - server-side encryption with Amazon S3 managed keys</p></li>  <li>   <p><code>KMS</code> - server-side encryption with KMS managed keys</p></li> </ul><br>
    ///   - [`s3_sse_kms_key_id(impl Into<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_sse_kms_key_id) / [`set_s3_sse_kms_key_id(Option<String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_sse_kms_key_id):<br>required: **false**<br><p>The ID of the KMS managed key used to encrypt the S3 bucket where export data will be stored (if applicable).</p><br>
    ///   - [`export_format(ExportFormat)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::export_format) / [`set_export_format(Option<ExportFormat>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_export_format):<br>required: **false**<br><p>The format for the exported data. Valid values for <code>ExportFormat</code> are <code>DYNAMODB_JSON</code> or <code>ION</code>.</p><br>
    ///   - [`export_type(ExportType)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::export_type) / [`set_export_type(Option<ExportType>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_export_type):<br>required: **false**<br><p>Choice of whether to execute as a full export or incremental export. Valid values are FULL_EXPORT or INCREMENTAL_EXPORT. The default value is FULL_EXPORT. If INCREMENTAL_EXPORT is provided, the IncrementalExportSpecification must also be used.</p><br>
    ///   - [`incremental_export_specification(IncrementalExportSpecification)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::incremental_export_specification) / [`set_incremental_export_specification(Option<IncrementalExportSpecification>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_incremental_export_specification):<br>required: **false**<br><p>Optional object containing the parameters specific to an incremental export.</p><br>
    /// - On success, responds with [`ExportTableToPointInTimeOutput`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput) with field(s):
    ///   - [`export_description(Option<ExportDescription>)`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput::export_description): <p>Contains a description of the table export.</p>
    /// - On failure, responds with [`SdkError<ExportTableToPointInTimeError>`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError)
    pub fn export_table_to_point_in_time(&self) -> crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder {
        crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::new(self.handle.clone())
    }
}
