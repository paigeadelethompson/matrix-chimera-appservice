// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchCreateAttendeeInput {
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub attendees: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>,
}
impl BatchCreateAttendeeInput {
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub fn meeting_id(&self) -> ::std::option::Option<&str> {
        self.meeting_id.as_deref()
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.attendees.is_none()`.
    pub fn attendees(&self) -> &[crate::types::CreateAttendeeRequestItem] {
        self.attendees.as_deref().unwrap_or_default()
    }
}
impl BatchCreateAttendeeInput {
    /// Creates a new builder-style object to manufacture [`BatchCreateAttendeeInput`](crate::operation::batch_create_attendee::BatchCreateAttendeeInput).
    pub fn builder() -> crate::operation::batch_create_attendee::builders::BatchCreateAttendeeInputBuilder {
        crate::operation::batch_create_attendee::builders::BatchCreateAttendeeInputBuilder::default()
    }
}

/// A builder for [`BatchCreateAttendeeInput`](crate::operation::batch_create_attendee::BatchCreateAttendeeInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BatchCreateAttendeeInputBuilder {
    pub(crate) meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) attendees: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>,
}
impl BatchCreateAttendeeInputBuilder {
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    /// This field is required.
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.meeting_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.meeting_id = input;
        self
    }
    /// <p>The Amazon Chime SDK ID of the meeting to which you're adding attendees.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.meeting_id
    }
    /// Appends an item to `attendees`.
    ///
    /// To override the contents of this collection use [`set_attendees`](Self::set_attendees).
    ///
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn attendees(mut self, input: crate::types::CreateAttendeeRequestItem) -> Self {
        let mut v = self.attendees.unwrap_or_default();
        v.push(input);
        self.attendees = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn set_attendees(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>) -> Self {
        self.attendees = input;
        self
    }
    /// <p>The attendee information, including attendees' IDs and join tokens.</p>
    pub fn get_attendees(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>> {
        &self.attendees
    }
    /// Consumes the builder and constructs a [`BatchCreateAttendeeInput`](crate::operation::batch_create_attendee::BatchCreateAttendeeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::batch_create_attendee::BatchCreateAttendeeInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::batch_create_attendee::BatchCreateAttendeeInput {
            meeting_id: self.meeting_id,
            attendees: self.attendees,
        })
    }
}
