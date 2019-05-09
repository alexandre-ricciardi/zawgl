#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file
#include "catch.hpp"
#include <cstdio>

#include "NodeStore.h"

TEST_CASE("Create a node record") {
	remove("C:/Temp/nodes.db");
	NodeStore ns("C:/Temp/nodes.db");
	NodeRecord rec{};
	auto id = ns.writeRecord(rec);
	REQUIRE(id == 0);
}

TEST_CASE("Get node record") {
	remove("C:/Temp/nodes.db");
	NodeStore ns("C:/Temp/nodes.db");
	NodeRecord rec{false, 3, 8};
	auto id = ns.writeRecord(rec);
	REQUIRE(id == 0);
	auto get = ns.readRecordAt(0);
	REQUIRE(!get.inUse);
	REQUIRE(get.nextPropId == 8);
	REQUIRE(get.nextRelId == 3);
}

TEST_CASE("Update a node record") {
	remove("C:/Temp/nodes.db");
	NodeStore ns("C:/Temp/nodes.db");
	NodeRecord rec{ false, 3, 8 };
	auto id = ns.writeRecord(rec);
	REQUIRE(id == 0);
	rec.nextPropId = 12;
	ns.updateRecord(id, rec);
	auto get = ns.readRecordAt(0);
	REQUIRE(!get.inUse);
	REQUIRE(get.nextPropId == 12);
	REQUIRE(get.nextRelId == 3);
}