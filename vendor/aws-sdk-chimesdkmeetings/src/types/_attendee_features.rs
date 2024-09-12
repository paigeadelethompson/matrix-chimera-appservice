// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lists the maximum number of attendees allowed into the meeting.</p><note>
/// <p>If you specify <code>FHD</code> for <code>MeetingFeatures:Video:MaxResolution</code>, or if you specify <code>UHD</code> for <code>MeetingFeatures:Content:MaxResolution</code>, the maximum number of attendees changes from the default of <code>250</code> to <code>25</code>.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AttendeeFeatures {
    /// <p>The maximum number of attendees allowed into the meeting.</p>
    pub max_count: ::std::option::Option<i32>,
}
impl AttendeeFeatures {
    /// <p>The maximum number of attendees allowed into the meeting.</p>
    pub fn max_count(&self) -> ::std::option::Option<i32> {
        self.max_count
    }
}
impl AttendeeFeatures {
    /// Creates a new builder-style object to manufacture [`AttendeeFeatures`](crate::types::AttendeeFeatures).
    pub fn builder() -> crate::types::builders::AttendeeFeaturesBuilder {
        crate::types::builders::AttendeeFeaturesBuilder::default()
    }
}

/// A builder for [`AttendeeFeatures`](crate::types::AttendeeFeatures).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AttendeeFeaturesBuilder {
    pub(crate) max_count: ::std::option::Option<i32>,
}
impl AttendeeFeaturesBuilder {
    /// <p>The maximum number of attendees allowed into the meeting.</p>
    pub fn max_count(mut self, input: i32) -> Self {
        self.max_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of attendees allowed into the meeting.</p>
    pub fn set_max_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_count = input;
        self
    }
    /// <p>The maximum number of attendees allowed into the meeting.</p>
    pub fn get_max_count(&self) -> &::std::option::Option<i32> {
        &self.max_count
    }
    /// Consumes the builder and constructs a [`AttendeeFeatures`](crate::types::AttendeeFeatures).
    pub fn build(self) -> crate::types::AttendeeFeatures {
        crate::types::AttendeeFeatures { max_count: self.max_count }
    }
}
