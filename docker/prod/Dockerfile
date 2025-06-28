# ####################
# # ビルドステージ
# ####################
FROM rust:1.87.0-alpine3.21 AS builder

WORKDIR /build

# ビルドに必要なパッケージをインストール
RUN apk update && \
    apk add --no-cache \
            openssl-dev \
            alpine-sdk \
            protobuf-dev \
            curl

# gRPCのバリデーション用にgoをインストール
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

# バリデーション用にgoのライブラリをインストール
RUN go install github.com/envoyproxy/protoc-gen-validate@v1.2.1

COPY . .

# ビルド
RUN cargo build --release --locked

# ####################
# # 実行ステージ
# ####################
FROM alpine:3.21 AS runner

WORKDIR /app

# コンテナ用ユーザー作成
RUN addgroup --system --gid 1001 appuser && \
    adduser --system --uid 1001 appuser

# ビルドステージで作成したバイナリをコピー
COPY --from=builder --chown=appuser:appuser /build/target/release/rust_grpc .

# ポートを設定
EXPOSE 50051

# コンテナ起動ユーザー設定
USER appuser

# APIサーバー起動コマンド
CMD ["./rust_grpc"]
