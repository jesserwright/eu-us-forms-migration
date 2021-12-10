use axum::extract::Form;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::{
        ClientOptions, Credential, ReadPreference, SelectionCriteria, ServerAddress, TlsOptions,
    },
    results::DeleteResult,
    Client,
};
// use s37::FormTemplate;

struct MongoConfig<'a> {
    host: &'a str,
    username: &'a str,
    password: &'a str,
    db: &'a str,
    collection: &'a str,
}

async fn print_collection_names(
    mongo_config: &MongoConfig<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    for x in build_connection(&mongo_config)
        .await?
        .database(mongo_config.db)
        .list_collection_names(None)
        .await?
    {
        println!("{}", x);
    }
    Ok(())
}

async fn print_from_to_templates(
    from: &MongoConfig<'_>,
    to: &MongoConfig<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    for f in build_connection(&from)
        .await?
        .database(from.db)
        .collection::<FormTemplate>(from.collection)
        .find(None, None) //  <- get the forms
        .await?
        .try_collect::<Vec<_>>()
        .await?
    {
        println!("{:?}", f);
    }
    println!("--------");
    for f in build_connection(&to)
        .await?
        .database(to.db)
        .collection::<FormTemplate>(to.collection)
        .find(None, None) //  <- get the forms
        .await?
        .try_collect::<Vec<_>>()
        .await?
    {
        println!("{:?}", f);
    }

    Ok(())
}

async fn do_copy(
    from: &MongoConfig<'_>,
    to: &MongoConfig<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    let forms = build_connection(&from)
        .await?
        .database(from.db)
        .collection(from.collection)
        .find(None, None) //  <- get the forms
        .await?
        .try_collect::<Vec<FormTemplate>>()
        .await?;
    build_connection(&to)
        .await?
        .database(to.db)
        .collection(to.collection)
        .insert_many(forms, None) //   <- insert the forms
        .await?;
    Ok(())
}

async fn print_collection(
    mongo_config: &MongoConfig<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    for f in build_connection(&mongo_config)
        .await?
        .database(mongo_config.db)
        .collection(mongo_config.collection)
        .find(None, None) //  <- get the forms
        .await?
        .try_collect::<Vec<FormTemplate>>()
        .await?
    {
        println!("{} | {} | {}", f.id, f.title, f.form_oid);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let from = MongoConfig {
        host: "dev1-platform.cluster-cti10lnrh4rb.us-west-2.docdb.amazonaws.com",
        username: "root",
        password: "cWgSdoCUjSCuylBa",
        db: "dev1-good-form-template",
        collection: "trial-416c6768-10af-4974-9004-6f5164ff8416",
    };
    let to = MongoConfig {
        host: "dev2-platform.cluster-cti10lnrh4rb.us-west-2.docdb.amazonaws.com",
        username: "root",
        password: "nKuvxmOUuP9ti528",
        db: "dev2-acme-form-template",
        collection: "trial-fcf331c5-aa89-4b9a-af94-513fefc00aa4",
    };

    // println!("From");
    // print_collection(&from).await?;
    // println!("To");
    // print_collection(&to).await?;
    // println!("FROM:");
    // print_collection(&from).await?;
    // println!("\nTO:");
    // print_collection(&to).await?;

    // // delete
    // build_connection(&from)
    //     .await?
    //     .database(from.db)
    //     .collection::<FormTemplate>(from.collection)
    //     .delete_one(doc! {"id": "35490eca-e176-46f1-a74a-0df473d61a99"}, None)
    //     .await?;

    println!("FROM");
    print_collection(&from).await?;
    println!("TO");
    print_collection(&to).await?;

    // do_copy(&from, &to).await?;

    // println!("TO AFTER");
    // print_collection(&to).await?;

    Ok(())
}

async fn build_connection(config: &MongoConfig<'_>) -> Result<Client, Box<dyn std::error::Error>> {
    let tls = TlsOptions::builder()
        .ca_file_path(Some("cert.pem".into()))
        .build();
    let credential = Credential::builder()
        .username(Some(config.username.into()))
        .password(Some(config.password.into()))
        .build();
    let options = ClientOptions::builder()
        .hosts(vec![ServerAddress::Tcp {
            host: config.host.into(),
            port: None,
        }])
        .direct_connection(Some(true))
        .credential(Some(credential))
        .tls(tls)
        .selection_criteria(Some(SelectionCriteria::ReadPreference(
            ReadPreference::Primary,
        )))
        .build();
    let client = Client::with_options(options)?;
    return Ok(client);
}

use mongodb::bson;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// checking for type correctness with the below structs would mean reading, writing, and deleting, then verifying that the result is the same.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTemplate {
    pub id: String,
    pub title: String,
    // trial_id: String,
    // created_by: String,
    // updated_by: String,
    // title_to_search: String,
    pub form_oid: String,
    // form_oid_to_search: String,
    // created_on: bson::DateTime,
    // last_updated_on: bson::DateTime,
    // tags: Vec<Tag>,
    // status: String,
    // form_type: String,
    // version: i64,
    // template: Template,
    // settings: Vec<Setting>,
    // pdf_templates: PdfTemplates,
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
