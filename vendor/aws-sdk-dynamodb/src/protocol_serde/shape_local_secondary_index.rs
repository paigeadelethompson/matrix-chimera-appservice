// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_local_secondary_index(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LocalSecondaryIndex,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("IndexName").string(input.index_name.as_str());
    }
    {
        let mut array_1 = object.key("KeySchema").start_array();
        for item_2 in &input.key_schema {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_key_schema_element::ser_key_schema_element(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    if let Some(var_4) = &input.projection {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Projection").start_object();
        crate::protocol_serde::shape_projection::ser_projection(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}