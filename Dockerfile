FROM centos:7

RUN mkdir /wmic
WORKDIR /wmic
COPY . .

RUN yum -y install \
    curl \
    gcc

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release
