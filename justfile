# set log level to: trace, debug, warn, info
clean:
    rm fizzbuzz/src/bin/svc-*

build:
    cargo build && cargo build

run-service name:
    @printf '\n'
    ./target/debug/{{name}}

run-services: build
    #!/bin/bash
    # Propagate CTRL+C to all background processes.
    trap "exit" INT TERM ERR
    trap "kill 0" EXIT

    just run-service broker &
    echo Waiting for broker to start up...
    sleep 1

    for svc in `ls fizzbuzz/src/bin | rg svc- | sed 's/\.rs//g'`;
        do just run-service $svc &
    done

    wait
