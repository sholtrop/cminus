input=${1%.*}
RUST_LOG='info,intermediate_code=debug,machine_code=debug' cargo run -- "$@"
gcc "$input.S" -no-pie -o $input
