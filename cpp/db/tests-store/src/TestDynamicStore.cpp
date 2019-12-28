#include "catch.hpp"

#include "DynamicStore.h"
#include <iostream>

TEST_CASE("Store an retrieve strings") {
	remove("C:/Temp/dyn.db");
	DynamicStore ds("C:/Temp/dyn.db");
	auto ids = ds.storeString("blabla");
	auto s = "verrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
		"lonnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnng strinnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnng"
		"withhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh 000000000000000000000000";
	auto idl = ds.storeString(s);
	auto res = ds.retrieveString(idl);
	auto ress = ds.retrieveString(ids);
	REQUIRE(ress == "blabla");
	REQUIRE(res == s);
}