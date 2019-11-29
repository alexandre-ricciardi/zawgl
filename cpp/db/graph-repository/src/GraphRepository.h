#pragma once

#include "Definitions.h"
#include "Config.h"
#include "Graph.h"
#include "NodeRepository.h"
#include "RelationshipRepository.h"
#include "PropertyRepository.h"

class DB_CLASS GraphRepository {
private:
	const Config& conf;
	PropertyRepository propRepo;
	RelationshipRepository relRepo;
	NodeRepository nodeRepo;
public:
	GraphRepository(const Config& c);
	void storeGraph(Graph& g);

};