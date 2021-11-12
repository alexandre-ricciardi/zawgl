package io.onegraph.gremlin;

import io.onegraph.gremlin.structure.OneGraph;
import org.apache.tinkerpop.gremlin.GraphProviderClass;
import org.apache.tinkerpop.gremlin.process.ProcessStandardSuite;
import org.junit.runner.RunWith;

//@RunWith(ProcessStandardSuite.class)
@GraphProviderClass(provider = OneGraphProvider.class, graph = OneGraph.class)
public class OneGraphProcessStandardTest {
}
