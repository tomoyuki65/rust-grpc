# RustのtonicによるgRPCサンプル
RustのtonicによるgRPCサーバーの開発用サンプルです。  
  
<br />
  
## 要件
・Rust: <span style="color:green">1.87</span>  
・tonic: <span style="color:green">0.13.1</span>  
・tonic-build: <span style="color:green">0.13.1</span>  
・prost: <span style="color:green">0.13.5</span>  
・prost-build: <span style="color:green">0.13.5</span>  
・prost-validate: <span style="color:green">0.2.7</span>  
・prost-validate-build: <span style="color:green">0.2.7</span>  
> <span style="color:red">※利用するバージョンによっては互換性が無い可能性があるので注意して下さい。</span>  
  
<br />
  
## ローカル開発環境構築
### 1. 環境変数ファイルをリネーム
```
cp ./.env.example ./.env
```  
  
### 2. コンテナのビルドと起動
```
docker compose build --no-cache
docker compose up -d
```  
> <span style="color:red">※テストコードを実行させる際はテスト用の環境変数ファイルを使うため、「docker compose --env-file ./.env.testing up -d」で起動すること。</span>
  
### 3. コンテナの停止・削除
```
docker compose down
```  
  
<br />
  
## コード修正後に使うコマンド
ローカルサーバー起動中に以下のコマンドを実行可能です。  
  
### 1. フォーマット修正
```
docker compose exec grpc cargo fmt
```  
  
### 2. コード解析チェック
```
docker compose exec grpc cargo clippy
```  
  
### 3. テストコードの実行
<span style="color:red">事前にテスト用環境変数を設定したローカルサーバーを起動（docker compose --env-file ./.env.testing up -d）してから以下のコマンドを使ってテストを実行して下さい</span>  
```
docker compose exec -e CARGO_TEST=testing grpc cargo test -- --nocapture --test-threads=1
```  
> ※DBのデータの同期を考慮して「--test-threads=1」で実行する
  
<br />
  
## protoファイルからのコード生成について  
ローカルサーバーを起動すると「build.rs」がビルドされ、target配下にprotoファイルに関するコードが生成されます。対象のprotoファイルを追加した際は、「build.rs」も修正して下さい。  
  
また、エンドポイントの仕様については、protoファイルから生成したドキュメントファイル「doc/docs.md」を確認して下さい。そしてドキュメントの修正が必要な際は以下のコマンドを実行して下さい。  
```
docker compose exec grpc protoc -I=.:../root/go/pkg/mod/github.com/envoyproxy/protoc-gen-validate@v1.2.1 --doc_out=./doc --doc_opt=markdown,docs.md ./proto/sample/sample.proto
```  
  
<br />
  
## 本番環境用のコンテナについて
本番環境用コンテナをローカルでビルドして確認したい場合は、以下の手順で行って下さい。  
  
### 1. .env.productionの修正
本番環境用の機密情報を含まない環境変数の設定には「.env.production」を使います。
ローカルで確認する場合は必要に応じて内容を修正して下さい。  
  
### 2. コンテナのビルド
以下のコマンドを実行し、コンテナをビルドします。  
```
docker build --no-cache -f ./docker/prod/Dockerfile -t rust-grpc:latest .
```  
  
### 3. コンテナの起動
以下のコマンドを実行し、コンテナを起動します。  
```
docker run -d -p 50051:50051 --env-file .env.production rust-grpc:latest
```  
  
