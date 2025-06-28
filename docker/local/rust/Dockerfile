FROM rust:1.87

WORKDIR /app

COPY . .

# インストール可能なパッケージ一覧の更新
RUN apt-get update && \
    # パッケージのインストール
    apt-get install -y \
      protobuf-compiler \
      curl

# gRPCのドキュメント生成やバリデーション用にgoをインストール
# goのバージョンを変更したい場合は以下リンク先のページで必要情報をご確認下さい。
# https://go.dev/dl/
ENV GO_VERSION=1.24.4
ENV GO_SHA256=d5501ee5aca0f258d5fe9bfaed401958445014495dc115f202d43d5210b45241
ENV GO_OS=linux
ENV GO_ARCH=arm64
ENV PATH="/usr/local/go/bin:/root/go/bin:${PATH}"

RUN curl -fsSLO "https://go.dev/dl/go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz" && \
    echo "${GO_SHA256}  go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz" | sha256sum -c - && \
    tar -C /usr/local -xzf "go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz" && \
    rm "go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz" && \
    rm -rf /var/lib/apt/lists/*

# gRPCのドキュメント生成用にgoのライブラリをインストール
RUN go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@v1.5.1

# バリデーション用にgoのライブラリをインストール
RUN go install github.com/envoyproxy/protoc-gen-validate@v1.2.1

# ホットリロード用のライブラリをインストール
RUN cargo install cargo-watch

# Rust用のリンターをインストール
RUN rustup component add clippy

# Rust用のフォーマッターをインストール
RUN rustup component add rustfmt
