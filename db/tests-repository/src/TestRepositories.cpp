#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file
#include "catch.hpp"
#include <cstdio>
#include "GraphRepository.h"
#include "PropertyRepository.h"
#include "TestCommon.h"

TEST_CASE("Property repository") {
	Config conf;
	configure(conf);
	deleteFiles(conf);
	PropertyRepository pr(conf.propertyStoreFileName, conf.dynamicStoreFileName);
	
	std::vector<Property> props;
	props.push_back({ { 0, true }, "key1", true });
	props.push_back({ { 0, true }, "key2", false });
	props.push_back({ { 0, true }, "key3", std::string("test-string") });
	props.push_back({ { 0, true }, "key4", 22ll });
	props.push_back({ { 0, true }, "key5", 12.005 });
	pr.putProperties(props);

	REQUIRE(props[0].id.id == 0);

	auto retrieve = pr.getPropertiesWithFirstId(0);

	REQUIRE(retrieve[0].key == "key1");
	REQUIRE(std::get<GBool>(retrieve[0].value));

	REQUIRE(retrieve[1].key == "key2");
	REQUIRE(!std::get<GBool>(retrieve[1].value));

	REQUIRE(retrieve[2].key == "key3");
	REQUIRE(std::get<GString>(retrieve[2].value) == "test-string");

	REQUIRE(retrieve[3].key == "key4");
	REQUIRE(std::get<GInteger>(retrieve[3].value) == 22);

	REQUIRE(retrieve[4].key == "key5");
	REQUIRE(std::get<GDouble>(retrieve[4].value) == 12.005);
}

TEST_CASE("Graph repository") {
	Config conf;
	configure(conf);
	deleteFiles(conf);
	GraphRepository gr(conf);


	Graph g;
	auto v0 = boost::add_vertex(g);
	auto& n0 = g[v0];
	n0.properties.push_back({ { 0, true }, "key1", true });
	n0.properties.push_back({ { 0, true }, "key2", false });
	n0.properties.push_back({ { 0, true }, "key3", std::string("test-string") });
	n0.properties.push_back({ { 0, true }, "key4", 22ll });
	n0.properties.push_back({ { 0, true }, "key5", 12.005 });

	auto v1 = boost::add_vertex(g);
	auto& n1 = g[v1];
	n1.properties.push_back({ { 0, true }, "key1", true });
	n1.properties.push_back({ { 0, true }, "key2", false });
	n1.properties.push_back({ { 0, true }, "key3", std::string("test-string") });
	n1.properties.push_back({ { 0, true }, "key4", 22ll });
	n1.properties.push_back({ { 0, true }, "key5", 12.005 });

	auto e0 = boost::add_edge(v0, v1, g);
	auto& rel0 = g[e0.first];
	rel0.properties.push_back({ { 0, true }, "key1", true });
	rel0.properties.push_back({ { 0, true }, "key2", false });
	rel0.properties.push_back({ { 0, true }, "key3", std::string("test-string") });
	rel0.properties.push_back({ { 0, true }, "key4", 22ll });
	rel0.properties.push_back({ { 0, true }, "key5", 12.005 });


	gr.storeGraph(g);

}