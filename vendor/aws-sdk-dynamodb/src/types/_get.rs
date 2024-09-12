// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies an item and related attribute values to retrieve in a <code>TransactGetItem</code> object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Get {
    /// <p>A map of attribute names to <code>AttributeValue</code> objects that specifies the primary key of the item to retrieve.</p>
    pub key: ::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>,
    /// <p>The name of the table from which to retrieve the specified item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub table_name: ::std::string::String,
    /// <p>A string that identifies one or more attributes of the specified item to retrieve from the table. The attributes in the expression must be separated by commas. If no attribute names are specified, then all attributes of the specified item are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    pub projection_expression: ::std::option::Option<::std::string::String>,
    /// <p>One or more substitution tokens for attribute names in the ProjectionExpression parameter.</p>
    pub expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl Get {
    /// <p>A map of attribute names to <code>AttributeValue</code> objects that specifies the primary key of the item to retrieve.</p>
    pub fn key(&self) -> &::std::collections::HashMap<::std::string::String, crate::types::AttributeValue> {
        &self.key
    }
    /// <p>The name of the table from which to retrieve the specified item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(&self) -> &str {
        use std::ops::Deref;
        self.table_name.deref()
    }
    /// <p>A string that identifies one or more attributes of the specified item to retrieve from the table. The attributes in the expression must be separated by commas. If no attribute names are specified, then all attributes of the specified item are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    pub fn projection_expression(&self) -> ::std::option::Option<&str> {
        self.projection_expression.as_deref()
    }
    /// <p>One or more substitution tokens for attribute names in the ProjectionExpression parameter.</p>
    pub fn expression_attribute_names(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.expression_attribute_names.as_ref()
    }
}
impl Get {
    /// Creates a new builder-style object to manufacture [`Get`](crate::types::Get).
    pub fn builder() -> crate::types::builders::GetBuilder {
        crate::types::builders::GetBuilder::default()
    }
}

/// A builder for [`Get`](crate::types::Get).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBuilder {
    pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) projection_expression: ::std::option::Option<::std::string::String>,
    pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl GetBuilder {
    /// Adds a key-value pair to `key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>A map of attribute names to <code>AttributeValue</code> objects that specifies the primary key of the item to retrieve.</p>
    pub fn key(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        let mut hash_map = self.key.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.key = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects that specifies the primary key of the item to retrieve.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>) -> Self {
        self.key = input;
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects that specifies the primary key of the item to retrieve.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        &self.key
    }
    /// <p>The name of the table from which to retrieve the specified item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    /// This field is required.
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table from which to retrieve the specified item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The name of the table from which to retrieve the specified item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>A string that identifies one or more attributes of the specified item to retrieve from the table. The attributes in the expression must be separated by commas. If no attribute names are specified, then all attributes of the specified item are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    pub fn projection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.projection_expression = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string that identifies one or more attributes of the specified item to retrieve from the table. The attributes in the expression must be separated by commas. If no attribute names are specified, then all attributes of the specified item are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    pub fn set_projection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.projection_expression = input;
        self
    }
    /// <p>A string that identifies one or more attributes of the specified item to retrieve from the table. The attributes in the expression must be separated by commas. If no attribute names are specified, then all attributes of the specified item are returned. If any of the requested attributes are not found, they do not appear in the result.</p>
    pub fn get_projection_expression(&self) -> &::std::option::Option<::std::string::String> {
        &self.projection_expression
    }
    /// Adds a key-value pair to `expression_attribute_names`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in the ProjectionExpression parameter.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.expression_attribute_names.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.expression_attribute_names = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more substitution tokens for attribute names in the ProjectionExpression parameter.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.expression_attribute_names = input;
        self
    }
    /// <p>One or more substitution tokens for attribute names in the ProjectionExpression parameter.</p>
    pub fn get_expression_attribute_names(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.expression_attribute_names
    }
    /// Consumes the builder and constructs a [`Get`](crate::types::Get).
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](crate::types::builders::GetBuilder::key)
    /// - [`table_name`](crate::types::builders::GetBuilder::table_name)
    pub fn build(self) -> ::std::result::Result<crate::types::Get, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Get {
            key: self.key.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field("key", "key was not specified but it is required when building Get")
            })?,
            table_name: self.table_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "table_name",
                    "table_name was not specified but it is required when building Get",
                )
            })?,
            projection_expression: self.projection_expression,
            expression_attribute_names: self.expression_attribute_names,
        })
    }
}