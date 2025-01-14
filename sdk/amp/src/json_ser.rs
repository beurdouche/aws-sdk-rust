// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_alert_manager_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAlertManagerDefinitionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1);
    }
    if let Some(var_2) = &input.data {
        object
            .key("data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_2));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_rule_groups_namespace_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRuleGroupsNamespaceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3);
    }
    if let Some(var_4) = &input.data {
        object
            .key("data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_4));
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5);
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9);
            }
        }
        object_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_workspace_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkspaceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.alias {
        object.key("alias").string(var_10);
    }
    if let Some(var_11) = &input.client_token {
        object.key("clientToken").string(var_11);
    }
    if let Some(var_12) = &input.tags {
        let mut object_13 = object.key("tags").start_object();
        for (key_14, value_15) in var_12 {
            {
                object_13.key(key_14).string(value_15);
            }
        }
        object_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_alert_manager_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAlertManagerDefinitionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_16) = &input.client_token {
        object.key("clientToken").string(var_16);
    }
    if let Some(var_17) = &input.data {
        object
            .key("data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_17));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_rule_groups_namespace_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRuleGroupsNamespaceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.client_token {
        object.key("clientToken").string(var_18);
    }
    if let Some(var_19) = &input.data {
        object
            .key("data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_19));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.tags {
        let mut object_21 = object.key("tags").start_object();
        for (key_22, value_23) in var_20 {
            {
                object_21.key(key_22).string(value_23);
            }
        }
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workspace_alias_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkspaceAliasInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.alias {
        object.key("alias").string(var_24);
    }
    if let Some(var_25) = &input.client_token {
        object.key("clientToken").string(var_25);
    }
    Ok(())
}
