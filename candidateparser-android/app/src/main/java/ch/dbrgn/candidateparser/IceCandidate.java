package ch.dbrgn.candidateparser;

import java.util.HashMap;

/**
 * A parsed ICE candidate POJO.
 */
public class IceCandidate {
    // Non-null fields
    private String foundation;
    private long componentId;
    private String transport;
    private long priority;
    private String connectionAddress;
    private int port;
    private String candidateType;

    // Extensions
    // Note: In theory the extension fields are byte arrays, not strings.
    // But since Java cannot use byte[] as map key, and because the Java
    // libwebrtc bindings return a String for the candidate SDP anyways,
    // we'll use strings.
    private HashMap<String, String> extensions = new HashMap<>();

    // Nullable fields
    private String relAddr = null;
    private Integer relPort = null;

    public IceCandidate() {
    }

    public IceCandidate(String foundation, long componentId,
                        String transport, long priority,
                        String connectionAddress, int port,
                        String candidateType) {
        this.foundation = foundation;
        this.componentId = componentId;
        this.transport = transport;
        this.priority = priority;
        this.connectionAddress = connectionAddress;
        this.port = port;
        this.candidateType = candidateType;
    }

    public String getFoundation() {
        return foundation;
    }

    public long getComponentId() {
        return componentId;
    }

    public String getTransport() {
        return transport;
    }

    public long getPriority() {
        return priority;
    }

    public String getConnectionAddress() {
        return connectionAddress;
    }

    public int getPort() {
        return port;
    }

    public String getCandidateType() {
        return candidateType;
    }

    public void setRelAddr(String relAddr) {
        this.relAddr = relAddr;
    }

    public String getRelAddr() {
        return relAddr;
    }

    public void setRelPort(int relPort) {
        this.relPort = relPort;
    }

    public Integer getRelPort() {
        return relPort;
    }

    public void addExtension(String key, String val) {
        this.extensions.put(key, val);
    }

    public HashMap<String, String> getExtensions() {
        return extensions;
    }

    public String getExtension(String key) {
        return extensions.get(key);
    }
}
