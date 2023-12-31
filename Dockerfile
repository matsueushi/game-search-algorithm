FROM rust:1.67

RUN rustup component add rustfmt
RUN apt update && \
    apt install -y \
    python3-pip 
RUN pip3 install \
    polars \
    pandas \
    numpy \
    matplotlib
