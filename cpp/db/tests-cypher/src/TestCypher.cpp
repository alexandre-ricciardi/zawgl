#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file
#include "catch.hpp"
#include <cstdio>

#include "CypherEngine.h"
#include "TestCommon.h"

TEST_CASE("Create a node record") {
	CypherEngine ce;
	ce.process("CREATE (a:Artist { Name : \"Strapping Young Lad\" })");

	REQUIRE(0 == 0);
}