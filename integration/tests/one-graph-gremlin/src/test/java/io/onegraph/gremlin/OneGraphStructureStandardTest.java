package io.onegraph.gremlin;

import org.apache.tinkerpop.gremlin.GraphProviderClass;
import org.apache.tinkerpop.gremlin.structure.StructureStandardSuite;
import org.junit.Test;
import org.junit.runner.RunWith;

public class GremlinTestSuite {

    @Test
    @RunWith(StructureStandardSuite.class)
    @GraphProviderClass(provider = OneGraphProvider.class, graph = OneGraph.class)
    public class OneGraphStructureStandardTest {}
}