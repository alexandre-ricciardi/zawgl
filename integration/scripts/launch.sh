#!/bin/bash
target/debug/og &
cd integration/tests/one-graph-gremlin
mvn clean package -q
