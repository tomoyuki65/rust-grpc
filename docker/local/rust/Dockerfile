FROM rust:1.87

# goのパス設定
ENV PATH="/root/go/bin:$PATH"

WORKDIR /app

COPY . .

RUN apt-get update && \
    apt-get install -y \
      protobuf-compiler \
      # gRPCのドキュメント生成用にgoをインストール
      golang

# gRPCのドキュメント生成用にgoのライブラリをインストール
RUN go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@latest

# ホットリロード用のライブラリをインストール
RUN cargo install cargo-watch

# Rust用のリンターをインストール
RUN rustup component add clippy

# Rust用のフォーマッターをインストール
RUN rustup component add rustfmt
