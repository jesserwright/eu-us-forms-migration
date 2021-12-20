- Install the Rust programming language toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- A valid `cert.pem` file must be present in the current for MongoDB connection.
- `aws` must be in path, configured with relevant permissions.
- Run `cargo r` to perform the migration (both mongo and s3 migrations are done with this command).
- A log of changes made during the migration are streamed to STDOUT
