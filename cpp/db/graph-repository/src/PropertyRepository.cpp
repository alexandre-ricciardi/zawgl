#include "PropertyRepository.h"

std::vector<GId> PropertyRepository::getNextFreeSlotIds(int n)
{
	return std::vector<GId>();
}

bool PropertyRepository::inlineProperty(const Property& p)
{
	auto size = p.key.size() + 1;
	if (std::holds_alternative<GBool>(p.value)) {
		size += sizeof(GBool);
	} else if (std::holds_alternative<GInteger>(p.value)) {
		size += sizeof(GInteger);
	} else if (std::holds_alternative<GDouble>(p.value)) {
		size += sizeof(GDouble);
	} else if (std::holds_alternative<GString>(p.value)) {
		size += std::get<GString>(p.value).size() + 1;
	}
	return size <= PropertyRecord::BlockSize;
}

PropertyRepository::PropertyRepository(const std::string & propfilename, const std::string& dsfilename): propStore(propfilename), dynStore(dsfilename)
{
}

template<typename T>
void insertPropertyValue(PropertyRecord& r, T value, int offset)
{
	std::memcpy(&r.block[offset], &value, sizeof(T));
}

void
PropertyRepository::putProperties(std::vector<Property>& props)
{
	
	std::vector<PropertyRecord> recs(props.size());
	for (int i = 0; i < props.size(); ++i) {
		auto& p = props[i];
		auto& rec = recs[i];
		if (inlineProperty(props[i])) {
			rec.setInlined();
			std::memcpy(rec.block, p.key.c_str(), p.key.size() + 1);
			if (std::holds_alternative<GBool>(p.value)) {
				rec.type = PropertyRecord::BOOL;
				insertPropertyValue(rec, std::get<GBool>(p.value), p.key.size() + 1);
			}
			else if (std::holds_alternative<GInteger>(p.value)) {
				rec.type = PropertyRecord::INTEGER;
				insertPropertyValue(rec, std::get<GInteger>(p.value), p.key.size() + 1);
			}
			else if (std::holds_alternative<GDouble>(p.value)) {
				rec.type = PropertyRecord::FLOAT;
				insertPropertyValue(rec, std::get<GDouble>(p.value), p.key.size() + 1);
			}
			else if (std::holds_alternative<GString>(p.value)) {
				rec.type = PropertyRecord::STRING;
				std::memcpy(rec.block + p.key.size() + 1, std::get<GString>(p.value).c_str(), sizeof(GString));
			}
		}
		else {
			auto keyId = dynStore.storeString(p.key);
			std::memcpy(rec.block, &keyId, sizeof(GId));
			if (std::holds_alternative<GBool>(p.value)) {
				rec.type = PropertyRecord::BOOL;
				std::memcpy(rec.block + sizeof(GId) + 1, &std::get<GBool>(p.value), sizeof(GBool));
			}
			else if (std::holds_alternative<GInteger>(p.value)) {
				rec.type = PropertyRecord::INTEGER;
				std::memcpy(rec.block + sizeof(GId) + 1, &std::get<GInteger>(p.value), sizeof(GInteger));
			}
			else if (std::holds_alternative<GDouble>(p.value)) {
				rec.type = PropertyRecord::FLOAT;
				std::memcpy(rec.block + sizeof(GId) + 1, &std::get<GDouble>(p.value), sizeof(GDouble));
			}
			else if (std::holds_alternative<GString>(p.value)) {
				rec.type = PropertyRecord::STRING;
				auto valueId = dynStore.storeString(std::get<GString>(p.value));
				std::memcpy(rec.block + sizeof(GId) + 1, &valueId, sizeof(GId));
			}
		}
	}
	auto ids = propStore.getFreeIds(props.size());

	for (int i = 0; i < ids.size(); ++i) {
		recs[i].setInUse();
		if (i == ids.size() - 1) recs[i].nextPropId = -1;
		else recs[i].nextPropId = ids[i + 1].id;
	}
	
	for (int i = 0; i < ids.size(); ++i) {
		auto id = ids[i];
		auto& p = props[i];
		p.id = id;
		if (id.toCreate) {
			propStore.writeRecord(recs[i]);
		}
		else {
			propStore.updateRecord(id.id, recs[i]);
		}
	}
}

template<typename T>
T extractPropertyValue(PropertyRecord& r, int offset)
{
	T value;
	std::memcpy(&value, &r.block[offset], sizeof(T));
	return value;
}

std::vector<Property> PropertyRepository::getPropertiesWithFirstId(GId id)
{
	std::vector<Property> props;
	auto prop = propStore.readRecordAt(id);

	while (true) {
		Property p;
		p.id = { id, false };
		if (prop.isInlined()) {
			p.key.assign(prop.block);
			switch (prop.type) {
			case PropertyRecord::BOOL:
			{
				p.value = extractPropertyValue<GBool>(prop, p.key.length() + 1);
				break;
			}
			case PropertyRecord::INTEGER:
			{
				p.value = extractPropertyValue<GInteger>(prop, p.key.length() + 1);
				break;
			}
			case PropertyRecord::FLOAT:
			{
				p.value = extractPropertyValue<GDouble>(prop, p.key.length() + 1);
				break;
			}
			case PropertyRecord::STRING:
			{
				p.value = std::string(&prop.block[p.key.length() + 1]);
				break;
			}
			}
		}
		else {

		}
		props.push_back(p);
		if (prop.nextPropId == -1) break;
		prop = propStore.readRecordAt(prop.nextPropId);
		id = prop.nextPropId;
	}
	return props;
}


std::vector<GId> PropertyRepository::getFreeIds()
{
	return std::vector<GId>();
}
