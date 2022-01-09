input=${1%.*}
RUST_LOG='info,intermediate_code=trace,machine_code=trace' cargo run -- "$@"
gcc "$input.S" -no-pie -o $input
