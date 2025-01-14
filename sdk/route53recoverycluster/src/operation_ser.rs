// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_get_routing_control_state(
    input: &crate::input::GetRoutingControlStateInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_routing_control_state_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_routing_control_state(
    input: &crate::input::UpdateRoutingControlStateInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_routing_control_state_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_routing_control_states(
    input: &crate::input::UpdateRoutingControlStatesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_routing_control_states_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
