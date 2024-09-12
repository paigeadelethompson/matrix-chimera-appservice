// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAttendeeCapabilitiesOutput {
    /// <p>The updated attendee data.</p>
    pub attendee: ::std::option::Option<crate::types::Attendee>,
    _request_id: Option<String>,
}
impl UpdateAttendeeCapabilitiesOutput {
    /// <p>The updated attendee data.</p>
    pub fn attendee(&self) -> ::std::option::Option<&crate::types::Attendee> {
        self.attendee.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateAttendeeCapabilitiesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateAttendeeCapabilitiesOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAttendeeCapabilitiesOutput`](crate::operation::update_attendee_capabilities::UpdateAttendeeCapabilitiesOutput).
    pub fn builder() -> crate::operation::update_attendee_capabilities::builders::UpdateAttendeeCapabilitiesOutputBuilder {
        crate::operation::update_attendee_capabilities::builders::UpdateAttendeeCapabilitiesOutputBuilder::default()
    }
}

/// A builder for [`UpdateAttendeeCapabilitiesOutput`](crate::operation::update_attendee_capabilities::UpdateAttendeeCapabilitiesOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateAttendeeCapabilitiesOutputBuilder {
    pub(crate) attendee: ::std::option::Option<crate::types::Attendee>,
    _request_id: Option<String>,
}
impl UpdateAttendeeCapabilitiesOutputBuilder {
    /// <p>The updated attendee data.</p>
    pub fn attendee(mut self, input: crate::types::Attendee) -> Self {
        self.attendee = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated attendee data.</p>
    pub fn set_attendee(mut self, input: ::std::option::Option<crate::types::Attendee>) -> Self {
        self.attendee = input;
        self
    }
    /// <p>The updated attendee data.</p>
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
    /// Consumes the builder and constructs a [`UpdateAttendeeCapabilitiesOutput`](crate::operation::update_attendee_capabilities::UpdateAttendeeCapabilitiesOutput).
    pub fn build(self) -> crate::operation::update_attendee_capabilities::UpdateAttendeeCapabilitiesOutput {
        crate::operation::update_attendee_capabilities::UpdateAttendeeCapabilitiesOutput {
            attendee: self.attendee,
            _request_id: self._request_id,
        }
    }
}
