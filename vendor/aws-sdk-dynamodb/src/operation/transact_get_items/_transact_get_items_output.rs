// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransactGetItemsOutput {
    /// <p>If the <i>ReturnConsumedCapacity</i> value was <code>TOTAL</code>, this is an array of <code>ConsumedCapacity</code> objects, one for each table addressed by <code>TransactGetItem</code> objects in the <i>TransactItems</i> parameter. These <code>ConsumedCapacity</code> objects report the read-capacity units consumed by the <code>TransactGetItems</code> call in that table.</p>
    pub consumed_capacity: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    /// <p>An ordered array of up to 100 <code>ItemResponse</code> objects, each of which corresponds to the <code>TransactGetItem</code> object in the same position in the <i>TransactItems</i> array. Each <code>ItemResponse</code> object contains a Map of the name-value pairs that are the projected attributes of the requested item.</p>
    /// <p>If a requested item could not be retrieved, the corresponding <code>ItemResponse</code> object is Null, or if the requested item has no projected attributes, the corresponding <code>ItemResponse</code> object is an empty Map.</p>
    pub responses: ::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>>,
    _request_id: Option<String>,
}
impl TransactGetItemsOutput {
    /// <p>If the <i>ReturnConsumedCapacity</i> value was <code>TOTAL</code>, this is an array of <code>ConsumedCapacity</code> objects, one for each table addressed by <code>TransactGetItem</code> objects in the <i>TransactItems</i> parameter. These <code>ConsumedCapacity</code> objects report the read-capacity units consumed by the <code>TransactGetItems</code> call in that table.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.consumed_capacity.is_none()`.
    pub fn consumed_capacity(&self) -> &[crate::types::ConsumedCapacity] {
        self.consumed_capacity.as_deref().unwrap_or_default()
    }
    /// <p>An ordered array of up to 100 <code>ItemResponse</code> objects, each of which corresponds to the <code>TransactGetItem</code> object in the same position in the <i>TransactItems</i> array. Each <code>ItemResponse</code> object contains a Map of the name-value pairs that are the projected attributes of the requested item.</p>
    /// <p>If a requested item could not be retrieved, the corresponding <code>ItemResponse</code> object is Null, or if the requested item has no projected attributes, the corresponding <code>ItemResponse</code> object is an empty Map.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.responses.is_none()`.
    pub fn responses(&self) -> &[crate::types::ItemResponse] {
        self.responses.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for TransactGetItemsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl TransactGetItemsOutput {
    /// Creates a new builder-style object to manufacture [`TransactGetItemsOutput`](crate::operation::transact_get_items::TransactGetItemsOutput).
    pub fn builder() -> crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder {
        crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder::default()
    }
}

/// A builder for [`TransactGetItemsOutput`](crate::operation::transact_get_items::TransactGetItemsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TransactGetItemsOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    pub(crate) responses: ::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>>,
    _request_id: Option<String>,
}
impl TransactGetItemsOutputBuilder {
    /// Appends an item to `consumed_capacity`.
    ///
    /// To override the contents of this collection use [`set_consumed_capacity`](Self::set_consumed_capacity).
    ///
    /// <p>If the <i>ReturnConsumedCapacity</i> value was <code>TOTAL</code>, this is an array of <code>ConsumedCapacity</code> objects, one for each table addressed by <code>TransactGetItem</code> objects in the <i>TransactItems</i> parameter. These <code>ConsumedCapacity</code> objects report the read-capacity units consumed by the <code>TransactGetItems</code> call in that table.</p>
    pub fn consumed_capacity(mut self, input: crate::types::ConsumedCapacity) -> Self {
        let mut v = self.consumed_capacity.unwrap_or_default();
        v.push(input);
        self.consumed_capacity = ::std::option::Option::Some(v);
        self
    }
    /// <p>If the <i>ReturnConsumedCapacity</i> value was <code>TOTAL</code>, this is an array of <code>ConsumedCapacity</code> objects, one for each table addressed by <code>TransactGetItem</code> objects in the <i>TransactItems</i> parameter. These <code>ConsumedCapacity</code> objects report the read-capacity units consumed by the <code>TransactGetItems</code> call in that table.</p>
    pub fn set_consumed_capacity(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>) -> Self {
        self.consumed_capacity = input;
        self
    }
    /// <p>If the <i>ReturnConsumedCapacity</i> value was <code>TOTAL</code>, this is an array of <code>ConsumedCapacity</code> objects, one for each table addressed by <code>TransactGetItem</code> objects in the <i>TransactItems</i> parameter. These <code>ConsumedCapacity</code> objects report the read-capacity units consumed by the <code>TransactGetItems</code> call in that table.</p>
    pub fn get_consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>> {
        &self.consumed_capacity
    }
    /// Appends an item to `responses`.
    ///
    /// To override the contents of this collection use [`set_responses`](Self::set_responses).
    ///
    /// <p>An ordered array of up to 100 <code>ItemResponse</code> objects, each of which corresponds to the <code>TransactGetItem</code> object in the same position in the <i>TransactItems</i> array. Each <code>ItemResponse</code> object contains a Map of the name-value pairs that are the projected attributes of the requested item.</p>
    /// <p>If a requested item could not be retrieved, the corresponding <code>ItemResponse</code> object is Null, or if the requested item has no projected attributes, the corresponding <code>ItemResponse</code> object is an empty Map.</p>
    pub fn responses(mut self, input: crate::types::ItemResponse) -> Self {
        let mut v = self.responses.unwrap_or_default();
        v.push(input);
        self.responses = ::std::option::Option::Some(v);
        self
    }
    /// <p>An ordered array of up to 100 <code>ItemResponse</code> objects, each of which corresponds to the <code>TransactGetItem</code> object in the same position in the <i>TransactItems</i> array. Each <code>ItemResponse</code> object contains a Map of the name-value pairs that are the projected attributes of the requested item.</p>
    /// <p>If a requested item could not be retrieved, the corresponding <code>ItemResponse</code> object is Null, or if the requested item has no projected attributes, the corresponding <code>ItemResponse</code> object is an empty Map.</p>
    pub fn set_responses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>>) -> Self {
        self.responses = input;
        self
    }
    /// <p>An ordered array of up to 100 <code>ItemResponse</code> objects, each of which corresponds to the <code>TransactGetItem</code> object in the same position in the <i>TransactItems</i> array. Each <code>ItemResponse</code> object contains a Map of the name-value pairs that are the projected attributes of the requested item.</p>
    /// <p>If a requested item could not be retrieved, the corresponding <code>ItemResponse</code> object is Null, or if the requested item has no projected attributes, the corresponding <code>ItemResponse</code> object is an empty Map.</p>
    pub fn get_responses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ItemResponse>> {
        &self.responses
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`TransactGetItemsOutput`](crate::operation::transact_get_items::TransactGetItemsOutput).
    pub fn build(self) -> crate::operation::transact_get_items::TransactGetItemsOutput {
        crate::operation::transact_get_items::TransactGetItemsOutput {
            consumed_capacity: self.consumed_capacity,
            responses: self.responses,
            _request_id: self._request_id,
        }
    }
}
