#include "NodeRepository.h"

NodeRepository::NodeRepository(const std::string& fname): nodeStore(fname)
{

}

GId NodeRepository::putNode(Node& node)
{
	NodeRecord rec{};
	rec.inUse = true;
	if (node.properties.size() > 0) {
		rec.nextPropId = node.properties[0].id.id;
	}
	auto id = nodeStore.writeRecord(rec);
	node.id = { id, false };
	return id;
}

std::vector<GStoreId> NodeRepository::getFreeIds(int n)
{
	return nodeStore.getFreeIds(n);
}

Node NodeRepository::getNodeById(GId id)
{
	return {};
}