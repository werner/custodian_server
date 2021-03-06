FROM rustlang/rust:nightly

WORKDIR /usr/src/custodian_server
COPY Cargo.toml .
COPY . .

EXPOSE 9100

RUN echo "deb http://deb.debian.org/debian testing main" >> /etc/apt/sources.list
RUN apt-get update && apt-get -t testing install -y --no-install-recommends \ 
    linux-compiler-gcc-7-x86 g++-7 libstdc++-7-dev

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential libc-bin libc-dev-bin cmake \
    libgmp-dev clang libclang-dev llvm llvm-dev

RUN cargo +nightly install
RUN cargo +nightly build

RUN ./prepare_tests.sh
RUN ./use_prepared_database.sh