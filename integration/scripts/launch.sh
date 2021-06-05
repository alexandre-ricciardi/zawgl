#!/bin/bash

target/debug/one-graph-db &
cd integration/tests
gradle test
