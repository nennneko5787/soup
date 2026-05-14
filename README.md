# soup
Rustで書かれた、非常に高速なHSP向けパッケージ及びプロジェクトマネージャー。

## ハイライト
- 今まで開発されたプロジェクト管理ツール・ビルドツールを置き換える単一のツール。
- HSPアップデーターよりも速い。
- プロジェクト管理機能と、ロックファイルを提供します。
- 複数バージョンのHSPを管理します。
- 仮想環境により、もうあなたはHSPがインストールされたフォルダを汚しません。
- グローバルキャッシュによる容量削減。
- uvライクなコマンド。
- インストールには、RustやHSPは必要ありません。
- Windowsのみをサポートしています。

## インストール
> [!Important]
> この項は書きかけです。

## ドキュメント
> [!Important]
> この項は書きかけです。

## 特徴

### プロジェクト

```console
PS C:\YourProjectFolder> soup new example
"example" プロジェクトを作成しました。 [0s]

プロジェクトを実行するには、以下のコマンドを使用してください。
cd example
soup run
```

### HSPバージョンの管理 (hspupd経由)

```console
PS C:\YourProjectFolder\example> soup hsp list
hsp36_base: HSP3フルセット(3.6) by ONION software (公開日 2021/08/10)
hsp36en_base: HSP3フルセット英語版(3.6) by ONION software (公開日 2021/08/10)
hsp37_base: HSP3基本システム(3.7) by ONION software (公開日 2025/09/04)
hsp37b10_base: HSP3基本システム(3.7beta10a) by ONION software (公開日 2025/03/13)
hsp37b7_base: HSP3基本システム(3.7beta7) by ONION software (公開日 2023/10/25)
hsp37b8_base: HSP3基本システム(3.7beta8) by ONION software (公開日 2024/01/15)
hsp37b9_base: HSP3基本システム(3.7beta9) by ONION software (公開日 2024/07/18)
hsp37rc1_base: HSP3基本システム(3.7rc1) by ONION software (公開日 2025/06/10)
詳細を確認するには、 soup hsp info <id> コマンドを使用します。
これらをインストールするには、 soup hsp install <id> コマンドを使用します。

PS C:\YourProjectFolder\example> soup hsp info hsp36_base
hsp36_base: HSP3フルセット(3.6) by ONION software (公開日 2021/08/10)
HSP3.6フルセットは、Windows上で動作する標準的なHSP3環境です。2021年にリリースされ長い期間使われている安定版としてお使い頂けます。
Windowsアプリケーション(Win32/x64)、HSP3Dishアプリケーション、HGIMG4アプリケーション及びHSP3Dishマルチプラットフォームアプリケーションを作成できます。
最新の機能やプラットフォームマルチサポートが必要な場合は、最新のβバージョンをご利用ください。
依存関係: *
```

## 貢献
> [!Important]
> この項は書きかけです。

このプロジェクトは未成熟で(そして、それは永遠に続くでしょう)、どなたからの貢献も嬉しくお待ちしております。  
詳しくは[貢献ガイド](https://github.com/nennneko5787/soup?tab=contributing-ov-file#contributing)をお読みください。

## よくある質問

### soupは実環境使用に耐えられますか？
soupはまだ開発初期の段階で、一部の機能が実装されていません。  
現段階での使用は自己責任です。

## 謝辞

soupのコンセプトは、[uv](https://github.com/astral-sh/uv)、[pnpm](https://pnpm.io/)などの素晴らしいツールから着想を得ました。この場を借りて開発者の皆様に感謝申し上げます。  
私がHSPという言語に出会うきっかけとなった、HSP及びhspupdater開発者の[おにたま氏](https://www.onionsoft.net/)にも感謝を申し上げます。

## ライセンス

soupは次のいずれかに基づいてライセンスされています。

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT.md) or <https://opensource.org/licenses/MIT>)

どちらのライセンスを適用するかはあなたが決めることができます。

明示的に別段の定めがない限り、Apache-2.0ライセンスで定義されているように、あなたが意図的にsoupに含めるために提出した貢献は、追加の条件なしに、上記のとおり二重にライセンスされるものとします。
