#pragma once

#include "StaticStore.h"

class DB_CLASS IntegerRepository {
private:
	StaticStore<GInteger> intStore;
public:
	IntegerRepository(const std::string& fname);
	GId putInteger(GInteger value);
	GInteger getInteger(GId id);
};