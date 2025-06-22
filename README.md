# RustのtonicによるgRPCサンプル
RustのtonicによるgRPCサーバーの開発用サンプルです。  
  
<br />
  
## 要件
・Rustのバージョンは<span style="color:green">1.87</span>です。  
  
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
docker compose exec api cargo fmt
```  
  
### 2. コード解析チェック
```
docker compose exec api cargo clippy
```  
  
<br />
  
## protoファイルからのコード生成について  
ローカルサーバーを起動すると「build.rs」がビルドされ、target配下にprotoファイルに関するコードが生成されます。対象のprotoファイルを追加した際は、「build.rs」も修正して下さい。  
  
また、エンドポイントの仕様については、protoファイルから生成したドキュメントファイル「doc/docs.md」を確認して下さい。そしてドキュメントの修正が必要な際は以下のコマンドを実行して下さい。  
```
docker compose exec grpc protoc --doc_out=./doc --doc_opt=markdown,docs.md ./proto/sample/sample.proto
```  
  