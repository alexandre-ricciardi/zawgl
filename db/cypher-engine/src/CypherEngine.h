#pragma once

#include <string>

#include "GraphRepository.h"
#include "CypherModel.h"

class DB_CLASS CypherEngine {
private:
	GraphRepository& gr;
public:
	CypherEngine(GraphRepository& gr);
	void process(const std::string& expr);
};