#pragma once


void deleteFiles(Config& conf)
{
	remove(conf.dynamicStoreFileName.c_str());
	remove(conf.propertyStoreFileName.c_str());
	remove(conf.nodeStoreFileName.c_str());
	remove(conf.relationshipStoreFileName.c_str());
}

void configure(Config& conf)
{
	conf.propertyStoreFileName = "C:/Temp/props.db";
	conf.dynamicStoreFileName = "C:/Temp/dyn.db";
	conf.nodeStoreFileName = "C:/Temp/nodes.db";
	conf.relationshipStoreFileName = "C:/Temp/rels.db";
}
