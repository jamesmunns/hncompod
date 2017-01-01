FROM  debian:jessie

RUN apt-get update
RUN apt-get install -y wget \
                       unzip \
                       default-jre

RUN wget https://github.com/marytts/marytts/releases/download/v5.2/marytts-5.2.zip
RUN unzip marytts-5.2.zip

CMD ["/bin/bash", "/marytts-5.2/bin/marytts-server"]
