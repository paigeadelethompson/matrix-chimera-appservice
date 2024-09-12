// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateMeetingWithAttendeesOutput {
    /// <p>The meeting information, including the meeting ID and <code>MediaPlacement</code>.</p>
    pub meeting: ::std::option::Option<crate::types::Meeting>,
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub attendees: ::std::option::Option<::std::vec::Vec<crate::types::Attendee>>,
    /// <p>If the action fails for one or more of the attendees in the request, a list of the attendees is returned, along with error codes and error messages.</p>
    pub errors: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeError>>,
    _request_id: Option<String>,
}
impl CreateMeetingWithAttendeesOutput {
    /// <p>The meeting information, including the meeting ID and <code>MediaPlacement</code>.</p>
    pub fn meeting(&self) -> ::std::option::Option<&crate::types::Meeting> {
        self.meeting.as_ref()
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.attendees.is_none()`.
    pub fn attendees(&self) -> &[crate::types::Attendee] {
        self.attendees.as_deref().unwrap_or_default()
    }
    /// <p>If the action fails for one or more of the attendees in the request, a list of the attendees is returned, along with error codes and error messages.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.errors.is_none()`.
    pub fn errors(&self) -> &[crate::types::CreateAttendeeError] {
        self.errors.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for CreateMeetingWithAttendeesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateMeetingWithAttendeesOutput {
    /// Creates a new builder-style object to manufacture [`CreateMeetingWithAttendeesOutput`](crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesOutput).
    pub fn builder() -> crate::operation::create_meeting_with_attendees::builders::CreateMeetingWithAttendeesOutputBuilder {
        crate::operation::create_meeting_with_attendees::builders::CreateMeetingWithAttendeesOutputBuilder::default()
    }
}

/// A builder for [`CreateMeetingWithAttendeesOutput`](crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateMeetingWithAttendeesOutputBuilder {
    pub(crate) meeting: ::std::option::Option<crate::types::Meeting>,
    pub(crate) attendees: ::std::option::Option<::std::vec::Vec<crate::types::Attendee>>,
    pub(crate) errors: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeError>>,
    _request_id: Option<String>,
}
impl CreateMeetingWithAttendeesOutputBuilder {
    /// <p>The meeting information, including the meeting ID and <code>MediaPlacement</code>.</p>
    pub fn meeting(mut self, input: crate::types::Meeting) -> Self {
        self.meeting = ::std::option::Option::Some(input);
        self
    }
    /// <p>The meeting information, including the meeting ID and <code>MediaPlacement</code>.</p>
    pub fn set_meeting(mut self, input: ::std::option::Option<crate::types::Meeting>) -> Self {
        self.meeting = input;
        self
    }
    /// <p>The meeting information, including the meeting ID and <code>MediaPlacement</code>.</p>
    pub fn get_meeting(&self) -> &::std::option::Option<crate::types::Meeting> {
        &self.meeting
    }
    /// Appends an item to `attendees`.
    ///
    /// To override the contents of this collection use [`set_attendees`](Self::set_attendees).
    ///
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn attendees(mut self, input: crate::types::Attendee) -> Self {
        let mut v = self.attendees.unwrap_or_default();
        v.push(input);
        self.attendees = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn set_attendees(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Attendee>>) -> Self {
        self.attendees = input;
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn get_attendees(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Attendee>> {
        &self.attendees
    }
    /// Appends an item to `errors`.
    ///
    /// To override the contents of this collection use [`set_errors`](Self::set_errors).
    ///
    /// <p>If the action fails for one or more of the attendees in the request, a list of the attendees is returned, along with error codes and error messages.</p>
    pub fn errors(mut self, input: crate::types::CreateAttendeeError) -> Self {
        let mut v = self.errors.unwrap_or_default();
        v.push(input);
        self.errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>If the action fails for one or more of the attendees in the request, a list of the attendees is returned, along with error codes and error messages.</p>
    pub fn set_errors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeError>>) -> Self {
        self.errors = input;
        self
    }
    /// <p>If the action fails for one or more of the attendees in the request, a list of the attendees is returned, along with error codes and error messages.</p>
    pub fn get_errors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeError>> {
        &self.errors
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateMeetingWithAttendeesOutput`](crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesOutput).
    pub fn build(self) -> crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesOutput {
        crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesOutput {
            meeting: self.meeting,
            attendees: self.attendees,
            errors: self.errors,
            _request_id: self._request_id,
        }
    }
}
