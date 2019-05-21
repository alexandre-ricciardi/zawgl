#pragma once

#include <string>

#include "GraphRepository.h"
#include "CypherModel.h"

class DB_CLASS CypherEngine {
public:
	CypherEngine();
	void process(const std::string& expr);
};