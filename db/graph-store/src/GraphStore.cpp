#include "GraphStore.h"

GraphStore::GraphStore(const Config& conf): conf(conf)
{
}

GId GraphStore::createNode()
{
	return GId();
}

GId GraphStore::createRelationship(GId source, GId target)
{
	return GId();
}

void GraphStore::storeGraph(Graph& graph)
{

}

void GraphStore::storeProperty(Property & prop)
{
}
