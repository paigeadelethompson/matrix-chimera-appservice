// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteProxySessionOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteProxySessionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteProxySessionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteProxySessionOutput`](crate::operation::delete_proxy_session::DeleteProxySessionOutput).
    pub fn builder() -> crate::operation::delete_proxy_session::builders::DeleteProxySessionOutputBuilder {
        crate::operation::delete_proxy_session::builders::DeleteProxySessionOutputBuilder::default()
    }
}

/// A builder for [`DeleteProxySessionOutput`](crate::operation::delete_proxy_session::DeleteProxySessionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteProxySessionOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteProxySessionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteProxySessionOutput`](crate::operation::delete_proxy_session::DeleteProxySessionOutput).
    pub fn build(self) -> crate::operation::delete_proxy_session::DeleteProxySessionOutput {
        crate::operation::delete_proxy_session::DeleteProxySessionOutput {
            _request_id: self._request_id,
        }
    }
}