cargo build --release  --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/proxy-handler .
func azure functionapp publish hyper-proxy
