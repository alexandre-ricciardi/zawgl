import org.apache.tinkerpop.gremlin.GraphProviderClass;
import org.apache.tinkerpop.gremlin.structure.StructureStandardSuite;
import org.junit.runner.RunWith;
import org.onegraph.gremlin.integration.test.OneGraphProvider;

public class GremlinTestSuite {
    
    @RunWith(StructureStandardSuite.class)
    @GraphProviderClass(provider = OneGraphProvider.class, graph = OneGraph.class)
    public class OneGraphStructureStandardTest {}
}