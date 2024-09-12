// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents an attribute for describing the schema for the table and indexes.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AttributeDefinition {
    /// <p>A name for the attribute.</p>
    pub attribute_name: ::std::string::String,
    /// <p>The data type for the attribute, where:</p>
    /// <ul>
    /// <li>
    /// <p><code>S</code> - the attribute is of type String</p></li>
    /// <li>
    /// <p><code>N</code> - the attribute is of type Number</p></li>
    /// <li>
    /// <p><code>B</code> - the attribute is of type Binary</p></li>
    /// </ul>
    pub attribute_type: crate::types::ScalarAttributeType,
}
impl AttributeDefinition {
    /// <p>A name for the attribute.</p>
    pub fn attribute_name(&self) -> &str {
        use std::ops::Deref;
        self.attribute_name.deref()
    }
    /// <p>The data type for the attribute, where:</p>
    /// <ul>
    /// <li>
    /// <p><code>S</code> - the attribute is of type String</p></li>
    /// <li>
    /// <p><code>N</code> - the attribute is of type Number</p></li>
    /// <li>
    /// <p><code>B</code> - the attribute is of type Binary</p></li>
    /// </ul>
    pub fn attribute_type(&self) -> &crate::types::ScalarAttributeType {
        &self.attribute_type
    }
}
impl AttributeDefinition {
    /// Creates a new builder-style object to manufacture [`AttributeDefinition`](crate::types::AttributeDefinition).
    pub fn builder() -> crate::types::builders::AttributeDefinitionBuilder {
        crate::types::builders::AttributeDefinitionBuilder::default()
    }
}

/// A builder for [`AttributeDefinition`](crate::types::AttributeDefinition).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AttributeDefinitionBuilder {
    pub(crate) attribute_name: ::std::option::Option<::std::string::String>,
    pub(crate) attribute_type: ::std::option::Option<crate::types::ScalarAttributeType>,
}
impl AttributeDefinitionBuilder {
    /// <p>A name for the attribute.</p>
    /// This field is required.
    pub fn attribute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.attribute_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the attribute.</p>
    pub fn set_attribute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.attribute_name = input;
        self
    }
    /// <p>A name for the attribute.</p>
    pub fn get_attribute_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.attribute_name
    }
    /// <p>The data type for the attribute, where:</p>
    /// <ul>
    /// <li>
    /// <p><code>S</code> - the attribute is of type String</p></li>
    /// <li>
    /// <p><code>N</code> - the attribute is of type Number</p></li>
    /// <li>
    /// <p><code>B</code> - the attribute is of type Binary</p></li>
    /// </ul>
    /// This field is required.
    pub fn attribute_type(mut self, input: crate::types::ScalarAttributeType) -> Self {
        self.attribute_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data type for the attribute, where:</p>
    /// <ul>
    /// <li>
    /// <p><code>S</code> - the attribute is of type String</p></li>
    /// <li>
    /// <p><code>N</code> - the attribute is of type Number</p></li>
    /// <li>
    /// <p><code>B</code> - the attribute is of type Binary</p></li>
    /// </ul>
    pub fn set_attribute_type(mut self, input: ::std::option::Option<crate::types::ScalarAttributeType>) -> Self {
        self.attribute_type = input;
        self
    }
    /// <p>The data type for the attribute, where:</p>
    /// <ul>
    /// <li>
    /// <p><code>S</code> - the attribute is of type String</p></li>
    /// <li>
    /// <p><code>N</code> - the attribute is of type Number</p></li>
    /// <li>
    /// <p><code>B</code> - the attribute is of type Binary</p></li>
    /// </ul>
    pub fn get_attribute_type(&self) -> &::std::option::Option<crate::types::ScalarAttributeType> {
        &self.attribute_type
    }
    /// Consumes the builder and constructs a [`AttributeDefinition`](crate::types::AttributeDefinition).
    /// This method will fail if any of the following fields are not set:
    /// - [`attribute_name`](crate::types::builders::AttributeDefinitionBuilder::attribute_name)
    /// - [`attribute_type`](crate::types::builders::AttributeDefinitionBuilder::attribute_type)
    pub fn build(self) -> ::std::result::Result<crate::types::AttributeDefinition, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AttributeDefinition {
            attribute_name: self.attribute_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "attribute_name",
                    "attribute_name was not specified but it is required when building AttributeDefinition",
                )
            })?,
            attribute_type: self.attribute_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "attribute_type",
                    "attribute_type was not specified but it is required when building AttributeDefinition",
                )
            })?,
        })
    }
}
