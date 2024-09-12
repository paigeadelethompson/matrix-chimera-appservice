// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>ListTables</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTablesInput {
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub exclusive_start_table_name: ::std::option::Option<::std::string::String>,
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub limit: ::std::option::Option<i32>,
}
impl ListTablesInput {
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub fn exclusive_start_table_name(&self) -> ::std::option::Option<&str> {
        self.exclusive_start_table_name.as_deref()
    }
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
}
impl ListTablesInput {
    /// Creates a new builder-style object to manufacture [`ListTablesInput`](crate::operation::list_tables::ListTablesInput).
    pub fn builder() -> crate::operation::list_tables::builders::ListTablesInputBuilder {
        crate::operation::list_tables::builders::ListTablesInputBuilder::default()
    }
}

/// A builder for [`ListTablesInput`](crate::operation::list_tables::ListTablesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListTablesInputBuilder {
    pub(crate) exclusive_start_table_name: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
}
impl ListTablesInputBuilder {
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub fn exclusive_start_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.exclusive_start_table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub fn set_exclusive_start_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.exclusive_start_table_name = input;
        self
    }
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    pub fn get_exclusive_start_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.exclusive_start_table_name
    }
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        &self.limit
    }
    /// Consumes the builder and constructs a [`ListTablesInput`](crate::operation::list_tables::ListTablesInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_tables::ListTablesInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_tables::ListTablesInput {
            exclusive_start_table_name: self.exclusive_start_table_name,
            limit: self.limit,
        })
    }
}