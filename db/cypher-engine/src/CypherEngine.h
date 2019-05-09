#pragma once

#include <string>

#include "GraphRepository.h"

class DB_CLASS CypherEngine {
private:
	Graph createGraph;
public:
	CypherEngine(GraphRepository& gr);
	void process(const std::string& expr);
};