# 世界定義 <!-- omit in toc -->

## 概要 <!-- omit in toc -->

人工生命の世界(空間・時間・食料)を定義する。
エージェント・食料の型は [model_definition.md](model_definition.md) を参照。

## 目次 <!-- omit in toc -->

- [1. 空間](#1-空間)
- [2. 時間](#2-時間)
- [3. 食料](#3-食料)
- [4. パラメータ](#4-パラメータ)

## 1. 空間

- 世界は連続2次元空間 `[0, WORLD_WIDTH] × [0, WORLD_HEIGHT]` とする。
- 座標は `(f32, f32)`。
- 世界の端は壁として扱う(ラップしない)。壁を越える移動は境界へクランプされる(詳細は [agent_move_definition.md](agents/agent_move_definition.md))。
- エージェント同士の衝突判定は行わない(重なりを許す)。

## 2. 時間

- シミュレーションは離散時間(tick)で進行する。
- 1 tick で全エージェントが1回ずつ行動する(処理順は [simulation_definition.md](simulation_definition.md) を参照)。

## 3. 食料

- 食料の型は [model_definition.md](model_definition.md) の「食料」を参照。
- **初期配置**: シミュレーション開始時に `INITIAL_FOOD` 個を一様乱数の位置に配置する。
- **スポーン**: 毎 tick、世界内の食料総数が `FOOD_CAP` 未満であれば、`FOOD_SPAWN_PER_TICK` 個を上限として一様乱数の位置に生成する。
- 各食料のエネルギー回復量は `FOOD_ENERGY` の固定値とする(個体差は将来バージョンで検討)。
- 食料はエージェントに摂取されると世界から除去される(詳細は [agent_eat_definition.md](agents/agent_eat_definition.md))。

## 4. パラメータ

| パラメータ | デフォルト | 説明 |
| --- | --- | --- |
| `WORLD_WIDTH` | 1000.0 | 世界の幅 |
| `WORLD_HEIGHT` | 800.0 | 世界の高さ |
| `INITIAL_FOOD` | 1600 | 開始時の食料数 |
| `FOOD_SPAWN_PER_TICK` | 50 | 1 tick あたりの食料スポーン上限 |
| `FOOD_CAP` | 4000 | 世界に同時に存在できる食料の上限 |
| `FOOD_ENERGY` | 30.0 | 食料1個のエネルギー回復量 |
