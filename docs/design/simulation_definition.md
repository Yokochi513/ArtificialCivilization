# シミュレーション定義 <!-- omit in toc -->

## 概要 <!-- omit in toc -->

シミュレーション全体の進行(初期化・tick ループ・終了条件)と観測(統計出力)を定義する。
各処理の詳細は以下を参照。

- 世界・食料: [world_definition.md](world_definition.md)
- 型定義: [model_definition.md](model_definition.md)
- 移動: [agents/agent_move_definition.md](agents/agent_move_definition.md)
- 食事: [agents/agent_eat_definition.md](agents/agent_eat_definition.md)
- 死亡: [agents/agent_die_definition.md](agents/agent_die_definition.md)
- 繁殖: [agents/agent_reproduction_definition.md](agents/agent_reproduction_definition.md)

## 目次 <!-- omit in toc -->

- [1. 初期化](#1-初期化)
- [2. tick ループ](#2-tick-ループ)
- [3. 乱数](#3-乱数)
- [4. 終了条件](#4-終了条件)
- [5. 統計出力](#5-統計出力)
- [6. バランス調整の目標](#6-バランス調整の目標)
- [7. モジュール構成](#7-モジュール構成)
- [8. パラメータ](#8-パラメータ)

## 1. 初期化

1. 世界を生成し、食料を `INITIAL_FOOD` 個配置する
2. 初期エージェントを `INITIAL_AGENTS` 体生成する
   - 位置・向きは一様乱数
   - `energy = INITIAL_ENERGY`、`generation = 0`、`age = 0`
   - ゲノムは各属性のデフォルト値(突然変異による多様化に任せるため、初期個体は均一とする)

## 2. tick ループ

```
各 tick:
  1. 食料スポーン処理
  2. エージェントの行動順をシャッフル
  3. 各エージェントについて順に:
     a. 基礎代謝      energy -= BASE_COST
     b. 移動          (知覚 → 意思決定 → 回頭 → 前進、移動コスト消費)
     c. 食事          (arm_length 内の最近食料を摂取)
     d. 繁殖          (閾値を満たせば子を生成。子は次 tick から行動)
     e. 死亡判定      (energy <= 0 または age >= max_age で死亡フラグ)
     f. age += 1
  4. 死亡個体の除去・新生個体の追加
  5. 統計の記録・出力
```

- 行動順を毎 tick シャッフルするのは、食料の競合(早い者勝ち)で特定個体が恒常的に有利になるのを防ぐため。

## 3. 乱数

- 再現性確保のため、乱数シードは `Config` で指定可能とする。
- 未指定時はエントロピーからシードを生成し、その値をログに出力する(後から同じ実行を再現できるようにする)。

## 4. 終了条件

以下のいずれかで終了する。

- tick 数が `MAX_TICKS` に到達
- 全エージェントが死亡(絶滅)

## 5. 統計出力

v1.0.0 は CLI 出力のみとする(可視化 UI は将来検討)。

- **定期統計**: `STATS_INTERVAL` tick ごとに1行出力する。
  - tick、個体数、平均エネルギー、平均年齢、最大世代、食料数、累計出生数、累計死亡数(餓死/寿命の内訳)
  - ゲノムの平均値(`move_speed` / `turn_speed` / `vision_range` / `arm_length`)— 淘汰の進行を観測するため
- **終了時サマリ**: 終了理由(tick 上限 or 絶滅)と最終統計を出力する。

出力例:

```
tick=  1000 pop=  62 avg_energy= 98.3 avg_age= 214 max_gen= 4 food= 350 births= 45 deaths= 33 (starve=30, old=3) genome[spd=1.02 turn=0.49 vis=5.3 arm=0.52]
```

## 6. バランス調整の目標

デフォルトパラメータの目標は「絶滅も爆発もせず、個体数が食料供給量に応じて振動しながら継続する」こと。

- `MAX_TICKS` 完走時に個体数が 10〜500 の範囲で推移している
- 世代交代が発生している(最大世代が増加していく)
- 食料スポーン率を上げ下げすると個体数がそれに追従する
- ゲノム平均値が初期値から変化していく(淘汰が機能している)

各設計書のパラメータは初期値の提案であり、実行結果を見て調整する。

## 7. モジュール構成

```bash
src/
├── main.rs        # エントリポイント。設定読み込み・シミュレーション起動
├── config.rs      # Config: 全パラメータ
├── models
│   ├─ agent.rs    # Agent(model_definition.md に対応)
│   ├─ food.rs     # Food(model_definition.md に対応)
│   └─ genome.rs   # Genome(model_definition.md に対応)
├── world.rs       # World: 空間・食料管理(world_definition.md に対応)
├── agent/
│   ├── mod.rs
│   ├── movement.rs      # 移動(agent_move_definition.md に対応)
│   ├── eat.rs           # 食事(agent_eat_definition.md に対応)
│   ├── die.rs           # 死亡(agent_die_definition.md に対応)
│   └── reproduction.rs  # 繁殖(agent_reproduction_definition.md に対応)
├── simulation.rs  # tick ループ(本書に対応)
└── stats.rs       # 統計収集・出力
```

依存クレート:

| クレート | 用途 |
| --- | --- |
| `rand` | 位置・向き・行動順シャッフル・突然変異の乱数 |
| `rand_distr` | 突然変異の正規分布乱数 |

## 8. パラメータ

| パラメータ | デフォルト | 説明 |
| --- | --- | --- |
| `INITIAL_AGENTS` | 50 | 初期エージェント数 |
| `INITIAL_ENERGY` | 100.0 | 初期個体の開始エネルギー |
| `MAX_ENERGY` | 200.0 | `max_energy` の初期値 |
| `BASE_COST` | 0.5 | 毎 tick の基礎代謝 |
| `MAX_TICKS` | 10000 | シミュレーションの最大 tick 数 |
| `STATS_INTERVAL` | 100 | 統計出力の間隔(tick) |
| `SEED` | なし | 乱数シード(未指定時は自動生成しログ出力) |
