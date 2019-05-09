#pragma once

#include "Definitions.h"
#include <string>
#include "RecordManager.h"
#include <boost/endian/arithmetic.hpp>
#include <iostream>

struct RelationshipRecord {
	bool inUse;
	GId sourceNodeId;
	GId targetNodeId;
	GId sourcePrevRelId;
	GId targetPrevRelId;
	GId sourceNextRelId;
	GId targetNextRelId;
	GId nextPropId;
	bool isInUse()
	{
		return inUse;
	}
	void writeBigEndian(std::ostream& os, GId id)
	{
		auto bid = boost::endian::native_to_big(id);
		os.write(reinterpret_cast<const char*>(&bid), sizeof(GId));
	}
	GId readBigEndian(std::istream& is)
	{
		GId bid;
		is.read(reinterpret_cast<char*>(&bid), sizeof(GId));
		return boost::endian::big_to_native(bid);
	}
	void serialize(std::ostream& os)
	{
		os.write(reinterpret_cast<const char*>(&inUse), sizeof(bool));
		writeBigEndian(os, sourceNodeId);
		writeBigEndian(os, targetNodeId);
		writeBigEndian(os, sourcePrevRelId);
		writeBigEndian(os, targetPrevRelId);
		writeBigEndian(os, sourceNextRelId);
		writeBigEndian(os, targetNextRelId);
		writeBigEndian(os, nextPropId);
	}
	void deSerialize(std::istream& is)
	{
		is.read(reinterpret_cast<char*>(&inUse), sizeof(short));
		sourceNodeId = readBigEndian(is);
		targetNodeId = readBigEndian(is);
		sourcePrevRelId = readBigEndian(is);
		targetPrevRelId = readBigEndian(is);
		sourceNextRelId = readBigEndian(is);
		targetNextRelId = readBigEndian(is);
		nextPropId = readBigEndian(is);
	}

	static constexpr int size()
	{
		return sizeof(bool) + 7 * sizeof(GId);
	}
};


class DB_CLASS RelationshipStore : public RecordManager<RelationshipRecord> {
public:
	RelationshipStore(const std::string fname);
	~RelationshipStore();
};