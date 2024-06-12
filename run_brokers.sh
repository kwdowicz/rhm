#!/bin/bash

usage() {
    echo "Usage: $0 <IP_ADDRESS> <BASE_PORT> <NUMBER_OF_PROGRAMS>"
    echo "Example: $0 127.0.0.1 5000 3"
    exit 1
}

if [ "$#" -ne 3 ]; then
    usage
fi

IP_ADDRESS=$1
BASE_PORT=$2
NUMBER_OF_PROGRAMS=$3

if [ "$NUMBER_OF_PROGRAMS" -lt 0 ] || [ "$NUMBER_OF_PROGRAMS" -gt 1000 ]; then
    echo "Error: NUMBER_OF_PROGRAMS must be between 0 and 1000"
    usage
fi

run_program() {
    local port=$1
    echo "Running program with IP $IP_ADDRESS and port $port"
    cargo run --bin server "$IP_ADDRESS:$port" &
}

for i in $(seq 0 $((NUMBER_OF_PROGRAMS - 1))); do
    PORT=$((BASE_PORT + i))
    run_program "$PORT"
done

wait
