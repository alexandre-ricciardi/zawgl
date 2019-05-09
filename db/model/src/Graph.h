#pragma once

#include <boost/config.hpp>
#include <iostream>
#include <vector>
#include <string>
#include <boost/graph/adjacency_list.hpp>
#include <boost/tuple/tuple.hpp>
#include <map>

#include <boost/graph/fruchterman_reingold.hpp>
#include <boost/graph/random_layout.hpp>
#include <boost/graph/topology.hpp>
#include <boost/lexical_cast.hpp>
#include <boost/random/linear_congruential.hpp>
#include <boost/progress.hpp>
#include <boost/shared_ptr.hpp>
#include <variant>


typedef long long GId;
typedef long long GInteger;
typedef bool GBool;
typedef double GDouble;
typedef std::string GString;
typedef std::variant<bool, long long, double, std::string> ValueType;

struct GStoreId {
	GId id;
	bool toCreate;
};
struct Property {
	GStoreId id;
	std::string key;
	ValueType value;
	
};

struct Node {
	GStoreId id;
	GId nextPropId;
	GId nextRelId;
	std::vector<Property> properties;
	bool hasProperties()
	{
		return properties.size() > 0;
	}
};

struct Relationship {
	GStoreId id;
	GId nextPropId;
	std::vector<Property> properties;
	GId sourceId;
	GId targetId;
	bool hasProperties()
	{
		return properties.size() > 0;
	}
};

using namespace boost;
typedef adjacency_list<vecS, vecS, directedS, Node, Relationship> Graph;