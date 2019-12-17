FROM xd009642/tarpaulin

RUN apt-get update &&\
    apt-get install -y swi-prolog
