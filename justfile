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
    sleep 1
    just run-service svc-logger &

    for svc in `ls fizzbuzz/src/bin | rg svc- | sed 's/\.rs//g'`;
        do just run-service $svc &
    done

    wait
