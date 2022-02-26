FROM openjdk:17-jdk-alpine AS builder

WORKDIR /player_code

COPY *.java ./

RUN javac -d classes *.java

RUN rm classes/Run.class

CMD [ "sh", "-c", "javac Main.java && jar cfe runner.jar Main *.class && cat runner.jar > run.jar" ]
