package io.onegraph.gremlin;

import io.onegraph.gremlin.structure.OneGraph;
import org.apache.tinkerpop.gremlin.GraphProviderClass;
import org.apache.tinkerpop.gremlin.structure.StructureStandardSuite;
import org.junit.runner.RunWith;

@RunWith(StructureStandardSuite.class)
@GraphProviderClass(provider = OneGraphProvider.class, graph = OneGraph.class)
public class OneGraphStructureStandardTest {
}