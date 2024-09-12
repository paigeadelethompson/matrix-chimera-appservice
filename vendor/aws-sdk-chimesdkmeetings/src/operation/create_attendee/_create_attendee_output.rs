// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateAttendeeOutput {
    /// <p>The attendee information, including attendee ID and join token.</p>
    pub attendee: ::std::option::Option<crate::types::Attendee>,
    _request_id: Option<String>,
}
impl CreateAttendeeOutput {
    /// <p>The attendee information, including attendee ID and join token.</p>
    pub fn attendee(&self) -> ::std::option::Option<&crate::types::Attendee> {
        self.attendee.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateAttendeeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateAttendeeOutput {
    /// Creates a new builder-style object to manufacture [`CreateAttendeeOutput`](crate::operation::create_attendee::CreateAttendeeOutput).
    pub fn builder() -> crate::operation::create_attendee::builders::CreateAttendeeOutputBuilder {
        crate::operation::create_attendee::builders::CreateAttendeeOutputBuilder::default()
    }
}

/// A builder for [`CreateAttendeeOutput`](crate::operation::create_attendee::CreateAttendeeOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateAttendeeOutputBuilder {
    pub(crate) attendee: ::std::option::Option<crate::types::Attendee>,
    _request_id: Option<String>,
}
impl CreateAttendeeOutputBuilder {
    /// <p>The attendee information, including attendee ID and join token.</p>
    pub fn attendee(mut self, input: crate::types::Attendee) -> Self {
        self.attendee = ::std::option::Option::Some(input);
        self
    }
    /// <p>The attendee information, including attendee ID and join token.</p>
    pub fn set_attendee(mut self, input: ::std::option::Option<crate::types::Attendee>) -> Self {
        self.attendee = input;
        self
    }
    /// <p>The attendee information, including attendee ID and join token.</p>
    pub fn get_attendee(&self) -> &::std::option::Option<crate::types::Attendee> {
        &self.attendee
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateAttendeeOutput`](crate::operation::create_attendee::CreateAttendeeOutput).
    pub fn build(self) -> crate::operation::create_attendee::CreateAttendeeOutput {
        crate::operation::create_attendee::CreateAttendeeOutput {
            attendee: self.attendee,
            _request_id: self._request_id,
        }
    }
}