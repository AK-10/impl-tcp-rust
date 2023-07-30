## toytcp
### 実装する機能
簡略化した実装を作成する
- linten, accept, connect, send, recv, closeといった基本的なAPI
- 再送制御
- 順序制御
- スライディングウィンドウ

toytcpを使用して以下のアプリケーションを作成する
- エコーサーバ、エコークライアント
  - 文字列のやり取りを行う
  - サーバは複数のクライアントを同時にハンドリングする
- ファイルサーバ、クライアント
  - アップロード機能のみ
  - クライアントから数MBのファイルをサーバにアップロードする
  - 少しパケットロスをしても問題なく完了する

## 制約/やらないこと
- IPv4にのみ対応
- RSTセグメントは扱わない（理由は第2章で述べました）
- 再送タイムアウト時間（Retransmission Timeout: RTO）は固定
- ウィンドウサイズは固定
  - 輻輳制御は行わない
  - フロー制御は行わない
- TCPオプションは利用しない
- 不測の事態（突然ホストがクラッシュするなど）には対応しない
- セキュリティ関連も深く考えない
- 同じホスト上でToyTCPとOSのTCPを同時に混ぜて使うことはできない

## References
- [Rustで始めるTCP自作入門](https://www.amazon.co.jp/Rust%E3%81%A7%E5%A7%8B%E3%82%81%E3%82%8BTCP%E8%87%AA%E4%BD%9C%E5%85%A5%E9%96%80-%E5%B0%8F%E9%87%8E-%E8%BC%9D%E4%B9%9F-ebook/dp/B09FG2SL2S)
- [RFC793](https://datatracker.ietf.org/doc/html/rfc793)
- [RFC793(日本語訳)](https://tex2e.github.io/rfc-translater/html/rfc9293.html)
