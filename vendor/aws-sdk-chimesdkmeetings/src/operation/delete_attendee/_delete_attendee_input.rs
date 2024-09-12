// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAttendeeInput {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub attendee_id: ::std::option::Option<::std::string::String>,
}
impl DeleteAttendeeInput {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn meeting_id(&self) -> ::std::option::Option<&str> {
        self.meeting_id.as_deref()
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub fn attendee_id(&self) -> ::std::option::Option<&str> {
        self.attendee_id.as_deref()
    }
}
impl DeleteAttendeeInput {
    /// Creates a new builder-style object to manufacture [`DeleteAttendeeInput`](crate::operation::delete_attendee::DeleteAttendeeInput).
    pub fn builder() -> crate::operation::delete_attendee::builders::DeleteAttendeeInputBuilder {
        crate::operation::delete_attendee::builders::DeleteAttendeeInputBuilder::default()
    }
}

/// A builder for [`DeleteAttendeeInput`](crate::operation::delete_attendee::DeleteAttendeeInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteAttendeeInputBuilder {
    pub(crate) meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) attendee_id: ::std::option::Option<::std::string::String>,
}
impl DeleteAttendeeInputBuilder {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    /// This field is required.
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.meeting_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.meeting_id = input;
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.meeting_id
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    /// This field is required.
    pub fn attendee_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.attendee_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub fn set_attendee_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.attendee_id = input;
        self
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub fn get_attendee_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.attendee_id
    }
    /// Consumes the builder and constructs a [`DeleteAttendeeInput`](crate::operation::delete_attendee::DeleteAttendeeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_attendee::DeleteAttendeeInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_attendee::DeleteAttendeeInput {
            meeting_id: self.meeting_id,
            attendee_id: self.attendee_id,
        })
    }
}