use dotenv::dotenv;
use envy;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::Document,
    options::{
        ClientOptions, Credential, ReadPreference, SelectionCriteria, ServerAddress, TlsOptions,
    },
    Client,
};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let from = envy::prefixed("FROM_").from_env::<MongoConfig>()?;
    let to = envy::prefixed("TO_").from_env::<MongoConfig>()?;

    println!("FROM");
    print_collection(&from).await?;
    println!("TO");
    print_collection(&to).await?;
    let modified_from_docs: Vec<_> = build_connection(&from)
        .await?
        .database(&from.db)
        .collection::<Document>(&from.collection)
        .find(None, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?
        .iter_mut()
        .map(|doc| {
            let new_doc = modify_doc(&from, &to, doc);
            return new_doc;
        })
        .collect();

    build_connection(&to)
        .await?
        .database(&to.db)
        .collection(&to.collection)
        .insert_many(modified_from_docs, None)
        .await?;

    println!("TO AFTER");
    print_collection(&to).await?;

    let s3_cmd = Command::new("aws")
        .args([
            "s3",
            "cp",
            &format!("s3://{}/form-template/{}/", &from.s3_bucket, &from.trial),
            &format!("s3://{}/form-template/{}/", &to.s3_bucket, &to.trial),
            "--recursive",
            "--source-region",
            &from.region,
            "--region",
            &to.region,
        ])
        .output()
        .await?;
    let s3_cmd_output = s3_cmd.stdout.iter().map(|u| *u as char).collect::<String>();
    println!("S3 command output:\n{}", s3_cmd_output);

    Ok(())
}

fn print_doc(doc: &Document) {
    println!("{} {}", doc.get("id").unwrap(), doc.get("title").unwrap());
}

fn modify_doc(from: &MongoConfig, to: &MongoConfig, doc: &mut Document) -> Document {
    println!("----------------");
    print_doc(&doc);
    let mut v: Vec<u8> = Vec::new();
    doc.to_writer(&mut v).unwrap();
    // Swap the URLS first
    let from_url = generate_url(from);
    let to_url = generate_url(to);
    insert_chars_eq(&mut v, &from_url, &to_url);
    // Swap the IDs
    // doc.get_mut("id").unwrap() = to.trial; // <- better way of doing this by getting a mutable ref?
    let from_trial = &from.trial.chars().map(|c| c as u8).collect::<Vec<_>>();
    let to_trial = &to.trial.chars().map(|c| c as u8).collect::<Vec<_>>();
    insert_chars_eq(&mut v, &from_trial, &to_trial);
    let modified_doc = Document::from_reader(std::io::Cursor::new(v)).unwrap();
    modified_doc
}

// given `a` includes a slice `b`, replace `b` with `c`
fn insert_chars_eq(a: &mut [u8], b: &[u8], c: &[u8]) {
    let mut idx = 0;
    while idx < a.len() - b.len() {
        let sl = &mut a[idx..b.len() + idx];
        if sl == b {
            println!("replacing");
            println!("{}", b.iter().map(|c| *c as char).collect::<String>());
            println!("with");
            println!("{}", c.iter().map(|c| *c as char).collect::<String>());
            for (i, x) in sl.iter_mut().enumerate() {
                *x = c[i];
            }
        }
        idx = idx + 1;
    }
}

#[derive(Deserialize, Debug)]
struct MongoConfig {
    s3_bucket: String,
    region: String,
    host: String,
    username: String,
    password: String,
    db: String,
    collection: String,
    instance: String,
    tenant: String,
    trial: String,
    domain: String,
}

async fn print_collection(mongo_config: &MongoConfig) -> Result<(), Box<dyn std::error::Error>> {
    for doc in build_connection(&mongo_config)
        .await?
        .database(&mongo_config.db)
        .collection(&mongo_config.collection)
        .find(None, None)
        .await?
        .try_collect::<Vec<Document>>()
        .await?
    {
        print_doc(&doc)
    }

    Ok(())
}

fn generate_url(cfg: &MongoConfig) -> Vec<u8> {
    let s = format!(
        "https://{}.{}.{}.com/api/v1/platform/form/trials/{}",
        cfg.tenant, cfg.instance, cfg.domain, cfg.trial
    );
    s.chars().map(|c| c as u8).collect::<Vec<_>>()
}

async fn build_connection(config: &MongoConfig) -> Result<Client, Box<dyn std::error::Error>> {
    let tls = TlsOptions::builder()
        .ca_file_path(Some("cert.pem".into()))
        .build();
    let credential = Credential::builder()
        .username(Some(config.username.clone()))
        .password(Some(config.password.clone()))
        .build();
    let options = ClientOptions::builder()
        .hosts(vec![ServerAddress::Tcp {
            host: config.host.clone(),
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
// fn generate_aws_script(from: &MongoConfig, to: &MongoConfig) -> String {
//     format!(
//         r#"
// #! /bin/bash
// # This file is generated. Maybe don't modify it.
// aws s3 cp {} {} --recursive --source-region {} --region {}
// "#,
//         generate_s3_uri(from),
//         generate_s3_uri(to),
//         from.region,
//         to.region
//     )
// }

#[test]
fn test_insert_chars() {
    let mut a = ['x', 'n', 'w', 'p', 'a', 'n', 'f', 'x', 'e']
        .iter()
        .map(|c| *c as u8)
        .collect::<Vec<_>>();
    let b = ['n', 'f', 'x'].iter().map(|c| *c as u8).collect::<Vec<_>>();
    let c = ['o', 'o', 'o'].iter().map(|c| *c as u8).collect::<Vec<_>>();
    insert_chars_eq(&mut a, &b, &c);

    assert_eq!(a[5], 'o' as u8);
    assert_eq!(a[6], 'o' as u8);
    assert_eq!(a[7], 'o' as u8);
}

#[test]
fn doc_ser_de() {
    use mongodb::bson;
    let mut v: Vec<u8> = Vec::new();
    let doc = bson::doc! {"id": "hey"};

    doc.to_writer(&mut v).unwrap();
    let id = "id".chars().map(|c| c as u8).collect::<Vec<_>>();
    let xy = "xy".chars().map(|c| c as u8).collect::<Vec<_>>();
    insert_chars_eq(&mut v, &id, &xy);
    let doc2 = Document::from_reader(std::io::Cursor::new(v)).unwrap();

    assert_eq!(bson::doc! {"xy": "hey"}, doc2);
}

// Delete many
// let ids = [
//     "f5be876f-112f-470e-a6c2-645a93850f72",
//     "91522fd2-1574-4cec-93fb-5e7abe6eda2b",
//     "f960f828-eaea-4022-a8db-d4ebc07ac7da",
//     "6abdde6b-8850-4e1f-927e-25b536b0c10f",
//     "8c429c4c-3b6d-411d-8f98-3f0badd9b228",
//     "2cf59d34-4c78-430b-83c0-cee0b50a5b74",
// ];
// let c = build_connection(&to)
//     .await?
//     .database(to.db)
//     .collection::<Document>(to.collection);
// for id in ids {
//     c.delete_one(mongodb::bson::doc! {"id": id}, None).await?;
// }
// print_collection(&from).await?;
// println!("---------");
// print_collection(&to).await?;
// return Ok(());

// // "move only these ids"
// let ids = [
//     "1e7a2fec-e692-4454-8c79-ff0b99b705ea",
//     "f3cfe220-b6e0-4901-9378-ad9f783302db",
//     "2cf59d34-4c78-430b-83c0-cee0b50a5b74",
//     "0065850f-1aee-4b42-869b-ef2c3642cf8e",
//     "d541f46c-7185-4b14-b40c-da2f8be08a4d",
//     "399a1461-bca1-4e88-982a-2236073560f3",
//     "8c429c4c-3b6d-411d-8f98-3f0badd9b228",
// ];

// let db_tmp = build_connection(&from)
//     .await?
//     .database(&from.db)
//     .collection::<Document>(&from.collection);
// let mut to_move: Vec<Document> = Vec::new();

// for id in ids {
//     let t = db_tmp
//         .find_one(mongodb::bson::doc! {"id": id}, None)
//         .await?
//         .unwrap();
//     to_move.push(t);
// }

// let modified_from_docs: Vec<_> = to_move
//     .iter_mut()
//     .map(|doc| {
//         print_doc(&doc);
//         let new_doc = modify_doc(&from, &to, doc);
//         return new_doc;
//     }).collect();

use mongodb::bson;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTemplate {
    id: String,
    title: String,
    trial_id: String,
    created_by: String,
    updated_by: String,
    title_to_search: String,
    form_oid: String,
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
