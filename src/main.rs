use futures::stream::TryStreamExt;
use mongodb::{
    bson::Document,
    options::{
        ClientOptions, Credential, ReadPreference, SelectionCriteria, ServerAddress, TlsOptions,
    },
    Client,
};

const FROM: MongoConfig = MongoConfig {
    s3_bucket_name: "dev1-good-tenant-private",
    region: "us-west-2",
    host: "dev1-platform.cluster-cti10lnrh4rb.us-west-2.docdb.amazonaws.com",
    username: "root",
    password: "cWgSdoCUjSCuylBa",
    db: "dev1-good-form-template",
    collection: "trial-416c6768-10af-4974-9004-6f5164ff8416",
    domain: "s37dev",
    instance: "dev1-platform",
    tenant: "good",
    trial: "416c6768-10af-4974-9004-6f5164ff8416",
};
const TO: MongoConfig = MongoConfig {
    s3_bucket_name: "dev1-good-tenant-private",
    region: "us-west-2",
    host: "dev2-platform.cluster-cti10lnrh4rb.us-west-2.docdb.amazonaws.com",
    username: "root",
    password: "nKuvxmOUuP9ti528",
    db: "dev2-acme-form-template",
    collection: "trial-fcf331c5-aa89-4b9a-af94-513fefc00aa4",
    instance: "dev2-platform",
    domain: "s37dev",
    tenant: "acme",
    trial: "fcf331c5-aa89-4b9a-af94-513fefc00aa4",
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("{}", generate_aws_script());
    // return Ok(()); // this is an early return to the migration does not run

    // Print the state before migration
    println!("FROM");
    print_collection(&FROM).await?;
    println!("TO");
    print_collection(&TO).await?;

    // Query the FROM db and do the transformation on each document
    let from_url = generate_url(&FROM);
    let to_url = generate_url(&TO);
    let transformed_from_docs: Vec<_> = build_connection(&FROM)
        .await?
        .database(FROM.db)
        .collection::<Document>(FROM.collection)
        .find(None, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?
        .iter_mut()
        .map(|doc| {
            println!("----------------");
            println!("{}", doc.get("id").unwrap());
            println!("{}", doc.get("title").unwrap());
            let mut v: Vec<u8> = Vec::new();
            doc.to_writer(&mut v).unwrap();
            // Swap the URLS **first**
            insert_chars_eq(&mut v, &from_url, &to_url);
            // Swap the IDs
            let from_trial = &FROM.trial.chars().map(|c| c as u8).collect::<Vec<_>>();
            let to_trial = &TO.trial.chars().map(|c| c as u8).collect::<Vec<_>>();
            insert_chars_eq(&mut v, &from_trial, &to_trial);
            let modified_doc = Document::from_reader(std::io::Cursor::new(v)).unwrap();
            modified_doc
        })
        .collect();

    // Write the results of the FROM db + transformation to TO db
    build_connection(&TO)
        .await?
        .database(TO.db)
        .collection(TO.collection)
        .insert_many(transformed_from_docs, None) //   <- insert the forms
        .await
        .expect("insertion failure");

    // Print the state after migration
    println!("TO AFTER");
    print_collection(&TO).await?;
    Ok(())
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

struct MongoConfig<'a> {
    s3_bucket_name: &'a str,
    region: &'a str,
    host: &'a str,
    username: &'a str,
    password: &'a str,
    db: &'a str,
    collection: &'a str,
    instance: &'a str,
    tenant: &'a str,
    trial: &'a str,
    domain: &'a str,
}

fn generate_s3_uri(cfg: MongoConfig) -> String {
    format!("s3://{}/form-template/{}/", cfg.s3_bucket_name, cfg.trial)
}

fn generate_aws_script() -> String {
    format!(
        r#"
#! /bin/bash
# This file is generated. Maybe don't modify it.
aws s3 cp {} {} --recursive --source-region {} --region {}
"#,
        generate_s3_uri(FROM),
        generate_s3_uri(TO),
        FROM.region,
        TO.region
    )
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
        .try_collect::<Vec<Document>>()
        .await?
    {
        println!("{} {}", f.get("id").unwrap(), f.get("title").unwrap(),);
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
