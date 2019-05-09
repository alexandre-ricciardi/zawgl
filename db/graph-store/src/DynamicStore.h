#pragma once

#include "Definitions.h"
#include "Graph.h"
#include <string>
#include <fstream>
#include "RecordManager.h"
#include <iostream>
#include <boost/endian/arithmetic.hpp>


struct PageRecord {
	static const int PayloadSize = 128;
	bool inUse;
	short payloadSize;
	char payload[PayloadSize];
	GId nextPageId;
	bool isInUse()
	{
		return inUse;
	}
	void serialize(std::ostream& os)
	{
		os.write(reinterpret_cast<const char*>(&inUse), sizeof(bool));
		os.write(reinterpret_cast<const char*>(&payloadSize), sizeof(short));
		os.write(payload, PayloadSize);
		auto bnp = boost::endian::native_to_big(nextPageId);
		os.write(reinterpret_cast<const char*>(&bnp), sizeof(GId));
	}
	void deSerialize(std::istream& is)
	{
		is.read(reinterpret_cast<char*>(&inUse), sizeof(bool));
		is.read(reinterpret_cast<char*>(&payloadSize), sizeof(short));
		is.read(payload, PayloadSize);
		GId bnp;
		is.read(reinterpret_cast<char*>(&bnp), sizeof(GId));
		nextPageId = boost::endian::big_to_native(bnp);
	}

	static constexpr int size()
	{
		return sizeof(bool) + sizeof(short) + sizeof(GId) + PayloadSize;
	}
};

class DB_CLASS DynamicStore : public RecordManager<PageRecord> {
public:
	DynamicStore(const std::string& file);
	GId storeString(const std::string& value);
	std::string retrieveString(GId id);
	~DynamicStore();
};