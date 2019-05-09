#pragma once

#include "Graph.h"
#include "Definitions.h"
#include "Config.h"
#include "NodeStore.h"

class DB_CLASS NodeRepository {
private:
	NodeStore nodeStore;
public:
	NodeRepository(const std::string& fname);
	Node getNodeById(GId id);
	GId putNode(Node& node);
	std::vector<GStoreId> getFreeIds(int n);
};