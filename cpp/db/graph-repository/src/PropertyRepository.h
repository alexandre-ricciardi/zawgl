#pragma once

#include "Definitions.h"
#include "PropertyStore.h"
#include "DynamicStore.h"
#include "Graph.h"

class DB_CLASS PropertyRepository {
private:
	PropertyStore propStore;
	DynamicStore dynStore;
	std::vector<GId> getNextFreeSlotIds(int n);
	bool inlineProperty(const Property& p);
public:
	PropertyRepository(const std::string& propfilename, const std::string& dsfilename);
	void putProperties(std::vector<Property>& props);
	std::vector<Property> getPropertiesWithFirstId(GId id);
	std::vector<GId> getFreeIds();
};