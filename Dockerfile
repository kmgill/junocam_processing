FROM rust:latest

ENV CSPICE_DIR=/juno/cspice 
ENV JUNOBASE=/juno/spice/

WORKDIR /juno

RUN apt update
RUN apt -y install clang csh
RUN curl -JO https://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z
RUN curl -JO https://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/importCSpice.csh
RUN csh importCSpice.csh
RUN ln -s /juno/cspice/lib/cspice.a /juno/cspice/lib/libcspice.a

COPY . .
RUN cargo install --path .

#RUN bash update_spice_data.sh
WORKDIR /data