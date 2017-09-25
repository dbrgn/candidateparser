package ch.dbrgn.candidateparser;

import android.support.test.runner.AndroidJUnit4;

import org.junit.Test;
import org.junit.runner.RunWith;

import static org.junit.Assert.assertEquals;

@RunWith(AndroidJUnit4.class)
public class CandidateParserTest {
    @Test
    public void testCandidateParser() throws Exception {
        final String sdpString = "candidate:842163049 1 udp 1686052607 1.2.3.4 46154 typ srflx raddr 10.0.0.17 rport 46154 generation 0 ufrag EEtu network-id 3 network-cost 10";
        final IceCandidate response = CandidateParser.parseSdp(sdpString);
        System.out.println("Parsed: " + response);
        assertEquals("842163049", response.getFoundation());
        assertEquals(1, response.getComponentId());
        assertEquals("udp", response.getTransport());
        assertEquals(1686052607L, response.getPriority());
        assertEquals("1.2.3.4", response.getConnectionAddress());
        assertEquals(46154, response.getPort());
        assertEquals("srflx", response.getCandidateType());
        assertEquals("10.0.0.17", response.getRelAddr());
        assertEquals(Integer.valueOf(46154), response.getRelPort());
        assertEquals(4, response.getExtensions().size());
        assertEquals("0", response.getExtensions().get("generation"));
        assertEquals("EEtu", response.getExtensions().get("ufrag"));
        assertEquals("3", response.getExtensions().get("network-id"));
        assertEquals("10", response.getExtensions().get("network-cost"));
    }
}
