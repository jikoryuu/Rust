# ■Rustで麻雀ゲーム作成設計書
## ◇基本設定
- 盤面のサイズは 37x27

![](MahjangImageMap.png)

- リアル麻雀の完全再現(40) プレイヤー名表示、洗牌、山積み、場決め、王牌を切る、ドラ表示、配牌、(理牌)、自摸、九種九牌判定、シーサンプトウ判定、(捨て牌選択表示)、自摸番の表示、残牌数表示、川に捨てる、次の人が自摸る、チー・ポン・カン・リーチの可能表示、(喰いタンなど役の判定処理)、(翻縛りの判定)、フリテンの表示、チー・ポン・カン・リーチ牌/棒表示、リーチ棒を出す、スーカンナガレ判定、嶺上牌をツモる、ヤクマンの可否表示、スーチャリーチ判定、スーフーレンダ判定、ナガシマンガン判定、上がり、連荘の場合積棒を出す、役名と点数の移動表示(別窓)、飛びの判定、(終局表示)、局と場の情報の更新、場の回転(牌の積順の関係で)、\[プレイヤー名表示に戻る\]

- 観戦モードの実装 場を回転できるようにする、全牌表示牌にする
- 譜牌の記録と再現
- 一人対戦(全プレイヤー操作)モードの実装(場の回転有り無し選択可能)
- AIまたは機械対応の実装
- (通信対戦の実装)

### 〇麻雀ルール
- 四人麻雀か三人麻雀か
- 一局戦、東風戦、半荘線、CPU戦
- ==以下詳細設定==
- 季節牌または及び花牌(抜きドラ\[中国ルール\])の可否
- 配給原点 25000スタート、27000スタート、30000スタート\[3人麻雀 35000-200000\]
- 1位必要点数 30000-200000 \[3人麻雀 40000-200000\]
- 自摸損(3人麻雀のみ アリ)
- 赤ドラの可否 赤ナシ、赤ドラ3、赤ドラ4、(3人麻雀 赤ドラ2)
- 飛びの可否(アリ)
- ローカル役の可否
- 喰いタンの可否(アリ)
- 翻縛り 一翻縛り、二翻縛り、四翻縛り

### 〇麻雀役(43)
- 一翻(16)　リーチ(メンゼンのみ)、タンヤオ、ツモ(メンゼンのみ)、自風牌、場風牌、三元牌、ピンフ(メンゼンのみ)、一盃口(メンゼンのみ)、チャンカン、リンシャン、ハイテイ、ホウテイ、イッパツ(リーチ時のみ)、ドラ、アカドラ、ヌキドラ
- 二翻(11)　ダブルリーチ(メンゼンのみ)、サンショクドーコー、サンカンツ、トイトイ、サンアンコー、ショウサンゲン、ホンロートー、チートイ(メンゼンのみ)、チャンタ(食い下がり一翻)、イッツー(食い下がり一翻)、サンショクドウジュン(食い下がり一翻)
- 三翻(3)　リャンペーコー(メンゼンのみ)、ジュンチャン(食い下がり一翻)、ホンイツ(食い下がり一翻)
- 六翻(1)　チンイツ(食い下がり一翻)
- 満貫(1)　ナガシマンガン
- 役満(11)　テンホー(親のみ)、チーホー(子のみ)、ダイサンゲン、スーアンコウ、ツーイーソー、リューイーソー、チンロートー、コクシムソウ(メンゼンのみ)、ショースーシー、スーカンツ、チューレンポートー(メンゼンのみ)
- ダブル役満(4)　スーアンコウタンキ(メンゼンのみ)、コクシムソウジュウサンメンマチ(メンゼンのみ)、チューレンキュウメンマチ(メンゼンのみ)、ダイスーシー
- 流局　
### 〇ローカル役(14)
- 一翻(3)　ツバメガエシ、カンブリ、シーアルラオタイ
- 二翻(2)　ウーメンチー、サンレンコー
- 三翻(1)　イッショクサンジュン
- 満貫(2)　イーピンモーユエ、チューピンラオユイ
- 役満(5)　レンホウ、ダイシャリン、ダイチクリン、ダイスウリン、イシノウエニモサンネン
- ダブル役満(1)　ダイチーシン(メンゼンのみ)

# ■