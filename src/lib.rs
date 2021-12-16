
use mongodb::bson;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// checking for type correctness with the below structs would mean reading, writing, and deleting, then verifying that the result is the same.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTemplate {
    id: String,
    title: String,
    trial_id: String,
    created_by: String,
    updated_by: String,
    title_to_search: String,
    pub form_oid: String,
    form_oid_to_search: String,
    created_on: bson::DateTime,
    last_updated_on: bson::DateTime,
    tags: Vec<Tag>,
    status: String,
    form_type: String,
    version: i64,
    template: Template,
    settings: Vec<Setting>,
    pdf_templates: PdfTemplates,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Template {
    resource_type: String,
    id: String,
    extension: Vec<Extension>,
    url: String,
    title: String,
    status: String,
    date: bson::DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueCoding {
    extension: Vec<Extension>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension {
    url: String,
    value_meta: Option<ValueMeta>,
    value_coding: Option<ValueCoding>,
    value_string: Option<String>,
    value_boolean: Option<bool>,
    value_integer: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueMeta {
    tag: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    #[serde(default)]
    extension: Vec<Extension>,
    code: String,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Setting {
    id: String,
    label: String,
    roles: Roles,
    permissions_form_access: PermissionsFormAccess,
    permissions_field_access: PermissionsFieldAccess,
    elements: Vec<Element>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Roles {
    id: String,
    label: String,
    #[serde(rename = "type")]
    type_field: String,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PermissionsFormAccess {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PermissionsFieldAccess {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Element {
    id: String,
    title: String,
    description: String,
    section: Section,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Section {
    id: String,
    title: Option<String>,
    description: Option<String>,
    entries: Vec<Entry>,
    signoff_ordering: Option<SignoffOrdering>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    role_permissions: Vec<RolePermission>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RolePermission {
    role: Role,
    permission: Option<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Role {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Permission {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignoffOrdering {
    id: String,
    hidden: bool,
    label: String,
    disabled: bool,
    #[serde(rename = "type")]
    type_field: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PdfTemplates {
    id: String,
    name: String,
    description: String,
    #[serde(rename = "type")]
    type_field: String,
    entries: Vec<Value>,
    accept: Vec<String>,
}
