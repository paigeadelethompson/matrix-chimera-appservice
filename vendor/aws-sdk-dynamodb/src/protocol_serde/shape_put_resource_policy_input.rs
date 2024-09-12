// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resource_policy_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_resource_policy::PutResourcePolicyInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.policy {
        object.key("Policy").string(var_2.as_str());
    }
    if let Some(var_3) = &input.expected_revision_id {
        object.key("ExpectedRevisionId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.confirm_remove_self_resource_access {
        object.key("ConfirmRemoveSelfResourceAccess").boolean(*var_4);
    }
    Ok(())
}