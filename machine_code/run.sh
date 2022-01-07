input=${1%.*}
RUST_LOG='info,machine_code=trace' cargo run -- "$@"
gcc "$input.S" -no-pie -o $input
