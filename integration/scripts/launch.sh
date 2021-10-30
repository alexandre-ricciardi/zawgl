#!/bin/bash

target/debug/one-graph-db &
cd integration/tests/app
mvn clean package
