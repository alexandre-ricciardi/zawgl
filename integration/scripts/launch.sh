#!/bin/bash
target/debug/og &
cd integration/tests/app
mvn clean package -q
