package io.onegraph.gremlin;

import org.apache.commons.configuration2.AbstractConfiguration;
import org.apache.commons.configuration2.Configuration;

import java.io.Serializable;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

public class OneGraphConfiguration extends AbstractConfiguration implements Serializable, Iterable {

    private final Map<String, Object> properties = new HashMap<>();

    public OneGraphConfiguration(final Configuration configuration) {
        this.copy(configuration);
    }

    @Override
    public Iterator iterator() {
        return properties.keySet().iterator();
    }

    @Override
    protected void addPropertyDirect(String s, Object o) {

    }

    @Override
    protected void clearPropertyDirect(String key) {
        properties.remove(key);
    }

    @Override
    protected Iterator<String> getKeysInternal() {
        return properties.keySet().iterator();
    }

    @Override
    protected Object getPropertyInternal(String s) {
        return properties.get(s);
    }

    @Override
    protected boolean isEmptyInternal() {
        return false;
    }

    @Override
    protected boolean containsKeyInternal(String s) {
        return false;
    }
}
