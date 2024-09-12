// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecuteTransactionOutput {
    /// <p>The response to a PartiQL transaction.</p>
    pub responses: ::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>>,
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub consumed_capacity: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    _request_id: Option<String>,
}
impl ExecuteTransactionOutput {
    /// <p>The response to a PartiQL transaction.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.responses.is_none()`.
    pub fn responses(&self) -> &[crate::types::ItemResponse] {
        self.responses.as_deref().unwrap_or_default()
    }
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.consumed_capacity.is_none()`.
    pub fn consumed_capacity(&self) -> &[crate::types::ConsumedCapacity] {
        self.consumed_capacity.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ExecuteTransactionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ExecuteTransactionOutput {
    /// Creates a new builder-style object to manufacture [`ExecuteTransactionOutput`](crate::operation::execute_transaction::ExecuteTransactionOutput).
    pub fn builder() -> crate::operation::execute_transaction::builders::ExecuteTransactionOutputBuilder {
        crate::operation::execute_transaction::builders::ExecuteTransactionOutputBuilder::default()
    }
}

/// A builder for [`ExecuteTransactionOutput`](crate::operation::execute_transaction::ExecuteTransactionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ExecuteTransactionOutputBuilder {
    pub(crate) responses: ::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>>,
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    _request_id: Option<String>,
}
impl ExecuteTransactionOutputBuilder {
    /// Appends an item to `responses`.
    ///
    /// To override the contents of this collection use [`set_responses`](Self::set_responses).
    ///
    /// <p>The response to a PartiQL transaction.</p>
    pub fn responses(mut self, input: crate::types::ItemResponse) -> Self {
        let mut v = self.responses.unwrap_or_default();
        v.push(input);
        self.responses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The response to a PartiQL transaction.</p>
    pub fn set_responses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>>) -> Self {
        self.responses = input;
        self
    }
    /// <p>The response to a PartiQL transaction.</p>
    pub fn get_responses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>> {
        &self.responses
    }
    /// Appends an item to `consumed_capacity`.
    ///
    /// To override the contents of this collection use [`set_consumed_capacity`](Self::set_consumed_capacity).
    ///
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub fn consumed_capacity(mut self, input: crate::types::ConsumedCapacity) -> Self {
        let mut v = self.consumed_capacity.unwrap_or_default();
        v.push(input);
        self.consumed_capacity = ::std::option::Option::Some(v);
        self
    }
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub fn set_consumed_capacity(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>) -> Self {
        self.consumed_capacity = input;
        self
    }
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub fn get_consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>> {
        &self.consumed_capacity
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ExecuteTransactionOutput`](crate::operation::execute_transaction::ExecuteTransactionOutput).
    pub fn build(self) -> crate::operation::execute_transaction::ExecuteTransactionOutput {
        crate::operation::execute_transaction::ExecuteTransactionOutput {
            responses: self.responses,
            consumed_capacity: self.consumed_capacity,
            _request_id: self._request_id,
        }
    }
}
