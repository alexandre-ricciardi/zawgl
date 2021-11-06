package io.onegraph.gremlin;

import org.apache.tinkerpop.gremlin.GraphProviderClass;
import org.apache.tinkerpop.gremlin.structure.StructureStandardSuite;
import org.junit.runner.RunWith;

public class GremlinTestSuite {
    
    @RunWith(StructureStandardSuite.class)
    @GraphProviderClass(provider = OneGraphProvider.class, graph = OneGraph.class)
    public class OneGraphStructureStandardTest {}
}