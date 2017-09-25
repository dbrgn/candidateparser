# Makefile to generate JNI header file for the CandidateParser class.

default:
	@echo 'Use "make jni" to generate JNI headers.'

all: jni

classfile: app/src/main/java/ch/dbrgn/candidateparser/CandidateParser.java app/src/main/java/ch/dbrgn/candidateparser/IceCandidate.java
	javac -classpath app/src/main/java/ app/src/main/java/ch/dbrgn/candidateparser/IceCandidate.java
	javac -classpath app/src/main/java/ app/src/main/java/ch/dbrgn/candidateparser/CandidateParser.java

jni: classfile
	javah -classpath app/src/main/java/ -o CandidateParserJNI.h ch.dbrgn.candidateparser.CandidateParser

clean:
	rm -f CandidateParserJNI.h app/src/main/java/ch/dbrgn/candidateparser/IceCandidate.class app/src/main/java/ch/dbrgn/candidateparser/CandidateParser.class

.PHONY: clean all
