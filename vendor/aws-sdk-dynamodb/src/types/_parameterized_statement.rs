// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a PartiQL statement that uses parameters.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ParameterizedStatement {
    /// <p>A PartiQL statement that uses parameters.</p>
    pub statement: ::std::string::String,
    /// <p>The parameter values.</p>
    pub parameters: ::std::option::Option<::std::vec::Vec<crate::types::AttributeValue>>,
    /// <p>An optional parameter that returns the item attributes for a PartiQL <code>ParameterizedStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub return_values_on_condition_check_failure: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
}
impl ParameterizedStatement {
    /// <p>A PartiQL statement that uses parameters.</p>
    pub fn statement(&self) -> &str {
        use std::ops::Deref;
        self.statement.deref()
    }
    /// <p>The parameter values.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.parameters.is_none()`.
    pub fn parameters(&self) -> &[crate::types::AttributeValue] {
        self.parameters.as_deref().unwrap_or_default()
    }
    /// <p>An optional parameter that returns the item attributes for a PartiQL <code>ParameterizedStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn return_values_on_condition_check_failure(&self) -> ::std::option::Option<&crate::types::ReturnValuesOnConditionCheckFailure> {
        self.return_values_on_condition_check_failure.as_ref()
    }
}
impl ParameterizedStatement {
    /// Creates a new builder-style object to manufacture [`ParameterizedStatement`](crate::types::ParameterizedStatement).
    pub fn builder() -> crate::types::builders::ParameterizedStatementBuilder {
        crate::types::builders::ParameterizedStatementBuilder::default()
    }
}

/// A builder for [`ParameterizedStatement`](crate::types::ParameterizedStatement).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ParameterizedStatementBuilder {
    pub(crate) statement: ::std::option::Option<::std::string::String>,
    pub(crate) parameters: ::std::option::Option<::std::vec::Vec<crate::types::AttributeValue>>,
    pub(crate) return_values_on_condition_check_failure: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
}
impl ParameterizedStatementBuilder {
    /// <p>A PartiQL statement that uses parameters.</p>
    /// This field is required.
    pub fn statement(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.statement = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A PartiQL statement that uses parameters.</p>
    pub fn set_statement(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.statement = input;
        self
    }
    /// <p>A PartiQL statement that uses parameters.</p>
    pub fn get_statement(&self) -> &::std::option::Option<::std::string::String> {
        &self.statement
    }
    /// Appends an item to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The parameter values.</p>
    pub fn parameters(mut self, input: crate::types::AttributeValue) -> Self {
        let mut v = self.parameters.unwrap_or_default();
        v.push(input);
        self.parameters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The parameter values.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeValue>>) -> Self {
        self.parameters = input;
        self
    }
    /// <p>The parameter values.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AttributeValue>> {
        &self.parameters
    }
    /// <p>An optional parameter that returns the item attributes for a PartiQL <code>ParameterizedStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn return_values_on_condition_check_failure(mut self, input: crate::types::ReturnValuesOnConditionCheckFailure) -> Self {
        self.return_values_on_condition_check_failure = ::std::option::Option::Some(input);
        self
    }
    /// <p>An optional parameter that returns the item attributes for a PartiQL <code>ParameterizedStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn set_return_values_on_condition_check_failure(
        mut self,
        input: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
    ) -> Self {
        self.return_values_on_condition_check_failure = input;
        self
    }
    /// <p>An optional parameter that returns the item attributes for a PartiQL <code>ParameterizedStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure> {
        &self.return_values_on_condition_check_failure
    }
    /// Consumes the builder and constructs a [`ParameterizedStatement`](crate::types::ParameterizedStatement).
    /// This method will fail if any of the following fields are not set:
    /// - [`statement`](crate::types::builders::ParameterizedStatementBuilder::statement)
    pub fn build(self) -> ::std::result::Result<crate::types::ParameterizedStatement, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ParameterizedStatement {
            statement: self.statement.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "statement",
                    "statement was not specified but it is required when building ParameterizedStatement",
                )
            })?,
            parameters: self.parameters,
            return_values_on_condition_check_failure: self.return_values_on_condition_check_failure,
        })
    }
}