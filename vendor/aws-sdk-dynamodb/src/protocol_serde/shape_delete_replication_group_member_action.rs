// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_replication_group_member_action(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DeleteReplicationGroupMemberAction,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("RegionName").string(input.region_name.as_str());
    }
    Ok(())
}
