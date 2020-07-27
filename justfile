# set log level to: trace, debug, warn, info
clean:
    rm fizzbuzz/src/bin/svc-*

build:
    cargo build && cargo build

build-release:
    cargo build --release

run-service name:
    @echo Starting {{name}}...
    @./target/debug/{{name}}

run-service-release name:
    @echo Starting {{name}}...
    @./target/release/{{name}}

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

run-services-release: build-release
    #!/bin/bash
    # Propagate CTRL+C to all background processes.
    trap "exit" INT TERM ERR
    trap "kill 0" EXIT

    just run-service broker &
    echo Waiting for broker to start up...
    sleep 1

    for svc in `ls fizzbuzz/src/bin | rg svc- | sed 's/\.rs//g'`;
        do just run-service-release $svc &
    done

    wait
