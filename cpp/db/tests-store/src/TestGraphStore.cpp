#include "catch.hpp"
#include <cstdio>
#include "GraphStore.h"

void deleteFiles(Config& conf)
{
	remove(conf.dynamicStoreFileName.c_str());
	remove(conf.propertyStoreFileName.c_str());
	remove(conf.nodeStoreFileName.c_str());
	remove(conf.relationshipStoreFileName.c_str());
}

void configure(Config& conf)
{
	conf.propertyStoreFileName = "C:/Temp/props.db";
	conf.dynamicStoreFileName = "C:/Temp/props.db";
	conf.nodeStoreFileName = "C:/Temp/nodes.db";
	conf.relationshipStoreFileName = "C:/Temp/rels.db";
}

TEST_CASE("Store graph property") {
	Config conf;
	configure(conf);
	deleteFiles(conf);
	GraphStore gs(conf);
	gs.createNode();
}