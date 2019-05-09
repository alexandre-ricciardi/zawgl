#include "GraphRepository.h"


GraphRepository::GraphRepository(const Config& c): conf(c),
	propRepo(c.propertyStoreFileName, c.dynamicStoreFileName),
	relRepo(c.relationshipStoreFileName),
	nodeRepo(c.nodeStoreFileName)
{

}

void GraphRepository::storeGraph(Graph& graph)
{

	auto nbNodes = boost::num_vertices(graph);
	auto nbRels = boost::num_edges(graph);
	auto nodeIds = nodeRepo.getFreeIds(nbNodes);
	auto relIds = relRepo.getFreeIds(nbRels);

	auto vs = boost::vertices(graph);
	int countVertices = 0;
	for (auto vsit = vs.first; vsit != vs.second; ++vsit) {
		auto& node = graph[*vsit];
		node.id = nodeIds[countVertices];
		if (node.hasProperties()) {
			propRepo.putProperties(node.properties);
			node.nextPropId = node.properties[0].id.id;
		}
		++countVertices;
	}
	auto es = boost::edges(graph);
	int countEdges = 0;
	for (auto eit = es.first; eit != es.second; ++eit) {
		auto& rel = graph[*eit];
		rel.id = relIds[countEdges];
		if (rel.hasProperties()) {
			propRepo.putProperties(rel.properties);
			rel.nextPropId = rel.properties[0].id.id;
		}
		auto source = boost::source(*eit, graph);
		auto& snode = graph[source];
		rel.sourceId = snode.id.id;
		auto target = boost::target(*eit, graph);
		auto& tnode = graph[target];
		rel.targetId = tnode.id.id;
		++countEdges;
	}

	for (auto vsit = vs.first; vsit != vs.second; ++vsit) {
		auto& node = graph[*vsit];
		nodeRepo.putNode(node);
	}

	for (auto eit = es.first; eit != es.second; ++eit) {
		auto& rel = graph[*eit];
		relRepo.putRelationship(rel);
	}

}