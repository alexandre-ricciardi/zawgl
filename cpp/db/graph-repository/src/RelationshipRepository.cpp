#include "RelationshipRepository.h"

RelationshipRepository::RelationshipRepository(const std::string& fname): relStore(fname)
{

}
std::vector<GStoreId> RelationshipRepository::getFreeIds(int n)
{
	return relStore.getFreeIds(n);
}

void RelationshipRepository::putRelationship(Relationship& rel)
{
	RelationshipRecord rec;
	rec.inUse = true;
	rec.sourceNodeId = rel.sourceId;
	rec.targetNodeId = rel.targetId;
	auto id = relStore.writeRecord(rec);
	rel.id = { id, false };
}

Relationship RelationshipRepository::getRelationshipById(GId id)
{
	Relationship rel{};
	auto rec = relStore.readRecordAt(id);
	rel.id = { id, false };
	rel.sourceId = rec.sourceNodeId;
	rel.targetId = rec.targetNodeId;
	return rel;
}

void RelationshipRepository::updateRelationship(Relationship& rel)
{
	RelationshipRecord rec;

	relStore.updateRecord(rel.id.id, rec);
}