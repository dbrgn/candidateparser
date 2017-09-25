package ch.dbrgn.candidateparser;

public class CandidateParser {

    static {
        System.loadLibrary("candidateparser_jni");
    }

    public static native IceCandidate parseSdp(String sdp);

}