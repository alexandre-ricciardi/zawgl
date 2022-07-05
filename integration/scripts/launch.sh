#!/bin/bash
target/debug/og &
cd integration/gremlin-tests/zawgl-gremlin
mvn clean package -q
