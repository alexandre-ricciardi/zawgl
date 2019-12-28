#pragma once

#include "Definitions.h"
#include <string>
#include <fstream>
#include "RecordManager.h"
#include <iostream>
#include <boost/endian/arithmetic.hpp>

struct NodeRecord {
	bool inUse;
	GId nextRelId;
	GId nextPropId;
	bool isInUse()
	{
		return inUse;
	}
	void serialize(std::ostream& os)
	{
		os.write(reinterpret_cast<const char*>(&inUse), sizeof(bool));
		auto bnr = boost::endian::native_to_big(nextRelId);
		os.write(reinterpret_cast<const char*>(&bnr), sizeof(GId));
		auto bnp = boost::endian::native_to_big(nextPropId);
		os.write(reinterpret_cast<const char*>(&bnp), sizeof(GId));
	}
	void deSerialize(std::istream& is)
	{
		is.read(reinterpret_cast<char*>(&inUse), sizeof(bool));
		GId bnr;
		is.read(reinterpret_cast<char*>(&bnr), sizeof(GId));
		nextRelId = boost::endian::big_to_native(bnr);
		GId bnp;
		is.read(reinterpret_cast<char*>(&bnp), sizeof(GId));
		nextPropId = boost::endian::big_to_native(bnp);
	}

	static constexpr int size()
	{
		return sizeof(bool) + 2 * sizeof(GId);
	}
};

class DB_CLASS NodeStore : public RecordManager<NodeRecord> {
public:
	NodeStore(const std::string& file);
	~NodeStore();
};