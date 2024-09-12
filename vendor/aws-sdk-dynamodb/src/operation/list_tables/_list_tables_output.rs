// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>ListTables</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTablesOutput {
    /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p>
    /// <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
    pub table_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p>
    /// <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
    pub last_evaluated_table_name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTablesOutput {
    /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p>
    /// <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.table_names.is_none()`.
    pub fn table_names(&self) -> &[::std::string::String] {
        self.table_names.as_deref().unwrap_or_default()
    }
    /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p>
    /// <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
    pub fn last_evaluated_table_name(&self) -> ::std::option::Option<&str> {
        self.last_evaluated_table_name.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListTablesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTablesOutput {
    /// Creates a new builder-style object to manufacture [`ListTablesOutput`](crate::operation::list_tables::ListTablesOutput).
    pub fn builder() -> crate::operation::list_tables::builders::ListTablesOutputBuilder {
        crate::operation::list_tables::builders::ListTablesOutputBuilder::default()
    }
}

/// A builder for [`ListTablesOutput`](crate::operation::list_tables::ListTablesOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListTablesOutputBuilder {
    pub(crate) table_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) last_evaluated_table_name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTablesOutputBuilder {
    /// Appends an item to `table_names`.
    ///
    /// To override the contents of this collection use [`set_table_names`](Self::set_table_names).
    ///
    /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p>
    /// <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
    pub fn table_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.table_names.unwrap_or_default();
        v.push(input.into());
        self.table_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p>
    /// <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
    pub fn set_table_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.table_names = input;
        self
    }
    /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p>
    /// <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
    pub fn get_table_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.table_names
    }
    /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p>
    /// <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
    pub fn last_evaluated_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.last_evaluated_table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p>
    /// <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
    pub fn set_last_evaluated_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.last_evaluated_table_name = input;
        self
    }
    /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p>
    /// <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
    pub fn get_last_evaluated_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.last_evaluated_table_name
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListTablesOutput`](crate::operation::list_tables::ListTablesOutput).
    pub fn build(self) -> crate::operation::list_tables::ListTablesOutput {
        crate::operation::list_tables::ListTablesOutput {
            table_names: self.table_names,
            last_evaluated_table_name: self.last_evaluated_table_name,
            _request_id: self._request_id,
        }
    }
}
