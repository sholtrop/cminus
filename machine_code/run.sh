RUST_LOG='info,machine_code=trace' cargo run -- "$@"
gcc test_cminus.S -no-pie -o test 