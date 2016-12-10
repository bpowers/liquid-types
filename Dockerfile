FROM bpowers/rust:nightly
MAINTAINER Bobby Powers "bobbypowers@gmail.com"

ENV HOME /src

RUN mkdir -p $HOME/liquid-types

COPY . $HOME/liquid-types
WORKDIR $HOME/liquid-types

RUN make

ENV PATH $PATH:/src/liquid-types/target/debug

CMD /bin/bash
