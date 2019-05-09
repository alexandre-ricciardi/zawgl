#pragma once

#include "Definitions.h"
#include <string>

struct Config {
	std::string propertyStoreFileName;
	std::string dynamicStoreFileName;
	std::string nodeStoreFileName;
	std::string relationshipStoreFileName;
};