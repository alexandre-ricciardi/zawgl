#pragma once

#include "Definitions.h"
#include "Graph.h"
#include <string>
#include <fstream>
#include "RecordManager.h"

template<typename T>
struct StaticRecord {
	bool inUse;
	T value;
};

template<typename T>
class DB_CLASS StaticStore {
private:
	RecordManager<StaticRecord<T>> recordMng;
public:
	StaticStore(const std::string& fname) : recordMng(fname)
	{

	}
	GId createRecord(StaticRecord<T>& rec)
	{
		return recordMng.writeRecord(rec);
	}
	void updateRecord(GId id, StaticRecord<T>& rec)
	{
		return recordMng.updateRecord(id, rec);
	}

};