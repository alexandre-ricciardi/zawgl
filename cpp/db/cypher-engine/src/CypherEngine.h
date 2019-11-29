#pragma once

#include <string>
#include "Definitions.h"

class DB_CLASS CypherEngine {
public:
	CypherEngine();
	void process(const std::string& expr);
};