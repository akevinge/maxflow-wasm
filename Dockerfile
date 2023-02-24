FROM rust:1.67-buster

RUN apt-get update && apt-get install -y curl

WORKDIR /usr/src/maxflow-wasm
COPY . .

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

CMD ["wasm-pack", "build", "--release", "--out-dir", "build"]