// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchUpdateAttendeeCapabilitiesExcept`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`meeting_id(impl Into<String>)`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::meeting_id) / [`set_meeting_id(Option<String>)`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::set_meeting_id):<br>required: **true**<br><p>The ID of the meeting associated with the update request.</p><br>
    ///   - [`excluded_attendee_ids(AttendeeIdItem)`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::excluded_attendee_ids) / [`set_excluded_attendee_ids(Option<Vec::<AttendeeIdItem>>)`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::set_excluded_attendee_ids):<br>required: **true**<br><p>The <code>AttendeeIDs</code> that you want to exclude from one or more capabilities.</p><br>
    ///   - [`capabilities(AttendeeCapabilities)`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::capabilities) / [`set_capabilities(Option<AttendeeCapabilities>)`](crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::set_capabilities):<br>required: **true**<br><p>The capabilities (<code>audio</code>, <code>video</code>, or <code>content</code>) that you want to update.</p><br>
    /// - On success, responds with [`BatchUpdateAttendeeCapabilitiesExceptOutput`](crate::operation::batch_update_attendee_capabilities_except::BatchUpdateAttendeeCapabilitiesExceptOutput)
    /// - On failure, responds with [`SdkError<BatchUpdateAttendeeCapabilitiesExceptError>`](crate::operation::batch_update_attendee_capabilities_except::BatchUpdateAttendeeCapabilitiesExceptError)
    pub fn batch_update_attendee_capabilities_except(
        &self,
    ) -> crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder {
        crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptFluentBuilder::new(
            self.handle.clone(),
        )
    }
}