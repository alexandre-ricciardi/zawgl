#!/bin/bash
target/debug/og &
cd integration/gremlin-tests/one-graph-gremlin
mvn clean package -q
