use serde::{Deserialize, Serialize};
use serde_json::Value;
// checking for type correctness with the below structs would mean reading, writing, and deleting, then verifying that the result is the same.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTemplate {
    #[serde(rename = "_id")]
    id: Id,
    #[serde(rename = "id")]
    id2: String,
    trial_id: String,
    created_by: String,
    updated_by: String,
    title: String,
    title_to_search: String,
    created_on: CreatedOn,
    last_updated_on: LastUpdatedOn,
    tags: Vec<Value>,
    status: String,
    form_type: String,
    template: Template,
    settings: Vec<Setting>,
    pdf_templates: PdfTemplates,
    version: i64,
    form_oid: String,
    form_oid_to_search: String,
}

// if $oid is db specific... then is there anything special to consider?
#[derive(Debug, Serialize, Deserialize)]
struct Id {
    #[serde(rename = "$oid")]
    oid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatedOn {
    #[serde(rename = "$date")]
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LastUpdatedOn {
    #[serde(rename = "$date")]
    date: String,
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
    date: Date,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension {
    url: String,
    value_coding: ValueCoding,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueCoding {
    extension: Vec<Extension2>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension2 {
    url: String,
    value_coding: ValueCoding2,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueCoding2 {
    extension: Vec<Extension3>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension3 {
    url: String,
    value_meta: Option<ValueMeta>,
    value_coding: Option<ValueCoding3>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueMeta {
    tag: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag {
    #[serde(default)]
    extension: Vec<Extension4>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension4 {
    url: String,
    value_string: Option<String>,
    value_boolean: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueCoding3 {
    extension: Vec<Extension5>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension5 {
    url: String,
    value_meta: ValueMeta2,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueMeta2 {
    tag: Vec<Tag2>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag2 {
    #[serde(default)]
    extension: Vec<Extension6>,
    code: String,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension6 {
    url: String,
    value_meta: Option<ValueMeta3>,
    value_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueMeta3 {
    tag: Vec<Tag3>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag3 {
    #[serde(default)]
    extension: Vec<Extension7>,
    code: String,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension7 {
    url: String,
    value_boolean: Option<bool>,
    value_string: Option<String>,
    value_meta: Option<ValueMeta4>,
    value_integer: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueMeta4 {
    tag: Vec<Tag4>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag4 {
    extension: Vec<Extension8>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension8 {
    url: String,
    value_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Date {
    #[serde(rename = "$date")]
    date: String,
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
    items: Vec<Item2>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item2 {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PermissionsFieldAccess {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    items: Vec<Item3>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item3 {
    key: String,
    value: String,
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
