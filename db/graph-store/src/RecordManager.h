#pragma once

#include <string>
#include <fstream>
#include "Definitions.h"
#include <functional>
#include "Graph.h"

template<class T>
class RecordManager {
private:
	std::ifstream ifs;
	std::ofstream ofs;
protected:
	std::set<GId> unusedRecords;
public:
	RecordManager(const std::string& fileName)
	{
		ofs.open(fileName, std::ios::binary | std::ios::out);
		ifs.open(fileName, std::ios::binary | std::ios::in);
		if (!ofs.is_open() || !ifs.is_open()) {
			throw std::exception("issue opening file");
		}
	}
	GId writeRecord(T rec)
	{
		ofs.seekp(0, ofs.end);
		auto length = ofs.tellp();
		rec.serialize(ofs);
		ofs.seekp(0, ofs.beg);
		ofs.flush();
		return length / T::size();
	}
	GId getLastRecordId()
	{
		ofs.seekp(0, ofs.end);
		auto length = ofs.tellp();
		ofs.seekp(0, ofs.beg);
		return length / T::size() - 1;
	}

	void updateRecord(GId id, T rec)
	{
		ofs.seekp(0, id * T::size());
		rec.serialize(ofs);
		ofs.flush();
	}
	T readRecordAt(GId id)
	{
		T rec;
		ifs.seekg(id * T::size());
		rec.deSerialize(ifs);
		return rec;
	}
	void scan(std::function<bool(GId, T&)> callback)
	{
		GId curr = 0;
		bool next = true;
		while (next && hasNextRecord(curr)) {
			next = callback(curr, readRecordAt(curr));
			++curr;
		}
	}
	
	bool hasNextRecord(GId id)
	{
		return getLastRecordId() - id > 0;
	}
	std::vector<GStoreId> getFreeIds(int n)
	{
		std::vector<GStoreId> res;
		scan([&](GId id, T& rec) {
			if (!rec.isInUse()) {
				res.push_back({ id, false });
			}
			return res.size() < n;
		});
		if (res.size() < n) {
			auto allocIds = getNextAllocatableIds(n - res.size());
			for (auto allocId : allocIds) {
				res.push_back({ allocId, true });
			}
		}
		return res;
	}

	void scanUnusedRecordIds()
	{
		unusedRecords.clear();
		scan([&](GId id, T& rec) {
			if (!rec.isInUse()) {
				unusedRecords.insert(id);
			}
			return true;
		});
	}
	std::set<GId>& getUnusedRecords()
	{
		return unusedRecords;
	}
	std::vector<GId> getNextAllocatableIds(int n) {
		std::vector<GId> res;
		auto first = getLastRecordId() + 1;
		for (int i = 0; i < n; ++i) {
			res.push_back(first + i);
		}
		return res;
	}
	~RecordManager()
	{
		ifs.close();
		ofs.close();
	}
};