// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteChannelBanOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteChannelBanOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteChannelBanOutput {
    /// Creates a new builder-style object to manufacture [`DeleteChannelBanOutput`](crate::operation::delete_channel_ban::DeleteChannelBanOutput).
    pub fn builder() -> crate::operation::delete_channel_ban::builders::DeleteChannelBanOutputBuilder {
        crate::operation::delete_channel_ban::builders::DeleteChannelBanOutputBuilder::default()
    }
}

/// A builder for [`DeleteChannelBanOutput`](crate::operation::delete_channel_ban::DeleteChannelBanOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteChannelBanOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteChannelBanOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteChannelBanOutput`](crate::operation::delete_channel_ban::DeleteChannelBanOutput).
    pub fn build(self) -> crate::operation::delete_channel_ban::DeleteChannelBanOutput {
        crate::operation::delete_channel_ban::DeleteChannelBanOutput {
            _request_id: self._request_id,
        }
    }
}