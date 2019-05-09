#include "DynamicStore.h"

DynamicStore::DynamicStore(const std::string & file): RecordManager(file)
{

}

GId DynamicStore::storeString(const std::string& value)
{
	auto nbPages = value.size() / PageRecord::PayloadSize;
	auto firstPageId = getLastRecordId() + 1;
	for (int i = 0; i < nbPages + 1; ++i) {
		auto rest = value.size() - i * PageRecord::PayloadSize;
		rest = rest < PageRecord::PayloadSize ? rest : PageRecord::PayloadSize;
		PageRecord p{};
		p.inUse = true;
		p.payloadSize = rest;
		if (i == nbPages) {
			p.nextPageId = -1;
		}
		else {
			p.nextPageId = firstPageId + 1;
		}
		
		value.copy(p.payload, rest, i * PageRecord::PayloadSize);
		writeRecord(p);
	}
	return firstPageId;
}

std::string DynamicStore::retrieveString(GId id)
{
	std::string res;
	auto p = readRecordAt(id);
	while (true) {
		res.append(p.payload, p.payloadSize);
		if (p.nextPageId == -1) break;
		p = readRecordAt(p.nextPageId);
	}
	return res;
}


DynamicStore::~DynamicStore()
{
}
