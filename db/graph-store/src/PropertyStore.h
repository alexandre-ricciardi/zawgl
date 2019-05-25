#pragma once

#include "RecordManager.h"
#include "DynamicStore.h"
#include <bitset>
#include <set>
#include <boost/endian/arithmetic.hpp>

struct PropertyRecord {
	static const int BlockSize = 32;
	enum PropType { BOOL, INTEGER, FLOAT, STRING };
	static const char IsInUse = 0b00000001;
	static const char Inlined = 0b00000010;
	unsigned char mask;
	signed char type;
	char block[BlockSize];
	GId nextPropId;
	bool isInUse()
	{
		return (mask & IsInUse) == IsInUse;
	}
	void setInUse()
	{
		mask += IsInUse;
	}
	bool isInlined()
	{
		return (mask & Inlined) == Inlined;
	}
	void setInlined()
	{
		mask += Inlined;
	}
	void serialize(std::ostream& os)
	{
		os.write(reinterpret_cast<const char*>(&mask), sizeof(unsigned char));
		os.write(reinterpret_cast<const char*>(&type), sizeof(signed char));
		os.write(block, BlockSize);
		auto bnp = boost::endian::native_to_big(nextPropId);
		os.write(reinterpret_cast<const char*>(&bnp), sizeof(GId));
	}
	void deSerialize(std::istream& is)
	{
		is.read(reinterpret_cast<char*>(&mask), sizeof(unsigned char));
		is.read(reinterpret_cast<char*>(&type), sizeof(signed char));
		is.read(block, BlockSize);
		GId bnp;
		is.read(reinterpret_cast<char*>(&bnp), sizeof(GId));
		nextPropId = boost::endian::big_to_native(bnp);
	}

	static constexpr std::size_t size()
	{
		return sizeof(char) * 2 + BlockSize + sizeof(GId);
	}
};

class DB_CLASS PropertyStore : public RecordManager<PropertyRecord> {
public:
	PropertyStore(const std::string& fname);
	~PropertyStore();
};
