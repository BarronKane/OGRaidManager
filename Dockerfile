FROM ubuntu:latest

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y build-essential git curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
RUN apt install -y postgresql-common
RUN /usr/share/postgresql-common/pgdg/apt.postgresql.org.sh -y
RUN apt-get install -y postgresql-client-16 postgresql-doc-16 libpq-dev postgresql-server-dev-16 
