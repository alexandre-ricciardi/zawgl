#pragma once
#include "Definitions.h"
#include "RelationshipStore.h"

class DB_CLASS RelationshipRepository {
private:
	RelationshipStore relStore;
public:
	RelationshipRepository(const std::string& fname);
	std::vector<GStoreId> getFreeIds(int n);
	void putRelationship(Relationship& rel);
	void updateRelationship(Relationship& rel);
	Relationship getRelationshipById(GId id);
};