# EU-US-FORMS-MIGRATION
1. This is a rust project with the source file at `src/main.rs`
1. This assumes a UNIX-like shell.
1. Download rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` or see https://www.rust-lang.org/tools/install
1. A valid `cert.pem` file must be present in the root directory to connect to MongoDB. It must be named `cert.pem`.
1. Ensure appropriate environment variables are in the `FROM` and `TO` structures at the top of `src/main.rs` file. Both the S3 and MongoDB migrations depend on these values.
1. To run the MongoDB migration, run `cargo r > output.txt` in the root directory. Output of the migration will be in `output.txt`
1. To run the S3 migration: Have the AWS CLI installed and configured, uncomment the first *two* lines within the `main` function, and run `cargo r > copy_s3.sh && chmod+x copy_s3.sh`. This writes an executable script to the root folder called `copy_s3.sh`. Run this with `./copy_s3.sh`.
