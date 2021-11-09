package io.onegraph.gremlin.structure;

import org.apache.tinkerpop.gremlin.structure.Element;
import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.structure.util.ElementHelper;

public abstract class OneGraphElement implements Element {

    private Element detachedElement;
    private OneGraph oneGraph;

    public OneGraphElement(OneGraph oneGraph, Element detachedElement) {
        this.detachedElement = detachedElement;
    }

    @Override
    public Object id() {
        return detachedElement.id();
    }

    @Override
    public String label() {
        return detachedElement.label();
    }

    @Override
    public Graph graph() {
        return oneGraph;
    }

    @Override
    public void remove() {
        detachedElement.remove();
    }


    @Override
    public boolean equals(final Object object) {
        return ElementHelper.areEqual(this, object);
    }

    @Override
    public int hashCode() {
        return ElementHelper.hashCode(this);
    }

}
