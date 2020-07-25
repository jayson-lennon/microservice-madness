clean:
    rm fizzbuzz/src/bin/svc-*

build:
    cargo build && cargo build

run-service name:
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
