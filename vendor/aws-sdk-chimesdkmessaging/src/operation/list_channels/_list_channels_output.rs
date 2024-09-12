// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ListChannelsOutput {
    /// <p>The information about each channel.</p>
    pub channels: ::std::option::Option<::std::vec::Vec<crate::types::ChannelSummary>>,
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListChannelsOutput {
    /// <p>The information about each channel.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.channels.is_none()`.
    pub fn channels(&self) -> &[crate::types::ChannelSummary] {
        self.channels.as_deref().unwrap_or_default()
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::std::fmt::Debug for ListChannelsOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListChannelsOutput");
        formatter.field("channels", &self.channels);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for ListChannelsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListChannelsOutput {
    /// Creates a new builder-style object to manufacture [`ListChannelsOutput`](crate::operation::list_channels::ListChannelsOutput).
    pub fn builder() -> crate::operation::list_channels::builders::ListChannelsOutputBuilder {
        crate::operation::list_channels::builders::ListChannelsOutputBuilder::default()
    }
}

/// A builder for [`ListChannelsOutput`](crate::operation::list_channels::ListChannelsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct ListChannelsOutputBuilder {
    pub(crate) channels: ::std::option::Option<::std::vec::Vec<crate::types::ChannelSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListChannelsOutputBuilder {
    /// Appends an item to `channels`.
    ///
    /// To override the contents of this collection use [`set_channels`](Self::set_channels).
    ///
    /// <p>The information about each channel.</p>
    pub fn channels(mut self, input: crate::types::ChannelSummary) -> Self {
        let mut v = self.channels.unwrap_or_default();
        v.push(input);
        self.channels = ::std::option::Option::Some(v);
        self
    }
    /// <p>The information about each channel.</p>
    pub fn set_channels(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ChannelSummary>>) -> Self {
        self.channels = input;
        self
    }
    /// <p>The information about each channel.</p>
    pub fn get_channels(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ChannelSummary>> {
        &self.channels
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListChannelsOutput`](crate::operation::list_channels::ListChannelsOutput).
    pub fn build(self) -> crate::operation::list_channels::ListChannelsOutput {
        crate::operation::list_channels::ListChannelsOutput {
            channels: self.channels,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for ListChannelsOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListChannelsOutputBuilder");
        formatter.field("channels", &self.channels);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
