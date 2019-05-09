#pragma once

#include "Graph.h"
#include "Definitions.h"
#include "Config.h"
#include "PropertyStore.h"
#include "DynamicStore.h"

class DB_CLASS GraphStore {
private:
	const Config& conf;
public:
	GraphStore(const Config& conf);
	GId createNode();
	GId createRelationship(GId source, GId target);
	void storeGraph(Graph& graph);
	void storeProperty(Property& prop);
};