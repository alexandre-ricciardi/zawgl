package io.onegraph.gremlin;

import io.onegraph.gremlin.structure.OneGraph;
import org.apache.tinkerpop.gremlin.GraphProviderClass;
import org.apache.tinkerpop.gremlin.process.ProcessComputerSuite;
import org.junit.runner.RunWith;

@RunWith(ProcessComputerSuite.class)
@GraphProviderClass(provider = OneGraphProvider.class, graph = OneGraph.class)
public class OneGraphProcessComputerTest {
}
