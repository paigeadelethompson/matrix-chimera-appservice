// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource from which the policy will be removed. The resources you can specify include tables and streams. If you remove the policy of a table, it will also remove the permissions for the table's indexes defined in that policy document. This is because index permissions are defined in the table's policy.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>A string value that you can use to conditionally delete your policy. When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, the request will fail and return a <code>PolicyNotFoundException</code>.</p>
    pub expected_revision_id: ::std::option::Option<::std::string::String>,
}
impl DeleteResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource from which the policy will be removed. The resources you can specify include tables and streams. If you remove the policy of a table, it will also remove the permissions for the table's indexes defined in that policy document. This is because index permissions are defined in the table's policy.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>A string value that you can use to conditionally delete your policy. When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, the request will fail and return a <code>PolicyNotFoundException</code>.</p>
    pub fn expected_revision_id(&self) -> ::std::option::Option<&str> {
        self.expected_revision_id.as_deref()
    }
}
impl DeleteResourcePolicyInput {
    /// Creates a new builder-style object to manufacture [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
    pub fn builder() -> crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder {
        crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder::default()
    }
}

/// A builder for [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteResourcePolicyInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) expected_revision_id: ::std::option::Option<::std::string::String>,
}
impl DeleteResourcePolicyInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource from which the policy will be removed. The resources you can specify include tables and streams. If you remove the policy of a table, it will also remove the permissions for the table's indexes defined in that policy document. This is because index permissions are defined in the table's policy.</p>
    /// This field is required.
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource from which the policy will be removed. The resources you can specify include tables and streams. If you remove the policy of a table, it will also remove the permissions for the table's indexes defined in that policy document. This is because index permissions are defined in the table's policy.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource from which the policy will be removed. The resources you can specify include tables and streams. If you remove the policy of a table, it will also remove the permissions for the table's indexes defined in that policy document. This is because index permissions are defined in the table's policy.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p>A string value that you can use to conditionally delete your policy. When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, the request will fail and return a <code>PolicyNotFoundException</code>.</p>
    pub fn expected_revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string value that you can use to conditionally delete your policy. When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, the request will fail and return a <code>PolicyNotFoundException</code>.</p>
    pub fn set_expected_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_revision_id = input;
        self
    }
    /// <p>A string value that you can use to conditionally delete your policy. When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, the request will fail and return a <code>PolicyNotFoundException</code>.</p>
    pub fn get_expected_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_revision_id
    }
    /// Consumes the builder and constructs a [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_resource_policy::DeleteResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::delete_resource_policy::DeleteResourcePolicyInput {
            resource_arn: self.resource_arn,
            expected_revision_id: self.expected_revision_id,
        })
    }
}